use std::collections::{hash_map::DefaultHasher, HashMap, HashSet};
use std::env;
use std::fs;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStderr, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use harness_client::{HarnessClientConfig, HarnessServerClient};
use harness_events::{EventPayload, RuntimeEvent as HarnessRuntimeEvent, TaskProgress};
use harness_policy::PermissionMode;
use harness_providers::ProviderConfig;
use harness_storage::{JsonSessionStore, SessionRecord};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager, State};
use uuid::Uuid;

const CLAW_RUNTIME_EVENT: &str = "claw-runtime";
const BROWSER_RUNTIME_EVENT: &str = "browser-runtime";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct DesktopSettings {
    backend_path: String,
    recent_workspaces: Vec<String>,
    selected_provider_id: String,
    selected_model: String,
    selected_permission: String,
}

impl Default for DesktopSettings {
    fn default() -> Self {
        Self {
            backend_path: default_backend_path_value(),
            recent_workspaces: vec![],
            selected_provider_id: "local".to_string(),
            selected_model: "gemma4:e2b".to_string(),
            selected_permission: "workspace-write".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct BackendHealth {
    requested_path: String,
    resolved_path: Option<String>,
    exists: bool,
    runnable: bool,
    version: Option<String>,
    status: String,
    message: String,
    sessions_directory: Option<String>,
    session_count: usize,
    local_runtime: Option<LocalRuntimeHealth>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct LocalRuntimeHealth {
    reachable: bool,
    version: Option<String>,
    has_selected_model: bool,
    available_models: Vec<String>,
    message: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ClawSessionSummary {
    id: String,
    path: String,
    modified_at: u64,
    modified_label: String,
    message_count: usize,
    preview: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ClawUsage {
    input_tokens: u64,
    output_tokens: u64,
    cache_creation_input_tokens: u64,
    cache_read_input_tokens: u64,
    total_tokens: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ClawTranscriptMessage {
    id: String,
    role: String,
    title: String,
    body: String,
    meta: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ClawActivityItem {
    id: String,
    label: String,
    status: String,
    summary: String,
    timestamp: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ClawFileChange {
    path: String,
    summary: String,
    additions: i64,
    deletions: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ClawSessionSnapshot {
    id: String,
    path: String,
    workspace_path: String,
    modified_at: u64,
    modified_label: String,
    message_count: usize,
    preview: String,
    transcript: Vec<ClawTranscriptMessage>,
    activity: Vec<ClawActivityItem>,
    changes: Vec<ClawFileChange>,
    latest_usage: Option<ClawUsage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LaunchRuntimeRequest {
    backend_path: String,
    workspace_path: String,
    provider_id: String,
    model: String,
    permission_mode: String,
    resume_session_path: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct RuntimeLaunch {
    runtime_id: String,
    workspace_path: String,
    provider_id: String,
    model: String,
    permission_mode: String,
    session_id: Option<String>,
    session_path: Option<String>,
    resumed: bool,
    message: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
enum RuntimeEventPayload {
    Started {
        launch: RuntimeLaunch,
    },
    Output {
        runtime_id: String,
        line: String,
        stream: String,
        timestamp: String,
    },
    SessionAttached {
        runtime_id: String,
        session_id: String,
        session_path: String,
        resumed: bool,
    },
    Snapshot {
        runtime_id: String,
        snapshot: ClawSessionSnapshot,
    },
    TurnStarted {
        runtime_id: String,
        input_preview: String,
    },
    TurnFinished {
        runtime_id: String,
        success: bool,
        duration_ms: u128,
    },
    Stopped {
        runtime_id: String,
        code: Option<i32>,
        message: String,
    },
    Error {
        runtime_id: String,
        message: String,
    },
}

struct RuntimeHandle {
    client: Arc<Mutex<HarnessServerClient>>,
    session_id: Uuid,
    session_path: PathBuf,
    workspace_path: PathBuf,
    turn_in_flight: Arc<Mutex<bool>>,
}

#[derive(Clone, Default)]
struct RuntimeManager {
    runtimes: Arc<Mutex<HashMap<String, RuntimeHandle>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BrowserLaunchRequest {
    browser_path: Option<String>,
    session: Option<String>,
    headless: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct BrowserRuntimeLaunch {
    runtime_id: String,
    session: String,
    browser_path: String,
    headless: bool,
    message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BrowserCommandRequest {
    runtime_id: String,
    command: Vec<String>,
    session: Option<String>,
    request_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BrowserCommandResult {
    id: Option<String>,
    session: String,
    success: bool,
    action: Option<String>,
    data: Option<serde_json::Value>,
    error: Option<String>,
    warning: Option<String>,
    error_type: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
enum BrowserRuntimeEventPayload {
    Started {
        launch: BrowserRuntimeLaunch,
    },
    Output {
        runtime_id: String,
        line: String,
        stream: String,
    },
    Response {
        runtime_id: String,
        response: BrowserCommandResult,
    },
    Stopped {
        runtime_id: String,
        code: Option<i32>,
        message: String,
    },
    Error {
        runtime_id: String,
        message: String,
    },
}

struct BrowserRuntimeProcess {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    session: String,
    browser_path: PathBuf,
}

#[derive(Clone, Default)]
struct BrowserRuntimeManager {
    runtimes: Arc<Mutex<HashMap<String, Arc<Mutex<BrowserRuntimeProcess>>>>>,
}

#[derive(Debug, Deserialize)]
struct OllamaVersionResponse {
    version: String,
}

#[derive(Debug, Deserialize)]
struct OllamaTagsResponse {
    #[serde(default)]
    models: Vec<OllamaModelTag>,
}

#[derive(Debug, Deserialize)]
struct OllamaModelTag {
    name: String,
}

fn normalize_provider_id(value: &str) -> &str {
    match value {
        "local" | "anthropic" | "grok" => value,
        _ => "local",
    }
}

fn default_model_for_provider(provider_id: &str) -> &str {
    match provider_id {
        "anthropic" => "claude-sonnet-4-6",
        "grok" => "grok-4.20-0309-reasoning",
        _ => "gemma4:e2b",
    }
}

fn provider_supports_model(provider_id: &str, model: &str) -> bool {
    match provider_id {
        "anthropic" => model == "claude-sonnet-4-6",
        "grok" => {
            model == "grok-4.20-0309-reasoning"
                || model == "grok-4-1-fast-reasoning"
                || model == "grok-code-fast-1"
        }
        _ => matches!(model, "gemma4:e2b" | "gemma4:e4b"),
    }
}

fn normalize_permission(value: &str) -> &str {
    match value {
        "read-only" | "workspace-write" | "danger-full-access" => value,
        _ => "workspace-write",
    }
}

fn normalize_workspace_path(path: String) -> String {
    if !path.contains("/Users/name/") {
        return path;
    }

    let Some(home) = env::var_os("HOME") else {
        return path;
    };

    let home = PathBuf::from(home);
    let remainder = path.trim_start_matches("/Users/name/");
    home.join(remainder).display().to_string()
}

fn normalize_desktop_settings(settings: DesktopSettings) -> DesktopSettings {
    let provider_id = normalize_provider_id(&settings.selected_provider_id).to_string();
    let selected_model = if provider_supports_model(&provider_id, &settings.selected_model) {
        settings.selected_model
    } else {
        default_model_for_provider(&provider_id).to_string()
    };

    let backend_path = settings.backend_path.trim();
    let backend_path = if backend_path.is_empty() || is_legacy_claw_backend_path(backend_path) {
        default_backend_path_value()
    } else {
        settings.backend_path
    };

    DesktopSettings {
        backend_path,
        recent_workspaces: settings
            .recent_workspaces
            .into_iter()
            .map(normalize_workspace_path)
            .collect(),
        selected_provider_id: provider_id,
        selected_model,
        selected_permission: normalize_permission(&settings.selected_permission).to_string(),
    }
}

fn is_legacy_claw_backend_path(value: &str) -> bool {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return false;
    }

    if trimmed == "claw" {
        return true;
    }

    Path::new(trimmed)
        .file_name()
        .and_then(|segment| segment.to_str())
        .is_some_and(|segment| segment == "claw")
}

fn settings_path(app: &AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?;
    fs::create_dir_all(&app_data_dir).map_err(|error| error.to_string())?;
    Ok(app_data_dir.join("settings.json"))
}

fn workspace_session_root(app: &AppHandle, workspace_path: &Path) -> Result<PathBuf, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?;
    let name = workspace_path
        .file_name()
        .and_then(|segment| segment.to_str())
        .unwrap_or("workspace");
    let slug = sanitize_segment(name);
    let hash = hash_path(workspace_path);
    let root = app_data_dir
        .join("sessions")
        .join(format!("{slug}-{hash:016x}"));
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    Ok(root)
}

fn sanitize_segment(value: &str) -> String {
    let sanitized = value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>();

    sanitized
        .trim_matches('-')
        .split('-')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

fn hash_path(path: &Path) -> u64 {
    let mut hasher = DefaultHasher::new();
    path.hash(&mut hasher);
    hasher.finish()
}

fn emit_runtime_event(app: &AppHandle, payload: RuntimeEventPayload) {
    let _ = app.emit(CLAW_RUNTIME_EVENT, payload);
}

fn emit_browser_runtime_event(app: &AppHandle, payload: BrowserRuntimeEventPayload) {
    let _ = app.emit(BROWSER_RUNTIME_EVENT, payload);
}

fn format_relative_timestamp(epoch_secs: u64) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(epoch_secs);
    let elapsed = now.saturating_sub(epoch_secs);
    match elapsed {
        0..=59 => format!("{elapsed}s ago"),
        60..=3_599 => format!("{}m ago", elapsed / 60),
        3_600..=86_399 => format!("{}h ago", elapsed / 3_600),
        _ => format!("{}d ago", elapsed / 86_400),
    }
}

fn format_relative_timestamp_ms(timestamp_ms: u64) -> String {
    format_relative_timestamp(timestamp_ms / 1_000)
}

fn truncate_text(value: &str, limit: usize) -> String {
    if value.chars().count() <= limit {
        return value.to_string();
    }

    let truncated = value.chars().take(limit).collect::<String>();
    format!("{truncated}…")
}

fn default_backend_path_value() -> String {
    resolve_common_harness_paths()
        .unwrap_or_else(|| PathBuf::from("harness-server"))
        .display()
        .to_string()
}

fn default_browser_session_value() -> String {
    format!("poro-{}", Uuid::new_v4().simple())
}

#[cfg(windows)]
fn agent_browser_binary_name() -> &'static str {
    "agent-browser.exe"
}

#[cfg(not(windows))]
fn agent_browser_binary_name() -> &'static str {
    "agent-browser"
}

fn agent_browser_target_profiles() -> [&'static str; 2] {
    if cfg!(debug_assertions) {
        ["debug", "release"]
    } else {
        ["release", "debug"]
    }
}

fn tauri_manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn tauri_manifest_path() -> PathBuf {
    tauri_manifest_dir().join("Cargo.toml")
}

fn resolve_common_agent_browser_paths() -> Vec<PathBuf> {
    let root = tauri_manifest_dir();
    agent_browser_target_profiles()
        .into_iter()
        .map(|profile| {
            root.join("target")
                .join(profile)
                .join(agent_browser_binary_name())
        })
        .collect()
}

fn resolve_agent_browser_path(requested_path: Option<&str>) -> Option<PathBuf> {
    if let Some(value) = requested_path {
        let trimmed = value.trim();
        if !trimmed.is_empty() {
            let explicit = PathBuf::from(trimmed);
            if explicit.components().count() > 1 || explicit.is_absolute() {
                return Some(explicit);
            }

            return resolve_executable_on_path(trimmed).or(Some(explicit));
        }
    }

    if let Ok(value) = env::var("PORO_AGENT_BROWSER_BIN") {
        let trimmed = value.trim();
        if !trimmed.is_empty() {
            return Some(PathBuf::from(trimmed));
        }
    }

    resolve_common_agent_browser_paths()
        .into_iter()
        .find(|candidate| candidate.exists())
        .or_else(|| resolve_executable_on_path("agent-browser"))
}

fn build_vendored_agent_browser() -> Result<PathBuf, String> {
    let manifest_path = tauri_manifest_path();
    let mut command = Command::new("cargo");
    command
        .arg("build")
        .arg("--manifest-path")
        .arg(&manifest_path)
        .arg("-p")
        .arg("agent-browser")
        .current_dir(tauri_manifest_dir());

    if !cfg!(debug_assertions) {
        command.arg("--release");
    }

    let status = command
        .status()
        .map_err(|error| format!("Failed to build vendored agent-browser: {error}"))?;
    if !status.success() {
        return Err(
            "Cargo could not build the vendored `agent-browser` binary for the desktop runtime."
                .to_string(),
        );
    }

    resolve_common_agent_browser_paths()
        .into_iter()
        .find(|candidate| candidate.exists() && is_executable(candidate))
        .ok_or_else(|| {
            "The vendored `agent-browser` binary finished building, but Poro could not find the executable."
                .to_string()
        })
}

fn ensure_agent_browser_path(requested_path: Option<&str>) -> Result<PathBuf, String> {
    if let Some(candidate) = resolve_agent_browser_path(requested_path) {
        if candidate.exists() && is_executable(&candidate) {
            return Ok(candidate);
        }

        if requested_path
            .map(|value| !value.trim().is_empty())
            .unwrap_or(false)
            || env::var("PORO_AGENT_BROWSER_BIN")
                .map(|value| !value.trim().is_empty())
                .unwrap_or(false)
        {
            return Err(format!(
                "Poro found an `agent-browser` path at `{}`, but it is missing or not executable.",
                candidate.display()
            ));
        }
    }

    build_vendored_agent_browser()
}

fn resolve_backend_path(requested_path: &str) -> Option<PathBuf> {
    let trimmed = requested_path.trim();
    if trimmed.is_empty() || trimmed == "harness-server" || is_legacy_claw_backend_path(trimmed) {
        return resolve_executable_on_path("harness-server").or_else(resolve_common_harness_paths);
    }

    let candidate = PathBuf::from(trimmed);
    if trimmed.contains(std::path::MAIN_SEPARATOR) {
        if candidate.file_name().and_then(|value| value.to_str()) == Some("harness-server") {
            return resolve_executable_on_path("harness-server")
                .or_else(resolve_common_harness_paths);
        }

        if candidate.exists() {
            return Some(candidate);
        }

        return None;
    }

    resolve_executable_on_path(trimmed)
}

fn resolve_executable_on_path(name: &str) -> Option<PathBuf> {
    let paths = env::var_os("PATH")?;
    env::split_paths(&paths)
        .map(|dir| dir.join(name))
        .find(|candidate| candidate.exists())
}

fn resolve_common_harness_paths() -> Option<PathBuf> {
    let home = env::var_os("HOME").map(PathBuf::from)?;
    [
        home.join("Repos/harness/target/debug/harness-server"),
        home.join("Repos/harness/target/release/harness-server"),
    ]
    .into_iter()
    .find(|candidate| candidate.exists())
}

#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;

    fs::metadata(path)
        .map(|metadata| metadata.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(not(unix))]
fn is_executable(path: &Path) -> bool {
    path.exists()
}

fn check_local_runtime_health(selected_model: Option<&str>) -> LocalRuntimeHealth {
    let client = match Client::builder().timeout(Duration::from_secs(4)).build() {
        Ok(client) => client,
        Err(error) => {
            return LocalRuntimeHealth {
                reachable: false,
                version: None,
                has_selected_model: false,
                available_models: vec![],
                message: format!("Could not create the Ollama health client: {error}"),
            };
        }
    };

    let version_response = client
        .get("http://127.0.0.1:11434/api/version")
        .send()
        .and_then(|response| response.error_for_status())
        .and_then(|response| response.json::<OllamaVersionResponse>());
    let tags_response = client
        .get("http://127.0.0.1:11434/api/tags")
        .send()
        .and_then(|response| response.error_for_status())
        .and_then(|response| response.json::<OllamaTagsResponse>());

    match (version_response, tags_response) {
        (Ok(version), Ok(tags)) => {
            let mut available_models = tags
                .models
                .into_iter()
                .map(|model| model.name)
                .collect::<Vec<_>>();
            available_models.sort();
            let has_selected_model = selected_model
                .map(|model| available_models.iter().any(|candidate| candidate == model))
                .unwrap_or(true);

            LocalRuntimeHealth {
                reachable: true,
                version: Some(version.version),
                has_selected_model,
                available_models,
                message: if has_selected_model {
                    "Ollama is running and the selected model is installed.".to_string()
                } else {
                    match selected_model {
                        Some(model) => {
                            format!("Ollama is reachable, but `{model}` is not installed yet.")
                        }
                        None => "Ollama is reachable.".to_string(),
                    }
                },
            }
        }
        _ => LocalRuntimeHealth {
            reachable: false,
            version: None,
            has_selected_model: false,
            available_models: vec![],
            message: "Ollama is not responding on http://127.0.0.1:11434.".to_string(),
        },
    }
}

fn provider_message_for_health(provider_id: &str, selected_model: Option<&str>) -> Option<String> {
    match provider_id {
        "grok" => Some(match env::var("XAI_API_KEY") {
            Ok(value) if !value.trim().is_empty() => format!(
                "Harness is ready for xAI / Grok. Launching will use `{}` through the hosted xAI API with the `XAI_API_KEY` already present in this app process.",
                selected_model.unwrap_or("grok-4.20-0309-reasoning")
            ),
            _ => "Harness is wired for xAI / Grok, but `XAI_API_KEY` is not set in this app process yet. Launch Poro from a terminal that exports it before starting a Grok session.".to_string(),
        }),
        "anthropic" => Some(
            "The sibling Harness is wired into Poro now, but the Anthropic provider path is not enabled in this first integration pass yet. Switch to Ollama Local for now.".to_string(),
        ),
        _ => None,
    }
}

fn provider_config_for_runtime(provider_id: &str, model: &str) -> Result<ProviderConfig, String> {
    match provider_id {
        "local" => Ok(ProviderConfig::local_default().with_model(model.to_string())),
        "grok" => {
            let api_key = env::var("XAI_API_KEY").map_err(|_| {
                "xAI / Grok requires `XAI_API_KEY` in the environment. Launch Poro from a terminal that exports it first.".to_string()
            })?;
            if api_key.trim().is_empty() {
                return Err(
                    "xAI / Grok requires `XAI_API_KEY` in the environment. The current value is empty."
                        .to_string(),
                );
            }

            Ok(ProviderConfig::xai_default().with_model(model.to_string()))
        }
        "anthropic" => Err(
            "Anthropic is not enabled in the sibling Harness bridge yet. Switch to Ollama Local for this pass."
                .to_string(),
        ),
        other => Err(format!("Unsupported provider `{other}`.")),
    }
}

fn permission_mode_from_str(value: &str) -> Result<PermissionMode, String> {
    match value {
        "read-only" => Ok(PermissionMode::ReadOnly),
        "workspace-write" => Ok(PermissionMode::WorkspaceWrite),
        "danger-full-access" => Ok(PermissionMode::DangerFullAccess),
        other => Err(format!("Unsupported permission mode `{other}`.")),
    }
}

fn session_store_for_workspace(
    app: &AppHandle,
    workspace_path: &Path,
) -> Result<JsonSessionStore, String> {
    Ok(JsonSessionStore::new(workspace_session_root(
        app,
        workspace_path,
    )?))
}

fn session_path_for_root(session_root: &Path, session_id: Uuid) -> PathBuf {
    session_root.join(format!("{session_id}.json"))
}

fn session_id_from_path(path: &Path) -> Option<Uuid> {
    path.file_stem()
        .and_then(|value| value.to_str())
        .and_then(|value| Uuid::parse_str(value).ok())
}

fn load_session_record(path: &Path) -> Result<SessionRecord, String> {
    let contents = fs::read_to_string(&path).map_err(|error| error.to_string())?;
    serde_json::from_str(&contents).map_err(|error| error.to_string())
}

fn list_workspace_session_records(
    app: &AppHandle,
    workspace_path: &Path,
) -> Result<Vec<(PathBuf, SessionRecord)>, String> {
    let store = session_store_for_workspace(app, workspace_path)?;
    let sessions = store.list_sessions().map_err(|error| error.to_string())?;
    sessions
        .into_iter()
        .map(|metadata| {
            let path = session_path_for_root(store.root(), metadata.session_id);
            let record = load_session_record(&path)?;
            Ok((path, record))
        })
        .collect()
}

fn summarize_progress(progress: &TaskProgress) -> String {
    if !progress.remaining_files.is_empty() {
        format!("Remaining files: {}", progress.remaining_files.join(", "))
    } else if let Some(reason) = &progress.blocked_reason {
        format!("Blocked: {reason}")
    } else {
        format!("Verification: {:?}", progress.verification)
    }
}

fn format_critique_reason(reason: &str) -> String {
    reason
        .split('_')
        .filter(|segment| !segment.trim().is_empty())
        .map(|segment| {
            let mut chars = segment.chars();
            match chars.next() {
                Some(first) => format!(
                    "{}{}",
                    first.to_ascii_uppercase(),
                    chars.as_str().to_ascii_lowercase()
                ),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn derive_session_preview(record: &SessionRecord) -> String {
    for event in record.events.iter().rev() {
        match &event.payload {
            EventPayload::MessageAssistant { message } => {
                return truncate_text(message, 120);
            }
            EventPayload::SessionCompleted { summary } => {
                return truncate_text(summary, 120);
            }
            EventPayload::MessageUser { message } => {
                return truncate_text(message, 120);
            }
            EventPayload::ProgressUpdated { progress } => {
                if !progress.goal.trim().is_empty() {
                    return truncate_text(&progress.goal, 120);
                }
            }
            _ => {}
        }
    }

    format!("{} session", record.metadata.model)
}

fn transcript_from_record(record: &SessionRecord) -> Vec<ClawTranscriptMessage> {
    let mut transcript = Vec::new();
    let mut last_assistant_body: Option<String> = None;

    for event in &record.events {
        let id = format!("{}-{}", record.metadata.session_id, event.sequence);
        match &event.payload {
            EventPayload::SessionStarted {
                cwd,
                permission_mode,
                provider,
                model,
            } => transcript.push(ClawTranscriptMessage {
                id,
                role: "system".to_string(),
                title: "Session started".to_string(),
                body: format!("Workspace: {cwd}"),
                meta: format!("{provider} • {model} • {permission_mode}"),
            }),
            EventPayload::MessageUser { message } => transcript.push(ClawTranscriptMessage {
                id,
                role: "user".to_string(),
                title: "You".to_string(),
                body: message.clone(),
                meta: "Prompt".to_string(),
            }),
            EventPayload::MessageAssistant { message } => transcript.push(ClawTranscriptMessage {
                id,
                role: "assistant".to_string(),
                title: "Harness".to_string(),
                body: {
                    last_assistant_body = Some(message.clone());
                    message.clone()
                },
                meta: "Assistant".to_string(),
            }),
            EventPayload::ToolStarted {
                tool,
                summary,
                arguments_json,
            } => transcript.push(ClawTranscriptMessage {
                id,
                role: "tool".to_string(),
                title: format!("Tool `{tool}`"),
                body: if summary.trim().is_empty() {
                    format!("Started with {arguments_json}")
                } else {
                    summary.clone()
                },
                meta: "Tool started".to_string(),
            }),
            EventPayload::ToolFinished {
                tool,
                summary,
                output,
                success,
                ..
            } => transcript.push(ClawTranscriptMessage {
                id,
                role: "tool".to_string(),
                title: format!("Tool result `{tool}`"),
                body: if output.trim().is_empty() {
                    summary.clone()
                } else {
                    truncate_text(output, 1_600)
                },
                meta: if *success {
                    "Tool finished".to_string()
                } else {
                    "Tool finished with issues".to_string()
                },
            }),
            EventPayload::ToolFailed { tool, error, .. } => {
                transcript.push(ClawTranscriptMessage {
                    id,
                    role: "tool".to_string(),
                    title: format!("Tool failed `{tool}`"),
                    body: error.clone(),
                    meta: "Tool failed".to_string(),
                })
            }
            EventPayload::ApprovalRequested { tool, reason, .. } => {
                transcript.push(ClawTranscriptMessage {
                    id,
                    role: "system".to_string(),
                    title: "Approval requested".to_string(),
                    body: reason.clone(),
                    meta: tool.clone(),
                })
            }
            EventPayload::ApprovalResolved { tool, approved, .. } => {
                transcript.push(ClawTranscriptMessage {
                    id,
                    role: "system".to_string(),
                    title: "Approval resolved".to_string(),
                    body: if *approved {
                        "The requested action was approved.".to_string()
                    } else {
                        "The requested action was rejected.".to_string()
                    },
                    meta: tool.clone(),
                })
            }
            EventPayload::PatchGenerated { files } => transcript.push(ClawTranscriptMessage {
                id,
                role: "system".to_string(),
                title: "Patch generated".to_string(),
                body: files.join(", "),
                meta: "Review".to_string(),
            }),
            EventPayload::VerificationStarted { command } => {
                transcript.push(ClawTranscriptMessage {
                    id,
                    role: "system".to_string(),
                    title: "Verification started".to_string(),
                    body: command.clone(),
                    meta: "Verification".to_string(),
                })
            }
            EventPayload::VerificationFinished {
                command, success, ..
            } => transcript.push(ClawTranscriptMessage {
                id,
                role: "system".to_string(),
                title: if *success {
                    "Verification passed".to_string()
                } else {
                    "Verification failed".to_string()
                },
                body: command.clone(),
                meta: "Verification".to_string(),
            }),
            EventPayload::CritiqueStarted { .. } => {}
            EventPayload::CritiqueFinished { .. } => {}
            EventPayload::ProgressUpdated { .. } => {}
            EventPayload::SessionCompleted { summary } => {
                if last_assistant_body.as_deref() != Some(summary.as_str()) {
                    transcript.push(ClawTranscriptMessage {
                        id,
                        role: "assistant".to_string(),
                        title: "Completed".to_string(),
                        body: summary.clone(),
                        meta: "Done".to_string(),
                    });
                }
            }
            EventPayload::SessionFailed { error } => transcript.push(ClawTranscriptMessage {
                id,
                role: "system".to_string(),
                title: "Session failed".to_string(),
                body: error.clone(),
                meta: "Runtime".to_string(),
            }),
        }
    }

    transcript
}

fn activity_from_record(record: &SessionRecord) -> Vec<ClawActivityItem> {
    let mut activity = Vec::new();

    for event in &record.events {
        let id = format!("activity-{}-{}", record.metadata.session_id, event.sequence);
        let timestamp = format_relative_timestamp_ms(event.timestamp_ms);
        let item = match &event.payload {
            EventPayload::SessionStarted {
                provider,
                model,
                permission_mode,
                ..
            } => Some(ClawActivityItem {
                id,
                label: "Session started".to_string(),
                status: "complete".to_string(),
                summary: format!("{provider} • {model} • {permission_mode}"),
                timestamp,
            }),
            EventPayload::ToolStarted { tool, summary, .. } => Some(ClawActivityItem {
                id,
                label: tool.clone(),
                status: "active".to_string(),
                summary: summary.clone(),
                timestamp,
            }),
            EventPayload::ToolFinished {
                tool,
                summary,
                success,
                ..
            } => Some(ClawActivityItem {
                id,
                label: tool.clone(),
                status: if *success {
                    "complete".to_string()
                } else {
                    "queued".to_string()
                },
                summary: summary.clone(),
                timestamp,
            }),
            EventPayload::ToolFailed { tool, error, .. } => Some(ClawActivityItem {
                id,
                label: tool.clone(),
                status: "queued".to_string(),
                summary: error.clone(),
                timestamp,
            }),
            EventPayload::ApprovalRequested { tool, reason, .. } => Some(ClawActivityItem {
                id,
                label: format!("Approval needed • {tool}"),
                status: "queued".to_string(),
                summary: reason.clone(),
                timestamp,
            }),
            EventPayload::ApprovalResolved { tool, approved, .. } => Some(ClawActivityItem {
                id,
                label: format!("Approval resolved • {tool}"),
                status: if *approved {
                    "complete".to_string()
                } else {
                    "queued".to_string()
                },
                summary: if *approved {
                    "The queued action was approved.".to_string()
                } else {
                    "The queued action was rejected.".to_string()
                },
                timestamp,
            }),
            EventPayload::PatchGenerated { files } => Some(ClawActivityItem {
                id,
                label: "Patch prepared".to_string(),
                status: "complete".to_string(),
                summary: files.join(", "),
                timestamp,
            }),
            EventPayload::VerificationStarted { command } => Some(ClawActivityItem {
                id,
                label: "Verification running".to_string(),
                status: "active".to_string(),
                summary: command.clone(),
                timestamp,
            }),
            EventPayload::VerificationFinished {
                command, success, ..
            } => Some(ClawActivityItem {
                id,
                label: if *success {
                    "Verification passed".to_string()
                } else {
                    "Verification failed".to_string()
                },
                status: if *success {
                    "complete".to_string()
                } else {
                    "queued".to_string()
                },
                summary: command.clone(),
                timestamp,
            }),
            EventPayload::CritiqueStarted { reason } => Some(ClawActivityItem {
                id,
                label: "Runtime critique".to_string(),
                status: "active".to_string(),
                summary: format!(
                    "Re-evaluating the plan after {}.",
                    format_critique_reason(reason)
                ),
                timestamp,
            }),
            EventPayload::CritiqueFinished { reason, summary } => Some(ClawActivityItem {
                id,
                label: format!("Critique • {}", format_critique_reason(reason)),
                status: "complete".to_string(),
                summary: truncate_text(summary, 220),
                timestamp,
            }),
            EventPayload::ProgressUpdated { progress } => Some(ClawActivityItem {
                id,
                label: "Task progress".to_string(),
                status: if progress.remaining_files.is_empty() {
                    "complete".to_string()
                } else {
                    "active".to_string()
                },
                summary: summarize_progress(progress),
                timestamp,
            }),
            EventPayload::SessionCompleted { summary } => Some(ClawActivityItem {
                id,
                label: "Session completed".to_string(),
                status: "complete".to_string(),
                summary: summary.clone(),
                timestamp,
            }),
            EventPayload::SessionFailed { error } => Some(ClawActivityItem {
                id,
                label: "Session failed".to_string(),
                status: "queued".to_string(),
                summary: error.clone(),
                timestamp,
            }),
            EventPayload::MessageUser { .. } | EventPayload::MessageAssistant { .. } => None,
        };

        if let Some(item) = item {
            activity.push(item);
        }
    }

    activity
}

fn changes_from_record(record: &SessionRecord) -> Vec<ClawFileChange> {
    let mut seen = HashSet::new();
    let mut ordered = Vec::new();

    for event in &record.events {
        match &event.payload {
            EventPayload::PatchGenerated { files } => {
                for path in files {
                    if seen.insert(path.clone()) {
                        ordered.push(path.clone());
                    }
                }
            }
            EventPayload::ProgressUpdated { progress } => {
                for path in &progress.completed_files {
                    if seen.insert(path.clone()) {
                        ordered.push(path.clone());
                    }
                }
            }
            _ => {}
        }
    }

    ordered
        .into_iter()
        .map(|path| ClawFileChange {
            path,
            summary: "Edited by the local Harness runtime.".to_string(),
            additions: 0,
            deletions: 0,
        })
        .collect()
}

fn session_message_count(record: &SessionRecord) -> usize {
    record
        .events
        .iter()
        .filter(|event| {
            matches!(
                event.payload,
                EventPayload::MessageUser { .. } | EventPayload::MessageAssistant { .. }
            )
        })
        .count()
}

fn summary_from_record(path: &Path, record: &SessionRecord) -> ClawSessionSummary {
    ClawSessionSummary {
        id: record.metadata.session_id.to_string(),
        path: path.display().to_string(),
        modified_at: record.metadata.updated_at_ms / 1_000,
        modified_label: format_relative_timestamp(record.metadata.updated_at_ms / 1_000),
        message_count: session_message_count(record),
        preview: derive_session_preview(record),
    }
}

fn snapshot_from_record(path: &Path, record: &SessionRecord) -> ClawSessionSnapshot {
    let preview = derive_session_preview(record);
    let transcript = transcript_from_record(record);
    ClawSessionSnapshot {
        id: record.metadata.session_id.to_string(),
        path: path.display().to_string(),
        workspace_path: record.metadata.cwd.clone(),
        modified_at: record.metadata.updated_at_ms / 1_000,
        modified_label: format_relative_timestamp(record.metadata.updated_at_ms / 1_000),
        message_count: transcript.len(),
        preview,
        transcript,
        activity: activity_from_record(record),
        changes: changes_from_record(record),
        latest_usage: None,
    }
}

fn load_snapshot(path: &Path) -> Result<ClawSessionSnapshot, String> {
    let record = load_session_record(path)?;
    Ok(snapshot_from_record(path, &record))
}

fn map_event_to_output_line(event: &HarnessRuntimeEvent) -> Option<(String, String)> {
    match &event.payload {
        EventPayload::SessionStarted {
            provider,
            model,
            permission_mode,
            ..
        } => Some((
            format!("Harness session started with {provider}:{model} ({permission_mode})."),
            "stdout".to_string(),
        )),
        EventPayload::ToolStarted { tool, summary, .. } => Some((
            if summary.trim().is_empty() {
                format!("Running `{tool}`.")
            } else {
                format!("{tool}: {summary}")
            },
            "stdout".to_string(),
        )),
        EventPayload::ToolFinished {
            tool,
            summary,
            success,
            ..
        } => Some((
            if *success {
                format!("{tool}: {summary}")
            } else {
                format!("{tool} finished with issues: {summary}")
            },
            if *success {
                "stdout".to_string()
            } else {
                "stderr".to_string()
            },
        )),
        EventPayload::ToolFailed { tool, error, .. } => {
            Some((format!("{tool} failed: {error}"), "stderr".to_string()))
        }
        EventPayload::ApprovalRequested { tool, reason, .. } => Some((
            format!("Approval requested for `{tool}`: {reason}"),
            "stdout".to_string(),
        )),
        EventPayload::ApprovalResolved { tool, approved, .. } => Some((
            if *approved {
                format!("Approved `{tool}`.")
            } else {
                format!("Rejected `{tool}`.")
            },
            "stdout".to_string(),
        )),
        EventPayload::PatchGenerated { files } => Some((
            format!("Patch prepared for {}.", files.join(", ")),
            "stdout".to_string(),
        )),
        EventPayload::VerificationStarted { command } => Some((
            format!("Running verification: {command}"),
            "stdout".to_string(),
        )),
        EventPayload::VerificationFinished {
            command, success, ..
        } => Some((
            if *success {
                format!("Verification passed: {command}")
            } else {
                format!("Verification failed: {command}")
            },
            if *success {
                "stdout".to_string()
            } else {
                "stderr".to_string()
            },
        )),
        EventPayload::CritiqueStarted { reason } => Some((
            format!(
                "Runtime critique started: {}.",
                format_critique_reason(reason)
            ),
            "stdout".to_string(),
        )),
        EventPayload::CritiqueFinished { reason, summary } => Some((
            format!(
                "Runtime critique finished: {}. {}",
                format_critique_reason(reason),
                truncate_text(summary, 200)
            ),
            "stdout".to_string(),
        )),
        EventPayload::ProgressUpdated { progress } => {
            Some((summarize_progress(progress), "stdout".to_string()))
        }
        EventPayload::SessionCompleted { summary } => Some((summary.clone(), "stdout".to_string())),
        EventPayload::SessionFailed { error } => Some((error.clone(), "stderr".to_string())),
        EventPayload::MessageAssistant { message } => {
            Some((truncate_text(message, 160), "stdout".to_string()))
        }
        EventPayload::MessageUser { .. } => None,
    }
}

fn emit_batch_events(app: &AppHandle, runtime_id: &str, events: &[HarnessRuntimeEvent]) {
    for event in events {
        if let Some((line, stream)) = map_event_to_output_line(event) {
            emit_runtime_event(
                app,
                RuntimeEventPayload::Output {
                    runtime_id: runtime_id.to_string(),
                    line,
                    stream,
                    timestamp: format_relative_timestamp_ms(event.timestamp_ms),
                },
            );
        }

        if let EventPayload::SessionFailed { error } = &event.payload {
            emit_runtime_event(
                app,
                RuntimeEventPayload::Error {
                    runtime_id: runtime_id.to_string(),
                    message: error.clone(),
                },
            );
        }
    }
}

fn spawn_browser_stderr_thread(app: AppHandle, runtime_id: String, stderr: ChildStderr) {
    thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines().map_while(Result::ok) {
            if line.trim().is_empty() {
                continue;
            }

            emit_browser_runtime_event(
                &app,
                BrowserRuntimeEventPayload::Output {
                    runtime_id: runtime_id.clone(),
                    line,
                    stream: "stderr".to_string(),
                },
            );
        }
    });
}

fn send_browser_stdio_request(
    runtime: &mut BrowserRuntimeProcess,
    command: Vec<String>,
    session: Option<String>,
    request_id: Option<String>,
) -> Result<BrowserCommandResult, String> {
    let request = serde_json::json!({
        "id": request_id.unwrap_or_else(|| Uuid::new_v4().to_string()),
        "command": command,
        "session": session,
    });
    let payload = serde_json::to_string(&request).map_err(|error| error.to_string())?;
    writeln!(runtime.stdin, "{payload}").map_err(|error| error.to_string())?;
    runtime.stdin.flush().map_err(|error| error.to_string())?;

    let mut line = String::new();
    let bytes_read = runtime
        .stdout
        .read_line(&mut line)
        .map_err(|error| error.to_string())?;
    if bytes_read == 0 {
        let status = runtime
            .child
            .try_wait()
            .map_err(|error| error.to_string())?
            .and_then(|status| status.code());
        return Err(match status {
            Some(code) => format!("agent-browser exited before responding (code {code})."),
            None => "agent-browser closed its stdio stream before responding.".to_string(),
        });
    }

    serde_json::from_str::<BrowserCommandResult>(line.trim())
        .map_err(|error| format!("Invalid JSON response from agent-browser: {error}"))
}

#[tauri::command]
fn load_desktop_settings(app: AppHandle) -> Result<DesktopSettings, String> {
    let path = settings_path(&app)?;

    if !path.exists() {
        return Ok(DesktopSettings::default());
    }

    let contents = fs::read_to_string(&path).map_err(|error| error.to_string())?;
    let settings =
        serde_json::from_str::<DesktopSettings>(&contents).map_err(|error| error.to_string())?;
    let normalized = normalize_desktop_settings(settings.clone());

    if normalized != settings {
        let updated =
            serde_json::to_string_pretty(&normalized).map_err(|error| error.to_string())?;
        fs::write(&path, updated).map_err(|error| error.to_string())?;
    }

    Ok(normalized)
}

#[tauri::command]
fn save_desktop_settings(app: AppHandle, settings: DesktopSettings) -> Result<(), String> {
    let path = settings_path(&app)?;
    let normalized = normalize_desktop_settings(settings);
    let contents = serde_json::to_string_pretty(&normalized).map_err(|error| error.to_string())?;
    fs::write(path, contents).map_err(|error| error.to_string())
}

#[tauri::command]
fn check_claw_backend(
    app: AppHandle,
    backend_path: String,
    workspace_path: Option<String>,
    provider_id: Option<String>,
    selected_model: Option<String>,
) -> Result<BackendHealth, String> {
    let normalized_provider = provider_id
        .as_deref()
        .map(normalize_provider_id)
        .unwrap_or("local");
    let local_runtime = if normalized_provider == "local" {
        Some(check_local_runtime_health(selected_model.as_deref()))
    } else {
        None
    };

    let requested_path = backend_path.clone();
    let resolved_path = resolve_backend_path(&backend_path);
    let (sessions_directory, session_count) = if let Some(path) = workspace_path.as_deref() {
        let workspace_path = PathBuf::from(normalize_workspace_path(path.to_string()));
        let directory = workspace_session_root(&app, &workspace_path)?;
        let count = JsonSessionStore::new(directory.clone())
            .list_sessions()
            .map(|sessions| sessions.len())
            .unwrap_or_default();
        (Some(directory.display().to_string()), count)
    } else {
        (None, 0)
    };

    let Some(resolved_path) = resolved_path else {
        return Ok(BackendHealth {
            requested_path,
            resolved_path: None,
            exists: false,
            runnable: false,
            version: None,
            status: "missing".to_string(),
            message: "Poro could not find the configured `harness-server` binary. Build it in the sibling `/Users/cyberowl/Repos/harness` repo first.".to_string(),
            sessions_directory,
            session_count,
            local_runtime,
        });
    };

    let exists = resolved_path.exists();
    let runnable = exists && is_executable(&resolved_path);
    let version = if runnable {
        std::process::Command::new(&resolved_path)
            .arg("--version")
            .output()
            .ok()
            .and_then(|output| {
                if output.status.success() {
                    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
                } else {
                    None
                }
            })
    } else {
        None
    };

    let message = if !runnable {
        "Poro found the `harness-server` path, but it is not executable yet.".to_string()
    } else if let Some(provider_message) =
        provider_message_for_health(normalized_provider, selected_model.as_deref())
    {
        provider_message
    } else {
        match &local_runtime {
            Some(local_runtime) if !local_runtime.reachable => {
                "Harness is available, but Ollama is not responding on http://127.0.0.1:11434."
                    .to_string()
            }
            Some(local_runtime) if !local_runtime.has_selected_model => match selected_model.as_deref()
            {
                Some(model) => format!(
                    "Harness is ready and Ollama is running. Pull `{model}` before launching your first local session."
                ),
                None => "Harness is ready and Ollama is running.".to_string(),
            },
            Some(_) => {
                "Harness and Ollama look healthy. You can launch a local Gemma session from Poro."
                    .to_string()
            }
            None => "Harness binary looks healthy. You can launch a session from the desktop shell."
                .to_string(),
        }
    };

    Ok(BackendHealth {
        requested_path,
        resolved_path: Some(resolved_path.display().to_string()),
        exists,
        runnable,
        version,
        status: if runnable {
            "ready".to_string()
        } else {
            "not-runnable".to_string()
        },
        message,
        sessions_directory,
        session_count,
        local_runtime,
    })
}

#[tauri::command]
fn list_claw_sessions(
    app: AppHandle,
    workspace_path: String,
) -> Result<Vec<ClawSessionSummary>, String> {
    let workspace_path = PathBuf::from(normalize_workspace_path(workspace_path));
    let mut summaries = list_workspace_session_records(&app, &workspace_path)?
        .into_iter()
        .map(|(path, record)| summary_from_record(&path, &record))
        .collect::<Vec<_>>();

    summaries.sort_by(|left, right| right.modified_at.cmp(&left.modified_at));
    Ok(summaries)
}

#[tauri::command]
fn load_claw_session(
    app: AppHandle,
    workspace_path: String,
    session_path: String,
) -> Result<ClawSessionSnapshot, String> {
    let workspace_path = PathBuf::from(normalize_workspace_path(workspace_path));
    let allowed_root = workspace_session_root(&app, &workspace_path)?;
    let session_path = PathBuf::from(session_path);

    if !session_path.starts_with(&allowed_root) {
        return Err("Requested session path is outside the workspace session store.".to_string());
    }

    load_snapshot(&session_path)
}

#[tauri::command]
fn delete_claw_session(
    app: AppHandle,
    manager: State<'_, RuntimeManager>,
    workspace_path: String,
    session_path: String,
) -> Result<(), String> {
    let workspace_path = PathBuf::from(normalize_workspace_path(workspace_path));
    let allowed_root = workspace_session_root(&app, &workspace_path)?;
    let session_path = PathBuf::from(session_path);

    if !session_path.starts_with(&allowed_root) {
        return Err("Requested session path is outside the workspace session store.".to_string());
    }

    {
        let runtimes = manager.runtimes.lock().map_err(|error| error.to_string())?;
        if runtimes
            .values()
            .any(|runtime| runtime.session_path == session_path)
        {
            return Err(
                "Stop the active Harness runtime before deleting this session.".to_string(),
            );
        }
    }

    if !session_path.exists() {
        return Ok(());
    }

    fs::remove_file(&session_path).map_err(|error| error.to_string())?;
    Ok(())
}

#[tauri::command]
fn start_claw_runtime(
    app: AppHandle,
    manager: State<'_, RuntimeManager>,
    request: LaunchRuntimeRequest,
) -> Result<RuntimeLaunch, String> {
    let resolved_path = resolve_backend_path(&request.backend_path)
        .ok_or_else(|| "Poro could not find the configured `harness-server` binary.".to_string())?;
    if !resolved_path.exists() || !is_executable(&resolved_path) {
        return Err(
            "The configured `harness-server` binary exists, but it is not executable yet."
                .to_string(),
        );
    }

    let browser_path = ensure_agent_browser_path(None)?;
    env::set_var("PORO_AGENT_BROWSER_BIN", &browser_path);
    if env::var_os("AGENT_BROWSER_HEADED").is_none() {
        env::set_var("AGENT_BROWSER_HEADED", "1");
    }

    let workspace_path = PathBuf::from(normalize_workspace_path(request.workspace_path));
    let session_root = workspace_session_root(&app, &workspace_path)?;
    let provider = provider_config_for_runtime(&request.provider_id, &request.model)?;
    let permission_mode = permission_mode_from_str(&request.permission_mode)?;

    let mut client = HarnessServerClient::spawn(HarnessClientConfig {
        server_bin: resolved_path,
        workspace_root: workspace_path.clone(),
        session_root: session_root.clone(),
        provider,
        permission_mode,
    })
    .map_err(|error| error.to_string())?;

    let resume_path = request.resume_session_path.as_ref().map(PathBuf::from);
    let (state, resumed) = match resume_path
        .as_ref()
        .filter(|path| path.starts_with(&session_root) && path.exists())
        .and_then(|path| session_id_from_path(path))
    {
        Some(session_id) => (
            client
                .load_session_state(session_id)
                .map_err(|error| error.to_string())?,
            true,
        ),
        None => (
            client.start_session().map_err(|error| error.to_string())?,
            false,
        ),
    };

    let runtime_id = Uuid::new_v4().to_string();
    let session_id = state.session.metadata.session_id;
    let session_path = session_path_for_root(&session_root, session_id);
    let launch = RuntimeLaunch {
        runtime_id: runtime_id.clone(),
        workspace_path: workspace_path.display().to_string(),
        provider_id: request.provider_id.clone(),
        model: request.model.clone(),
        permission_mode: request.permission_mode.clone(),
        session_id: Some(session_id.to_string()),
        session_path: Some(session_path.display().to_string()),
        resumed,
        message: if resumed {
            "Reattached Poro to an existing Harness session.".to_string()
        } else {
            "Harness runtime is ready for this workspace.".to_string()
        },
    };

    {
        let mut runtimes = manager.runtimes.lock().map_err(|error| error.to_string())?;
        runtimes.insert(
            runtime_id.clone(),
            RuntimeHandle {
                client: Arc::new(Mutex::new(client)),
                session_id,
                session_path: session_path.clone(),
                workspace_path: workspace_path.clone(),
                turn_in_flight: Arc::new(Mutex::new(false)),
            },
        );
    }

    emit_runtime_event(
        &app,
        RuntimeEventPayload::Started {
            launch: launch.clone(),
        },
    );
    emit_runtime_event(
        &app,
        RuntimeEventPayload::SessionAttached {
            runtime_id: runtime_id.clone(),
            session_id: session_id.to_string(),
            session_path: session_path.display().to_string(),
            resumed,
        },
    );
    emit_runtime_event(
        &app,
        RuntimeEventPayload::Snapshot {
            runtime_id,
            snapshot: snapshot_from_record(&session_path, &state.session),
        },
    );

    Ok(launch)
}

#[tauri::command]
fn send_claw_input(
    app: AppHandle,
    manager: State<'_, RuntimeManager>,
    runtime_id: String,
    input: String,
) -> Result<(), String> {
    let (client, session_id, session_path, turn_in_flight) = {
        let runtimes = manager.runtimes.lock().map_err(|error| error.to_string())?;
        let runtime = runtimes
            .get(&runtime_id)
            .ok_or_else(|| format!("Runtime `{runtime_id}` is not active."))?;
        (
            Arc::clone(&runtime.client),
            runtime.session_id,
            runtime.session_path.clone(),
            Arc::clone(&runtime.turn_in_flight),
        )
    };

    {
        let mut in_flight = turn_in_flight.lock().map_err(|error| error.to_string())?;
        if *in_flight {
            return Err(
                "Harness is still working on the previous turn for this workspace.".to_string(),
            );
        }
        *in_flight = true;
    }

    let input_preview = truncate_text(&input, 120);
    emit_runtime_event(
        &app,
        RuntimeEventPayload::TurnStarted {
            runtime_id: runtime_id.clone(),
            input_preview,
        },
    );

    let app_handle = app.clone();
    let runtime_id_for_thread = runtime_id.clone();
    thread::spawn(move || {
        let started_at = Instant::now();
        let result = (|| -> Result<(), String> {
            let events = {
                let mut client = client.lock().map_err(|error| error.to_string())?;
                client
                    .submit_user_message(session_id, input)
                    .map_err(|error| error.to_string())?
                    .events
            };

            emit_batch_events(&app_handle, &runtime_id_for_thread, &events);
            emit_runtime_event(
                &app_handle,
                RuntimeEventPayload::Snapshot {
                    runtime_id: runtime_id_for_thread.clone(),
                    snapshot: load_snapshot(&session_path)?,
                },
            );
            Ok(())
        })();

        if let Err(error) = &result {
            emit_runtime_event(
                &app_handle,
                RuntimeEventPayload::Error {
                    runtime_id: runtime_id_for_thread.clone(),
                    message: error.clone(),
                },
            );
        }

        if let Ok(mut in_flight) = turn_in_flight.lock() {
            *in_flight = false;
        }

        emit_runtime_event(
            &app_handle,
            RuntimeEventPayload::TurnFinished {
                runtime_id: runtime_id_for_thread,
                success: result.is_ok(),
                duration_ms: started_at.elapsed().as_millis(),
            },
        );
    });

    Ok(())
}

#[tauri::command]
fn stop_claw_runtime(
    app: AppHandle,
    manager: State<'_, RuntimeManager>,
    runtime_id: String,
) -> Result<(), String> {
    let runtime = {
        let mut runtimes = manager.runtimes.lock().map_err(|error| error.to_string())?;
        runtimes
            .remove(&runtime_id)
            .ok_or_else(|| format!("Runtime `{runtime_id}` is not active."))?
    };

    let workspace_label = runtime.workspace_path.display().to_string();
    let mut client = runtime.client.lock().map_err(|error| error.to_string())?;
    client.shutdown().map_err(|error| error.to_string())?;

    emit_runtime_event(
        &app,
        RuntimeEventPayload::Stopped {
            runtime_id,
            code: Some(0),
            message: format!("Harness runtime stopped for {workspace_label}."),
        },
    );

    Ok(())
}

#[tauri::command]
fn start_browser_runtime(
    app: AppHandle,
    manager: State<'_, BrowserRuntimeManager>,
    request: BrowserLaunchRequest,
) -> Result<BrowserRuntimeLaunch, String> {
    let browser_path = ensure_agent_browser_path(request.browser_path.as_deref())?;
    let session = request
        .session
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(default_browser_session_value);
    let headless = request.headless.unwrap_or(true);
    let runtime_id = Uuid::new_v4().to_string();

    let mut command = Command::new(&browser_path);
    command.arg("--session").arg(&session);
    if headless {
        command.arg("--headless");
    }
    command
        .arg("stdio")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = command
        .spawn()
        .map_err(|error| format!("Could not start agent-browser: {error}"))?;
    let stdin = child
        .stdin
        .take()
        .ok_or_else(|| "Poro could not capture agent-browser stdin.".to_string())?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| "Poro could not capture agent-browser stdout.".to_string())?;
    if let Some(stderr) = child.stderr.take() {
        spawn_browser_stderr_thread(app.clone(), runtime_id.clone(), stderr);
    }

    let process = BrowserRuntimeProcess {
        child,
        stdin,
        stdout: BufReader::new(stdout),
        session: session.clone(),
        browser_path: browser_path.clone(),
    };
    manager
        .runtimes
        .lock()
        .map_err(|error| error.to_string())?
        .insert(runtime_id.clone(), Arc::new(Mutex::new(process)));

    let launch = BrowserRuntimeLaunch {
        runtime_id: runtime_id.clone(),
        session,
        browser_path: browser_path.display().to_string(),
        headless,
        message: "Browser runtime is ready in stdio mode.".to_string(),
    };

    emit_browser_runtime_event(
        &app,
        BrowserRuntimeEventPayload::Started {
            launch: launch.clone(),
        },
    );

    Ok(launch)
}

#[tauri::command]
fn send_browser_command(
    app: AppHandle,
    manager: State<'_, BrowserRuntimeManager>,
    request: BrowserCommandRequest,
) -> Result<BrowserCommandResult, String> {
    if request.command.is_empty() {
        return Err("Browser command requests need a non-empty command array.".to_string());
    }

    let runtime = {
        let runtimes = manager.runtimes.lock().map_err(|error| error.to_string())?;
        runtimes
            .get(&request.runtime_id)
            .cloned()
            .ok_or_else(|| format!("Browser runtime `{}` is not active.", request.runtime_id))?
    };

    let runtime_id = request.runtime_id.clone();
    let response = {
        let mut runtime = runtime.lock().map_err(|error| error.to_string())?;
        send_browser_stdio_request(
            &mut runtime,
            request.command,
            request.session.clone(),
            request.request_id,
        )
    };

    let response = match response {
        Ok(response) => response,
        Err(error) => {
            emit_browser_runtime_event(
                &app,
                BrowserRuntimeEventPayload::Error {
                    runtime_id,
                    message: error.clone(),
                },
            );
            return Err(error);
        }
    };

    emit_browser_runtime_event(
        &app,
        BrowserRuntimeEventPayload::Response {
            runtime_id: request.runtime_id,
            response: response.clone(),
        },
    );

    Ok(response)
}

#[tauri::command]
fn stop_browser_runtime(
    app: AppHandle,
    manager: State<'_, BrowserRuntimeManager>,
    runtime_id: String,
) -> Result<(), String> {
    let runtime = {
        let mut runtimes = manager.runtimes.lock().map_err(|error| error.to_string())?;
        runtimes
            .remove(&runtime_id)
            .ok_or_else(|| format!("Browser runtime `{runtime_id}` is not active."))?
    };

    let (session, browser_path, code) = {
        let mut runtime = runtime.lock().map_err(|error| error.to_string())?;
        let session = runtime.session.clone();
        let browser_path = runtime.browser_path.display().to_string();

        let _ = send_browser_stdio_request(&mut runtime, vec!["close".to_string()], None, None);
        let _ = runtime.child.kill();
        let code = runtime
            .child
            .wait()
            .map_err(|error| error.to_string())?
            .code();
        (session, browser_path, code)
    };

    emit_browser_runtime_event(
        &app,
        BrowserRuntimeEventPayload::Stopped {
            runtime_id,
            code,
            message: format!("Browser runtime for `{session}` stopped ({browser_path})."),
        },
    );

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(RuntimeManager::default())
        .manage(BrowserRuntimeManager::default())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            load_desktop_settings,
            save_desktop_settings,
            check_claw_backend,
            list_claw_sessions,
            load_claw_session,
            delete_claw_session,
            start_claw_runtime,
            send_claw_input,
            stop_claw_runtime,
            start_browser_runtime,
            send_browser_command,
            stop_browser_runtime
        ])
        .run(tauri::generate_context!())
        .expect("error while running Poro desktop");
}
