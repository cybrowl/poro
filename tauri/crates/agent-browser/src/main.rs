mod browser_cache;
mod color;
mod commands;
mod connection;
mod flags;
mod native;
mod output;
#[cfg(test)]
mod test_utils;
mod validation;

use serde_json::json;
use std::env;
use std::fs;
use std::process::exit;

#[cfg(windows)]
use windows_sys::Win32::Foundation::CloseHandle;
#[cfg(windows)]
use windows_sys::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION};

use commands::{gen_id, parse_command, ParseError};
use connection::{
    cleanup_stale_files, ensure_daemon, get_socket_dir, send_command, DaemonOptions, Response,
};
use flags::{clean_args, parse_flags, Flags};
use output::{
    print_command_help, print_help, print_response_with_opts, print_version, OutputOptions,
};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};

fn serialize_json_value(value: &serde_json::Value) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| {
        r#"{"success":false,"error":"Failed to serialize JSON response"}"#.to_string()
    })
}

fn print_json_value(value: serde_json::Value) {
    println!("{}", serialize_json_value(&value));
}

fn print_json_error(message: impl AsRef<str>) {
    print_json_value(json!({
        "success": false,
        "error": message.as_ref(),
    }));
}

fn print_json_error_with_type(message: impl AsRef<str>, error_type: &str) {
    print_json_value(json!({
        "success": false,
        "error": message.as_ref(),
        "type": error_type,
    }));
}

struct ParsedProxy {
    server: String,
    username: Option<String>,
    password: Option<String>,
}

#[derive(Debug, Deserialize)]
struct StdioRequest {
    #[serde(default)]
    id: Option<String>,
    command: Vec<String>,
    #[serde(default)]
    session: Option<String>,
}

#[derive(Debug, Serialize)]
struct StdioResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    session: String,
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    warning: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_type: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum BaselineMode {
    Capture,
    Compare,
    Approve,
}

#[derive(Debug, Clone)]
struct BaselineCommand {
    mode: BaselineMode,
    name: String,
    slug: String,
    selector: Option<String>,
    full_page: bool,
    threshold: Option<f64>,
}

#[derive(Debug, Clone)]
struct BaselinePaths {
    directory: PathBuf,
    baseline: PathBuf,
    current: PathBuf,
    diff: PathBuf,
    meta: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct BaselineMetadata {
    name: String,
    slug: String,
    session: String,
    selector: Option<String>,
    full_page: bool,
    threshold: f64,
    updated_at: String,
    baseline_path: String,
    current_path: String,
    diff_path: String,
}

#[derive(Debug)]
struct BaselineExecution {
    action: String,
    success: bool,
    data: serde_json::Value,
    error: Option<String>,
}

fn slugify_baseline_name(name: &str) -> Option<String> {
    let mut slug = String::with_capacity(name.len());
    let mut last_dash = false;

    for ch in name.trim().chars() {
        let normalized = if ch.is_ascii_alphanumeric() {
            last_dash = false;
            Some(ch.to_ascii_lowercase())
        } else if matches!(ch, '-' | '_' | '.' | ' ') {
            if last_dash {
                None
            } else {
                last_dash = true;
                Some('-')
            }
        } else if last_dash {
            None
        } else {
            last_dash = true;
            Some('-')
        };

        if let Some(value) = normalized {
            slug.push(value);
        }
    }

    let slug = slug.trim_matches('-').to_string();
    if slug.is_empty() {
        None
    } else {
        Some(slug)
    }
}

fn parse_baseline_command(args: &[String]) -> Result<BaselineCommand, ParseError> {
    let subcommand =
        args.get(1)
            .map(|value| value.as_str())
            .ok_or_else(|| ParseError::MissingArguments {
                context: "baseline".to_string(),
                usage: "baseline <capture|compare|approve> <name>",
            })?;

    let mode = match subcommand {
        "capture" => BaselineMode::Capture,
        "compare" => BaselineMode::Compare,
        "approve" => BaselineMode::Approve,
        other => {
            return Err(ParseError::UnknownSubcommand {
                subcommand: other.to_string(),
                valid_options: &["capture", "compare", "approve"],
            });
        }
    };

    let name = args.get(2).ok_or_else(|| ParseError::MissingArguments {
        context: format!("baseline {}", subcommand),
        usage: "baseline <capture|compare|approve> <name>",
    })?;

    let slug = slugify_baseline_name(name).ok_or_else(|| ParseError::InvalidValue {
        message: format!("Invalid baseline name: '{}'", name),
        usage: "baseline <capture|compare|approve> <name>",
    })?;

    let mut selector = None;
    let mut full_page = false;
    let mut threshold = None;
    let mut index = 3;

    while index < args.len() {
        match args[index].as_str() {
            "-s" | "--selector" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| ParseError::MissingArguments {
                        context: format!("baseline {} --selector", subcommand),
                        usage:
                            "baseline <capture|compare> <name> [--selector <sel>] [--full/-f] [--threshold <0-1>]",
                    })?;
                selector = Some(value.clone());
                index += 1;
            }
            "--full" | "-f" => {
                full_page = true;
            }
            "-t" | "--threshold" => {
                if !matches!(mode, BaselineMode::Compare) {
                    return Err(ParseError::InvalidValue {
                        message: "--threshold only applies to baseline compare".to_string(),
                        usage: "baseline compare <name> [--threshold <0-1>]",
                    });
                }
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| ParseError::MissingArguments {
                        context: "baseline compare --threshold".to_string(),
                        usage: "baseline compare <name> [--threshold <0-1>]",
                    })?;
                let parsed = value.parse::<f64>().map_err(|_| ParseError::InvalidValue {
                    message: format!("Invalid threshold value: {}", value),
                    usage: "baseline compare <name> [--threshold <0-1>]",
                })?;
                if !(0.0..=1.0).contains(&parsed) {
                    return Err(ParseError::InvalidValue {
                        message: format!("Threshold must be between 0 and 1, got {}", parsed),
                        usage: "baseline compare <name> [--threshold <0-1>]",
                    });
                }
                threshold = Some(parsed);
                index += 1;
            }
            other => {
                return Err(ParseError::InvalidValue {
                    message: format!("Unknown flag: {}", other),
                    usage:
                        "baseline <capture|compare> <name> [--selector <sel>] [--full/-f] [--threshold <0-1>]",
                });
            }
        }
        index += 1;
    }

    Ok(BaselineCommand {
        mode,
        name: name.clone(),
        slug,
        selector,
        full_page,
        threshold,
    })
}

fn baseline_root_dir() -> PathBuf {
    if let Ok(path) = env::var("AGENT_BROWSER_BASELINE_DIR") {
        if !path.trim().is_empty() {
            return PathBuf::from(path);
        }
    }

    env::current_dir()
        .unwrap_or_else(|_| env::temp_dir())
        .join(".agent-browser")
        .join("baselines")
}

fn baseline_paths(slug: &str) -> BaselinePaths {
    let directory = baseline_root_dir().join(slug);
    BaselinePaths {
        baseline: directory.join("baseline.png"),
        current: directory.join("current.png"),
        diff: directory.join("diff.png"),
        meta: directory.join("meta.json"),
        directory,
    }
}

fn read_baseline_metadata(path: &Path) -> Option<BaselineMetadata> {
    fs::read_to_string(path)
        .ok()
        .and_then(|content| serde_json::from_str::<BaselineMetadata>(&content).ok())
}

fn write_baseline_metadata(path: &Path, metadata: &BaselineMetadata) -> Result<(), String> {
    let serialized = serde_json::to_string_pretty(metadata)
        .map_err(|error| format!("Failed to serialize baseline metadata: {}", error))?;
    fs::write(path, serialized).map_err(|error| {
        format!(
            "Failed to write baseline metadata to {}: {}",
            path.display(),
            error
        )
    })
}

fn capture_screenshot_to_path(
    flags: &Flags,
    session: &str,
    path: &Path,
    selector: Option<&str>,
    full_page: bool,
) -> Result<Response, String> {
    let mut command = json!({
        "id": gen_id(),
        "action": "screenshot",
        "path": path.to_string_lossy().to_string(),
        "fullPage": full_page,
    });

    if let Some(selector) = selector {
        command["selector"] = json!(selector);
    }

    let (response, _) = execute_supported_command(flags, session, command, false)?;
    Ok(response)
}

fn run_diff_screenshot(
    flags: &Flags,
    session: &str,
    baseline_path: &Path,
    diff_path: &Path,
    selector: Option<&str>,
    full_page: bool,
    threshold: f64,
) -> Result<Response, String> {
    let mut command = json!({
        "id": gen_id(),
        "action": "diff_screenshot",
        "baseline": baseline_path.to_string_lossy().to_string(),
        "output": diff_path.to_string_lossy().to_string(),
        "threshold": threshold,
        "fullPage": full_page,
    });

    if let Some(selector) = selector {
        command["selector"] = json!(selector);
    }

    let (response, _) = execute_supported_command(flags, session, command, false)?;
    Ok(response)
}

fn execute_baseline_command(
    flags: &Flags,
    session: &str,
    command: &BaselineCommand,
) -> Result<BaselineExecution, String> {
    let paths = baseline_paths(&command.slug);
    fs::create_dir_all(&paths.directory).map_err(|error| {
        format!(
            "Failed to create baseline directory {}: {}",
            paths.directory.display(),
            error
        )
    })?;

    let existing_meta = read_baseline_metadata(&paths.meta).unwrap_or_default();
    let selector = command
        .selector
        .clone()
        .or(existing_meta.selector.clone())
        .filter(|value| !value.is_empty());
    let full_page = command.full_page || existing_meta.full_page;
    let threshold = command.threshold.unwrap_or_else(|| {
        if existing_meta.threshold > 0.0 {
            existing_meta.threshold
        } else {
            0.1
        }
    });

    match command.mode {
        BaselineMode::Capture => {
            let response = capture_screenshot_to_path(
                flags,
                session,
                &paths.baseline,
                selector.as_deref(),
                full_page,
            )?;
            if !response.success {
                return Ok(BaselineExecution {
                    action: "baseline_capture".to_string(),
                    success: false,
                    data: json!({}),
                    error: response.error,
                });
            }

            let _ = fs::remove_file(&paths.current);
            let _ = fs::remove_file(&paths.diff);

            let metadata = BaselineMetadata {
                name: command.name.clone(),
                slug: command.slug.clone(),
                session: session.to_string(),
                selector,
                full_page,
                threshold,
                updated_at: chrono::Utc::now().to_rfc3339(),
                baseline_path: paths.baseline.to_string_lossy().to_string(),
                current_path: paths.current.to_string_lossy().to_string(),
                diff_path: paths.diff.to_string_lossy().to_string(),
            };
            write_baseline_metadata(&paths.meta, &metadata)?;

            Ok(BaselineExecution {
                action: "baseline_capture".to_string(),
                success: true,
                data: json!({
                    "name": command.name,
                    "slug": command.slug,
                    "baselinePath": paths.baseline,
                    "metaPath": paths.meta,
                    "selector": metadata.selector,
                    "fullPage": metadata.full_page,
                    "threshold": metadata.threshold,
                }),
                error: None,
            })
        }
        BaselineMode::Compare => {
            if !paths.baseline.exists() {
                return Err(format!(
                    "No saved baseline found for '{}' at {}. Run `agent-browser baseline capture {}` first.",
                    command.name,
                    paths.baseline.display(),
                    command.slug
                ));
            }

            let _ = fs::remove_file(&paths.diff);
            let response = capture_screenshot_to_path(
                flags,
                session,
                &paths.current,
                selector.as_deref(),
                full_page,
            )?;
            if !response.success {
                return Ok(BaselineExecution {
                    action: "baseline_compare".to_string(),
                    success: false,
                    data: json!({}),
                    error: response.error,
                });
            }

            let diff_response = run_diff_screenshot(
                flags,
                session,
                &paths.baseline,
                &paths.diff,
                selector.as_deref(),
                full_page,
                threshold,
            )?;

            if !diff_response.success {
                return Ok(BaselineExecution {
                    action: "baseline_compare".to_string(),
                    success: false,
                    data: json!({}),
                    error: diff_response.error,
                });
            }

            let mut data = diff_response.data.unwrap_or_else(|| json!({}));
            let matched = data
                .get("match")
                .and_then(|value| value.as_bool())
                .unwrap_or(false);

            if !paths.diff.exists() {
                if let Some(object) = data.as_object_mut() {
                    object.insert("diffPath".to_string(), serde_json::Value::Null);
                }
            }

            let metadata = BaselineMetadata {
                name: command.name.clone(),
                slug: command.slug.clone(),
                session: session.to_string(),
                selector,
                full_page,
                threshold,
                updated_at: chrono::Utc::now().to_rfc3339(),
                baseline_path: paths.baseline.to_string_lossy().to_string(),
                current_path: paths.current.to_string_lossy().to_string(),
                diff_path: paths.diff.to_string_lossy().to_string(),
            };
            write_baseline_metadata(&paths.meta, &metadata)?;

            if let Some(object) = data.as_object_mut() {
                object.insert(
                    "baselinePath".to_string(),
                    json!(paths.baseline.to_string_lossy().to_string()),
                );
                object.insert(
                    "currentPath".to_string(),
                    json!(paths.current.to_string_lossy().to_string()),
                );
                object.insert(
                    "metaPath".to_string(),
                    json!(paths.meta.to_string_lossy().to_string()),
                );
                object.insert("name".to_string(), json!(command.name.clone()));
                object.insert("slug".to_string(), json!(command.slug.clone()));
            }

            Ok(BaselineExecution {
                action: "baseline_compare".to_string(),
                success: matched,
                data,
                error: if matched {
                    None
                } else {
                    Some("Baseline mismatch".to_string())
                },
            })
        }
        BaselineMode::Approve => {
            if !paths.current.exists() {
                if !paths.baseline.exists() {
                    return Err(format!(
                        "No baseline state exists for '{}'. Run `agent-browser baseline capture {}` first.",
                        command.name, command.slug
                    ));
                }

                let response = capture_screenshot_to_path(
                    flags,
                    session,
                    &paths.current,
                    selector.as_deref(),
                    full_page,
                )?;
                if !response.success {
                    return Ok(BaselineExecution {
                        action: "baseline_approve".to_string(),
                        success: false,
                        data: json!({}),
                        error: response.error,
                    });
                }
            }

            fs::copy(&paths.current, &paths.baseline).map_err(|error| {
                format!(
                    "Failed to update baseline {}: {}",
                    paths.baseline.display(),
                    error
                )
            })?;
            let _ = fs::remove_file(&paths.diff);

            let metadata = BaselineMetadata {
                name: command.name.clone(),
                slug: command.slug.clone(),
                session: session.to_string(),
                selector,
                full_page,
                threshold,
                updated_at: chrono::Utc::now().to_rfc3339(),
                baseline_path: paths.baseline.to_string_lossy().to_string(),
                current_path: paths.current.to_string_lossy().to_string(),
                diff_path: paths.diff.to_string_lossy().to_string(),
            };
            write_baseline_metadata(&paths.meta, &metadata)?;

            Ok(BaselineExecution {
                action: "baseline_approve".to_string(),
                success: true,
                data: json!({
                    "name": command.name,
                    "slug": command.slug,
                    "baselinePath": paths.baseline,
                    "currentPath": paths.current,
                    "metaPath": paths.meta,
                }),
                error: None,
            })
        }
    }
}

fn parse_proxy(proxy_str: &str) -> ParsedProxy {
    let Some(protocol_end) = proxy_str.find("://") else {
        return ParsedProxy {
            server: proxy_str.to_string(),
            username: None,
            password: None,
        };
    };
    let protocol = &proxy_str[..protocol_end + 3];
    let rest = &proxy_str[protocol_end + 3..];

    let Some(at_pos) = rest.rfind('@') else {
        return ParsedProxy {
            server: proxy_str.to_string(),
            username: None,
            password: None,
        };
    };

    let creds = &rest[..at_pos];
    let server_part = &rest[at_pos + 1..];
    let server = format!("{}{}", protocol, server_part);

    let (username, password) = match creds.find(':') {
        Some(colon_pos) => {
            let u = &creds[..colon_pos];
            let p = &creds[colon_pos + 1..];
            (
                if u.is_empty() {
                    None
                } else {
                    Some(u.to_string())
                },
                if p.is_empty() {
                    None
                } else {
                    Some(p.to_string())
                },
            )
        }
        None => (
            if creds.is_empty() {
                None
            } else {
                Some(creds.to_string())
            },
            None,
        ),
    };

    ParsedProxy {
        server,
        username,
        password,
    }
}

fn run_close_all(flags: &Flags) {
    let socket_dir = get_socket_dir();
    let mut sessions: Vec<(String, u32)> = Vec::new();

    if let Ok(entries) = fs::read_dir(&socket_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if let Some(session_name) = name.strip_suffix(".pid") {
                if session_name.is_empty() {
                    continue;
                }
                let pid_path = socket_dir.join(&name);
                if let Ok(pid_str) = fs::read_to_string(&pid_path) {
                    if let Ok(pid) = pid_str.trim().parse::<u32>() {
                        #[cfg(unix)]
                        let running = unsafe {
                            libc::kill(pid as i32, 0) == 0
                                || std::io::Error::last_os_error().raw_os_error()
                                    != Some(libc::ESRCH)
                        };
                        #[cfg(windows)]
                        let running = unsafe {
                            let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
                            if handle != 0 {
                                CloseHandle(handle);
                                true
                            } else {
                                false
                            }
                        };
                        if running {
                            sessions.push((session_name.to_string(), pid));
                        } else {
                            // Process is gone but stale files remain; clean them up
                            cleanup_stale_files(session_name);
                        }
                    }
                } else {
                    // PID file exists but is unreadable; clean up stale files
                    cleanup_stale_files(session_name);
                }
            }
        }
    }

    // Also scan for orphaned .sock files without corresponding .pid files
    #[cfg(unix)]
    if let Ok(entries) = fs::read_dir(&socket_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if let Some(session_name) = name.strip_suffix(".sock") {
                if session_name.is_empty() {
                    continue;
                }
                let pid_path = socket_dir.join(format!("{}.pid", session_name));
                if !pid_path.exists() {
                    // Orphaned socket file with no PID file; remove it
                    cleanup_stale_files(session_name);
                }
            }
        }
    }

    if sessions.is_empty() {
        if flags.json {
            print_json_value(json!({
                "success": true,
                "data": { "closed": 0, "sessions": [] },
            }));
        } else {
            println!("No active sessions");
        }
        return;
    }

    let mut closed: Vec<String> = Vec::new();
    let mut failed: Vec<(String, String)> = Vec::new();

    for (session, pid) in &sessions {
        let cmd = json!({ "id": gen_id(), "action": "close" });
        match send_command(cmd, session) {
            Ok(resp) if resp.success => closed.push(session.clone()),
            Ok(resp) => {
                let err = resp.error.unwrap_or_else(|| "Unknown error".to_string());
                failed.push((session.clone(), err));
            }
            Err(_) => {
                // Daemon is unreachable despite its process existing.
                // Force-kill the process and clean up stale files so future
                // sessions are not poisoned.
                #[cfg(unix)]
                unsafe {
                    libc::kill(*pid as i32, libc::SIGKILL);
                }
                #[cfg(windows)]
                unsafe {
                    let handle = OpenProcess(1, 0, *pid); // PROCESS_TERMINATE = 1
                    if handle != 0 {
                        windows_sys::Win32::System::Threading::TerminateProcess(handle, 1);
                        CloseHandle(handle);
                    }
                }
                cleanup_stale_files(session);
                closed.push(session.clone());
            }
        }
    }

    if flags.json {
        print_json_value(json!({
            "success": failed.is_empty(),
            "data": {
                "closed": closed.len(),
                "sessions": closed,
                "failed": failed.iter().map(|(s, e)| json!({"session": s, "error": e})).collect::<Vec<_>>(),
            },
        }));
    } else {
        for s in &closed {
            println!("{} Closed session: {}", color::green("✓"), s);
        }
        for (s, e) in &failed {
            eprintln!("{} Failed to close {}: {}", color::error_indicator(), s, e);
        }
        if closed.is_empty() && !failed.is_empty() {
            exit(1);
        }
    }

    if !failed.is_empty() {
        exit(1);
    }
}

fn close_all_response(session: String) -> StdioResponse {
    let socket_dir = get_socket_dir();
    let mut sessions: Vec<(String, u32)> = Vec::new();

    if let Ok(entries) = fs::read_dir(&socket_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if let Some(session_name) = name.strip_suffix(".pid") {
                if session_name.is_empty() {
                    continue;
                }
                let pid_path = socket_dir.join(&name);
                if let Ok(pid_str) = fs::read_to_string(&pid_path) {
                    if let Ok(pid) = pid_str.trim().parse::<u32>() {
                        #[cfg(unix)]
                        let running = unsafe {
                            libc::kill(pid as i32, 0) == 0
                                || std::io::Error::last_os_error().raw_os_error()
                                    != Some(libc::ESRCH)
                        };
                        #[cfg(windows)]
                        let running = unsafe {
                            let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
                            if handle != 0 {
                                CloseHandle(handle);
                                true
                            } else {
                                false
                            }
                        };
                        if running {
                            sessions.push((session_name.to_string(), pid));
                        } else {
                            cleanup_stale_files(session_name);
                        }
                    }
                } else {
                    cleanup_stale_files(session_name);
                }
            }
        }
    }

    #[cfg(unix)]
    if let Ok(entries) = fs::read_dir(&socket_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if let Some(session_name) = name.strip_suffix(".sock") {
                if session_name.is_empty() {
                    continue;
                }
                let pid_path = socket_dir.join(format!("{}.pid", session_name));
                if !pid_path.exists() {
                    cleanup_stale_files(session_name);
                }
            }
        }
    }

    if sessions.is_empty() {
        return StdioResponse {
            id: None,
            session,
            success: true,
            action: Some("close_all".to_string()),
            data: Some(json!({ "closed": 0, "sessions": [] })),
            error: None,
            warning: None,
            error_type: None,
        };
    }

    let mut closed: Vec<String> = Vec::new();
    let mut failed: Vec<serde_json::Value> = Vec::new();

    for (target_session, pid) in &sessions {
        let cmd = json!({ "id": gen_id(), "action": "close" });
        match send_command(cmd, target_session) {
            Ok(resp) if resp.success => closed.push(target_session.clone()),
            Ok(resp) => {
                failed.push(json!({
                    "session": target_session,
                    "error": resp.error.unwrap_or_else(|| "Unknown error".to_string()),
                }));
            }
            Err(_) => {
                #[cfg(unix)]
                unsafe {
                    libc::kill(*pid as i32, libc::SIGKILL);
                }
                #[cfg(windows)]
                unsafe {
                    let handle = OpenProcess(1, 0, *pid);
                    if handle != 0 {
                        windows_sys::Win32::System::Threading::TerminateProcess(handle, 1);
                        CloseHandle(handle);
                    }
                }
                cleanup_stale_files(target_session);
                closed.push(target_session.clone());
            }
        }
    }

    StdioResponse {
        id: None,
        session,
        success: failed.is_empty(),
        action: Some("close_all".to_string()),
        data: Some(json!({
            "closed": closed.len(),
            "sessions": closed,
            "failed": failed,
        })),
        error: None,
        warning: None,
        error_type: None,
    }
}

fn parse_error_type(error: &ParseError) -> &'static str {
    match error {
        ParseError::UnknownCommand { .. } => "unknown_command",
        ParseError::UnknownSubcommand { .. } => "unknown_subcommand",
        ParseError::MissingArguments { .. } => "missing_arguments",
        ParseError::InvalidValue { .. } => "invalid_value",
        ParseError::InvalidSessionName { .. } => "invalid_session_name",
    }
}

fn validate_session_configuration(flags: &Flags) -> Result<(), String> {
    if let Some(ref name) = flags.session_name {
        if !validation::is_valid_session_name(name) {
            return Err(validation::session_name_error(name));
        }
    }
    Ok(())
}

fn validate_launch_flags(flags: &Flags) -> Result<(), String> {
    if flags.cdp.is_some() && flags.provider.is_some() {
        return Err("Cannot use --cdp and -p/--provider together".to_string());
    }

    if flags.auto_connect && flags.cdp.is_some() {
        return Err("Cannot use --auto-connect and --cdp together".to_string());
    }

    if flags.auto_connect && flags.provider.is_some() {
        return Err("Cannot use --auto-connect and -p/--provider together".to_string());
    }

    if flags.provider.is_some() && !flags.extensions.is_empty() {
        return Err(
            "Cannot use --extension with -p/--provider (extensions require local browser)"
                .to_string(),
        );
    }

    if flags.cdp.is_some() && !flags.extensions.is_empty() {
        return Err(
            "Cannot use --extension with --cdp (extensions require local browser)".to_string(),
        );
    }

    Ok(())
}

fn build_daemon_options<'a>(
    flags: &'a Flags,
    proxy_server: Option<&'a str>,
    proxy_username: Option<&'a str>,
    proxy_password: Option<&'a str>,
) -> DaemonOptions<'a> {
    DaemonOptions {
        headed: flags.headed,
        debug: flags.debug,
        executable_path: flags.executable_path.as_deref(),
        extensions: &flags.extensions,
        args: flags.args.as_deref(),
        user_agent: flags.user_agent.as_deref(),
        proxy: proxy_server,
        proxy_bypass: flags.proxy_bypass.as_deref(),
        proxy_username,
        proxy_password,
        ignore_https_errors: flags.ignore_https_errors,
        allow_file_access: flags.allow_file_access,
        profile: flags.profile.as_deref(),
        state: flags.state.as_deref(),
        provider: flags.provider.as_deref(),
        device: flags.device.as_deref(),
        session_name: flags.session_name.as_deref(),
        download_path: flags.download_path.as_deref(),
        allowed_domains: flags.allowed_domains.as_deref(),
        action_policy: flags.action_policy.as_deref(),
        confirm_actions: flags.confirm_actions.as_deref(),
        engine: flags.engine.as_deref(),
        auto_connect: flags.auto_connect,
        idle_timeout: flags.idle_timeout.as_deref(),
        default_timeout: flags.default_timeout,
        cdp: flags.cdp.as_deref(),
        no_auto_dialog: flags.no_auto_dialog,
    }
}

fn warn_ignored_launch_flags(flags: &Flags) {
    let ignored_flags: Vec<&str> = [
        if flags.cli_executable_path {
            Some("--executable-path")
        } else {
            None
        },
        if flags.cli_extensions {
            Some("--extension")
        } else {
            None
        },
        if flags.cli_profile {
            Some("--profile")
        } else {
            None
        },
        if flags.cli_state {
            Some("--state")
        } else {
            None
        },
        if flags.cli_args { Some("--args") } else { None },
        if flags.cli_user_agent {
            Some("--user-agent")
        } else {
            None
        },
        if flags.cli_proxy {
            Some("--proxy")
        } else {
            None
        },
        if flags.cli_proxy_bypass {
            Some("--proxy-bypass")
        } else {
            None
        },
        flags.ignore_https_errors.then_some("--ignore-https-errors"),
        flags.cli_allow_file_access.then_some("--allow-file-access"),
        flags.cli_download_path.then_some("--download-path"),
        flags.cli_headed.then_some("--headed"),
    ]
    .into_iter()
    .flatten()
    .collect();

    if !ignored_flags.is_empty() && !flags.json {
        eprintln!(
            "{} {} ignored: daemon already running. Use 'agent-browser close' first to restart with new options.",
            color::warning_indicator(),
            ignored_flags.join(", ")
        );
    }
}

fn maybe_launch_browser(flags: &Flags, session: &str, already_running: bool) -> Result<(), String> {
    if flags.auto_connect && !already_running {
        let mut launch_cmd = json!({
            "id": gen_id(),
            "action": "launch",
            "autoConnect": true
        });

        if flags.ignore_https_errors {
            launch_cmd["ignoreHTTPSErrors"] = json!(true);
        }

        if let Some(ref cs) = flags.color_scheme {
            launch_cmd["colorScheme"] = json!(cs);
        }

        if let Some(ref dp) = flags.download_path {
            launch_cmd["downloadPath"] = json!(dp);
        }

        match send_command(launch_cmd, session) {
            Ok(resp) if resp.success => {}
            Ok(resp) => {
                return Err(resp
                    .error
                    .unwrap_or_else(|| "Auto-connect failed".to_string()));
            }
            Err(e) => return Err(e.to_string()),
        }
    }

    if let Some(ref cdp_value) = flags.cdp {
        let launch_cmd = if cdp_value.starts_with("ws://")
            || cdp_value.starts_with("wss://")
            || cdp_value.starts_with("http://")
            || cdp_value.starts_with("https://")
        {
            json!({
                "id": gen_id(),
                "action": "launch",
                "cdpUrl": cdp_value
            })
        } else {
            let cdp_port: u16 = match cdp_value.parse::<u32>() {
                Ok(0) => {
                    return Err("Invalid CDP port: port must be greater than 0".to_string());
                }
                Ok(p) if p > 65535 => {
                    return Err(format!(
                        "Invalid CDP port: {} is out of range (valid range: 1-65535)",
                        p
                    ));
                }
                Ok(p) => p as u16,
                Err(_) => {
                    return Err(format!(
                        "Invalid CDP value: '{}' is not a valid port number or URL",
                        cdp_value
                    ));
                }
            };
            json!({
                "id": gen_id(),
                "action": "launch",
                "cdpPort": cdp_port
            })
        };

        if !already_running {
            let mut launch_cmd = launch_cmd;

            if flags.ignore_https_errors {
                launch_cmd["ignoreHTTPSErrors"] = json!(true);
            }

            if let Some(ref cs) = flags.color_scheme {
                launch_cmd["colorScheme"] = json!(cs);
            }

            if let Some(ref dp) = flags.download_path {
                launch_cmd["downloadPath"] = json!(dp);
            }

            match send_command(launch_cmd, session) {
                Ok(resp) if resp.success => {}
                Ok(resp) => {
                    return Err(resp
                        .error
                        .unwrap_or_else(|| "CDP connection failed".to_string()));
                }
                Err(e) => return Err(e.to_string()),
            }
        }
    }

    if let Some(ref provider) = flags.provider {
        if !already_running {
            let mut launch_cmd = json!({
                "id": gen_id(),
                "action": "launch",
                "provider": provider
            });

            if let Some(ref cs) = flags.color_scheme {
                launch_cmd["colorScheme"] = json!(cs);
            }

            match send_command(launch_cmd, session) {
                Ok(resp) if resp.success => {}
                Ok(resp) => {
                    return Err(resp
                        .error
                        .unwrap_or_else(|| "Provider connection failed".to_string()));
                }
                Err(e) => return Err(e.to_string()),
            }
        }
    }

    if (flags.headed
        || flags.cli_headed
        || flags.executable_path.is_some()
        || flags.profile.is_some()
        || flags.state.is_some()
        || flags.proxy.is_some()
        || flags.args.is_some()
        || flags.user_agent.is_some()
        || flags.allow_file_access
        || flags.color_scheme.is_some()
        || flags.download_path.is_some()
        || flags.engine.is_some()
        || !flags.extensions.is_empty())
        && flags.cdp.is_none()
        && flags.provider.is_none()
        && !flags.auto_connect
    {
        let mut launch_cmd = json!({
            "id": gen_id(),
            "action": "launch",
            "headless": !flags.headed
        });

        let cmd_obj = launch_cmd
            .as_object_mut()
            .expect("json! macro guarantees object type");

        if let Some(ref exec_path) = flags.executable_path {
            cmd_obj.insert("executablePath".to_string(), json!(exec_path));
        }

        if let Some(ref profile_path) = flags.profile {
            cmd_obj.insert("profile".to_string(), json!(profile_path));
        }

        if let Some(ref state_path) = flags.state {
            cmd_obj.insert("storageState".to_string(), json!(state_path));
        }

        if let Some(ref proxy_str) = flags.proxy {
            let parsed = parse_proxy(proxy_str);
            let mut proxy_obj = json!({ "server": parsed.server });
            if let Some(ref username) = parsed.username {
                proxy_obj["username"] = json!(username);
            }
            if let Some(ref password) = parsed.password {
                proxy_obj["password"] = json!(password);
            }
            if let Some(ref bypass) = flags.proxy_bypass {
                proxy_obj["bypass"] = json!(bypass);
            }
            cmd_obj.insert("proxy".to_string(), proxy_obj);
        }

        if let Some(ref ua) = flags.user_agent {
            cmd_obj.insert("userAgent".to_string(), json!(ua));
        }

        if let Some(ref a) = flags.args {
            let args_vec: Vec<String> = a
                .split(&[',', '\n'][..])
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            cmd_obj.insert("args".to_string(), json!(args_vec));
        }

        if !flags.extensions.is_empty() {
            cmd_obj.insert("extensions".to_string(), json!(&flags.extensions));
        }

        if flags.ignore_https_errors {
            launch_cmd["ignoreHTTPSErrors"] = json!(true);
        }

        if flags.allow_file_access {
            launch_cmd["allowFileAccess"] = json!(true);
        }

        if let Some(ref cs) = flags.color_scheme {
            launch_cmd["colorScheme"] = json!(cs);
        }

        if let Some(ref dp) = flags.download_path {
            launch_cmd["downloadPath"] = json!(dp);
        }

        if let Some(ref domains) = flags.allowed_domains {
            launch_cmd["allowedDomains"] = json!(domains);
        }

        if let Some(ref engine) = flags.engine {
            launch_cmd["engine"] = json!(engine);
        }

        match send_command(launch_cmd, session) {
            Ok(resp) if resp.success => {}
            Ok(resp) => {
                return Err(resp
                    .error
                    .unwrap_or_else(|| "Browser launch failed".to_string()));
            }
            Err(e) => return Err(e.to_string()),
        }
    }

    Ok(())
}

fn execute_supported_command(
    flags: &Flags,
    session: &str,
    cmd: serde_json::Value,
    emit_warnings: bool,
) -> Result<(Response, Option<String>), String> {
    validate_session_configuration(flags)?;
    validate_launch_flags(flags)?;

    let (proxy_server, proxy_username, proxy_password) = if let Some(ref proxy_str) = flags.proxy {
        let parsed = parse_proxy(proxy_str);
        (Some(parsed.server), parsed.username, parsed.password)
    } else {
        (None, None, None)
    };

    let daemon_opts = build_daemon_options(
        flags,
        proxy_server.as_deref(),
        proxy_username.as_deref(),
        proxy_password.as_deref(),
    );

    let daemon_result = ensure_daemon(session, &daemon_opts).map_err(|e| e.to_string())?;
    if daemon_result.already_running && emit_warnings {
        warn_ignored_launch_flags(flags);
    }

    maybe_launch_browser(flags, session, daemon_result.already_running)?;

    let action = cmd
        .get("action")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let response = send_command(cmd, session).map_err(|e| e.to_string())?;
    Ok((response, action))
}

fn run_stdio_protocol(base_flags: &Flags) {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout().lock();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => {
                let response = StdioResponse {
                    id: None,
                    session: base_flags.session.clone(),
                    success: false,
                    action: None,
                    data: None,
                    error: Some(format!("Failed to read stdin: {}", error)),
                    warning: None,
                    error_type: Some("stdin_read_error"),
                };
                let _ = writeln!(stdout, "{}", serialize_json_value(&json!(response)));
                let _ = stdout.flush();
                break;
            }
        };

        if line.trim().is_empty() {
            continue;
        }

        let request: StdioRequest = match serde_json::from_str(&line) {
            Ok(request) => request,
            Err(error) => {
                let response = StdioResponse {
                    id: None,
                    session: base_flags.session.clone(),
                    success: false,
                    action: None,
                    data: None,
                    error: Some(format!("Invalid request JSON: {}", error)),
                    warning: None,
                    error_type: Some("invalid_request"),
                };
                let _ = writeln!(stdout, "{}", serialize_json_value(&json!(response)));
                let _ = stdout.flush();
                continue;
            }
        };

        let session = request
            .session
            .clone()
            .unwrap_or_else(|| base_flags.session.clone());

        if request.command.is_empty() {
            let response = StdioResponse {
                id: request.id,
                session,
                success: false,
                action: None,
                data: None,
                error: Some("Request must include a non-empty command array".to_string()),
                warning: None,
                error_type: Some("missing_command"),
            };
            let _ = writeln!(stdout, "{}", serialize_json_value(&json!(response)));
            let _ = stdout.flush();
            continue;
        }

        if request.command.first().map(|value| value.as_str()) == Some("baseline") {
            let parsed = match parse_baseline_command(&request.command) {
                Ok(parsed) => parsed,
                Err(error) => {
                    let response = StdioResponse {
                        id: request.id,
                        session,
                        success: false,
                        action: None,
                        data: None,
                        error: Some(error.format()),
                        warning: None,
                        error_type: Some(parse_error_type(&error)),
                    };
                    let _ = writeln!(stdout, "{}", serialize_json_value(&json!(response)));
                    let _ = stdout.flush();
                    continue;
                }
            };

            let response = match execute_baseline_command(base_flags, &session, &parsed) {
                Ok(result) => StdioResponse {
                    id: request.id,
                    session,
                    success: result.success,
                    action: Some(result.action),
                    data: Some(result.data),
                    error: result.error,
                    warning: None,
                    error_type: if result.success {
                        None
                    } else {
                        Some("baseline_mismatch")
                    },
                },
                Err(error) => StdioResponse {
                    id: request.id,
                    session,
                    success: false,
                    action: Some("baseline_error".to_string()),
                    data: None,
                    error: Some(error),
                    warning: None,
                    error_type: Some("execution_error"),
                },
            };

            let _ = writeln!(stdout, "{}", serialize_json_value(&json!(response)));
            let _ = stdout.flush();
            continue;
        }

        if matches!(
            request.command.first().map(|s| s.as_str()),
            Some("close") | Some("quit") | Some("exit")
        ) && request.command.iter().any(|arg| arg == "--all")
        {
            let response = close_all_response(session);
            let _ = writeln!(stdout, "{}", serialize_json_value(&json!(response)));
            let _ = stdout.flush();
            continue;
        }

        let parsed = match parse_command(&request.command, base_flags) {
            Ok(parsed) => parsed,
            Err(error) => {
                let response = StdioResponse {
                    id: request.id,
                    session,
                    success: false,
                    action: None,
                    data: None,
                    error: Some(error.format()),
                    warning: None,
                    error_type: Some(parse_error_type(&error)),
                };
                let _ = writeln!(stdout, "{}", serialize_json_value(&json!(response)));
                let _ = stdout.flush();
                continue;
            }
        };

        let response = match execute_supported_command(base_flags, &session, parsed, false) {
            Ok((response, action)) => StdioResponse {
                id: request.id,
                session,
                success: response.success,
                action,
                data: response.data,
                error: response.error,
                warning: response.warning,
                error_type: None,
            },
            Err(error) => StdioResponse {
                id: request.id,
                session,
                success: false,
                action: None,
                data: None,
                error: Some(error),
                warning: None,
                error_type: Some("execution_error"),
            },
        };

        let _ = writeln!(stdout, "{}", serialize_json_value(&json!(response)));
        let _ = stdout.flush();
    }
}

fn main() {
    // Rust ignores SIGPIPE by default, causing println! to panic on broken pipes.
    // Reset to SIG_DFL so the OS terminates the process cleanly instead.
    #[cfg(unix)]
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }

    // Prevent MSYS/Git Bash path translation from mangling arguments
    #[cfg(windows)]
    {
        env::set_var("MSYS_NO_PATHCONV", "1");
        env::set_var("MSYS2_ARG_CONV_EXCL", "*");
    }

    // Native daemon mode: when AGENT_BROWSER_DAEMON is set, run as the daemon process
    if env::var("AGENT_BROWSER_DAEMON").is_ok() {
        // Ignore SIGPIPE so the daemon isn't killed when the parent drops
        // the piped stderr handle after confirming the daemon is ready.
        #[cfg(unix)]
        unsafe {
            libc::signal(libc::SIGPIPE, libc::SIG_IGN);
        }
        let session = env::var("AGENT_BROWSER_SESSION").unwrap_or_else(|_| "default".to_string());
        let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
        rt.block_on(native::daemon::run_daemon(&session));
        return;
    }

    let args: Vec<String> = env::args().skip(1).collect();
    let flags = parse_flags(&args);
    let clean = clean_args(&args);

    let has_help = args.iter().any(|a| a == "--help" || a == "-h");
    let has_version = args.iter().any(|a| a == "--version" || a == "-V");

    if has_help {
        if let Some(cmd) = clean.first() {
            if print_command_help(cmd) {
                return;
            }
        }
        print_help();
        return;
    }

    if has_version {
        print_version();
        return;
    }

    if clean.is_empty() {
        print_help();
        return;
    }

    if clean.first().map(|s| s.as_str()) == Some("stdio") {
        run_stdio_protocol(&flags);
        return;
    }

    if clean.first().map(|value| value.as_str()) == Some("baseline") {
        let parsed = match parse_baseline_command(&clean) {
            Ok(parsed) => parsed,
            Err(error) => {
                if flags.json {
                    let error_type = parse_error_type(&error);
                    print_json_error_with_type(error.format(), error_type);
                } else {
                    eprintln!("{}", color::red(&error.format()));
                }
                exit(1);
            }
        };

        match execute_baseline_command(&flags, &flags.session, &parsed) {
            Ok(result) => {
                if flags.json {
                    print_json_value(json!({
                        "success": result.success,
                        "action": result.action,
                        "data": result.data,
                        "error": result.error,
                    }));
                } else {
                    match result.action.as_str() {
                        "baseline_capture" => {
                            println!(
                                "{} Captured baseline '{}' at {}",
                                color::success_indicator(),
                                parsed.name,
                                color::green(
                                    result
                                        .data
                                        .get("baselinePath")
                                        .and_then(|value| value.as_str())
                                        .unwrap_or("")
                                )
                            );
                        }
                        "baseline_compare" => {
                            let mismatch = result
                                .data
                                .get("mismatchPercentage")
                                .and_then(|value| value.as_f64())
                                .unwrap_or(0.0);
                            let baseline_path = result
                                .data
                                .get("baselinePath")
                                .and_then(|value| value.as_str())
                                .unwrap_or("");
                            let current_path = result
                                .data
                                .get("currentPath")
                                .and_then(|value| value.as_str())
                                .unwrap_or("");
                            let diff_path = result
                                .data
                                .get("diffPath")
                                .and_then(|value| value.as_str())
                                .filter(|value| !value.is_empty());

                            if result.success {
                                println!(
                                    "{} Baseline '{}' matched (0% difference)",
                                    color::success_indicator(),
                                    parsed.name
                                );
                            } else {
                                println!(
                                    "{} Baseline '{}' changed ({:.2}% mismatch)",
                                    color::error_indicator(),
                                    parsed.name,
                                    mismatch
                                );
                            }
                            println!("  Baseline: {}", color::green(baseline_path));
                            println!("  Current:  {}", color::green(current_path));
                            if let Some(diff_path) = diff_path {
                                println!("  Diff:     {}", color::green(diff_path));
                            }
                        }
                        "baseline_approve" => {
                            println!(
                                "{} Approved baseline '{}' at {}",
                                color::success_indicator(),
                                parsed.name,
                                color::green(
                                    result
                                        .data
                                        .get("baselinePath")
                                        .and_then(|value| value.as_str())
                                        .unwrap_or("")
                                )
                            );
                        }
                        _ => {
                            println!("{}", color::success_indicator());
                        }
                    }
                }

                if !result.success {
                    exit(1);
                }
                return;
            }
            Err(error) => {
                if flags.json {
                    print_json_error(error);
                } else {
                    eprintln!("{} {}", color::error_indicator(), error);
                }
                exit(1);
            }
        }
    }

    // Handle close --all: close all active sessions
    if matches!(
        clean.first().map(|s| s.as_str()),
        Some("close") | Some("quit") | Some("exit")
    ) && clean.iter().any(|a| a == "--all")
    {
        run_close_all(&flags);
        return;
    }

    let mut cmd = match parse_command(&clean, &flags) {
        Ok(c) => c,
        Err(e) => {
            if flags.json {
                let error_type = parse_error_type(&e);
                print_json_error_with_type(e.format(), error_type);
            } else {
                eprintln!("{}", color::red(&e.format()));
            }
            exit(1);
        }
    };

    // Handle --password-stdin for auth save
    if cmd.get("action").and_then(|v| v.as_str()) == Some("auth_save") {
        if cmd.get("password").is_some() {
            eprintln!(
                "{} Passwords on the command line may be visible in process listings and shell history. Use --password-stdin instead.",
                color::warning_indicator()
            );
        }
        if cmd
            .get("passwordStdin")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            let mut pass = String::new();
            if std::io::stdin().read_line(&mut pass).is_err() || pass.is_empty() {
                eprintln!(
                    "{} Failed to read password from stdin",
                    color::error_indicator()
                );
                exit(1);
            }
            let pass = pass.trim_end_matches('\n').trim_end_matches('\r');
            if pass.is_empty() {
                eprintln!("{} Password from stdin is empty", color::error_indicator());
                exit(1);
            }
            cmd["password"] = json!(pass);
            cmd.as_object_mut().unwrap().remove("passwordStdin");
        }
    }

    let output_opts = OutputOptions::from_flags(&flags);

    match execute_supported_command(&flags, &flags.session, cmd.clone(), true) {
        Ok(resp) => {
            let (resp, action) = resp;
            let success = resp.success;
            // Handle interactive confirmation
            if flags.confirm_interactive {
                if let Some(data) = &resp.data {
                    if data
                        .get("confirmation_required")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false)
                    {
                        let desc = data
                            .get("description")
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown action");
                        let category = data.get("category").and_then(|v| v.as_str()).unwrap_or("");
                        let cid = data
                            .get("confirmation_id")
                            .and_then(|v| v.as_str())
                            .unwrap_or("");

                        eprintln!("[agent-browser] Action requires confirmation:");
                        eprintln!("  {}: {}", category, desc);
                        eprint!("  Allow? [y/N]: ");

                        let mut input = String::new();
                        let approved = if std::io::IsTerminal::is_terminal(&std::io::stdin()) {
                            std::io::stdin().read_line(&mut input).is_ok()
                                && matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
                        } else {
                            false
                        };

                        let confirm_cmd = if approved {
                            json!({ "id": gen_id(), "action": "confirm", "confirmationId": cid })
                        } else {
                            json!({ "id": gen_id(), "action": "deny", "confirmationId": cid })
                        };

                        match send_command(confirm_cmd, &flags.session) {
                            Ok(r) => {
                                if !approved {
                                    eprintln!("{} Action denied", color::error_indicator());
                                    exit(1);
                                }
                                print_response_with_opts(&r, None, &output_opts);
                            }
                            Err(e) => {
                                eprintln!("{} {}", color::error_indicator(), e);
                                exit(1);
                            }
                        }
                        return;
                    }
                }
            }
            print_response_with_opts(&resp, action.as_deref(), &output_opts);
            if !success {
                exit(1);
            }
        }
        Err(e) => {
            if flags.json {
                print_json_error(e);
            } else {
                eprintln!("{} {}", color::error_indicator(), e);
            }
            exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_proxy_simple() {
        let result = parse_proxy("http://proxy.com:8080");
        assert_eq!(result.server, "http://proxy.com:8080");
        assert!(result.username.is_none());
        assert!(result.password.is_none());
    }

    #[test]
    fn test_parse_proxy_with_auth() {
        let result = parse_proxy("http://user:pass@proxy.com:8080");
        assert_eq!(result.server, "http://proxy.com:8080");
        assert_eq!(result.username.as_deref(), Some("user"));
        assert_eq!(result.password.as_deref(), Some("pass"));
    }

    #[test]
    fn test_parse_proxy_username_only() {
        let result = parse_proxy("http://user@proxy.com:8080");
        assert_eq!(result.server, "http://proxy.com:8080");
        assert_eq!(result.username.as_deref(), Some("user"));
        assert!(result.password.is_none());
    }

    #[test]
    fn test_parse_proxy_no_protocol() {
        let result = parse_proxy("proxy.com:8080");
        assert_eq!(result.server, "proxy.com:8080");
        assert!(result.username.is_none());
    }

    #[test]
    fn test_parse_proxy_socks5() {
        let result = parse_proxy("socks5://proxy.com:1080");
        assert_eq!(result.server, "socks5://proxy.com:1080");
        assert!(result.username.is_none());
    }

    #[test]
    fn test_parse_proxy_socks5_with_auth() {
        let result = parse_proxy("socks5://admin:secret@proxy.com:1080");
        assert_eq!(result.server, "socks5://proxy.com:1080");
        assert_eq!(result.username.as_deref(), Some("admin"));
        assert_eq!(result.password.as_deref(), Some("secret"));
    }

    #[test]
    fn test_parse_proxy_complex_password() {
        let result = parse_proxy("http://user:p@ss:w0rd@proxy.com:8080");
        assert_eq!(result.server, "http://proxy.com:8080");
        assert_eq!(result.username.as_deref(), Some("user"));
        assert_eq!(result.password.as_deref(), Some("p@ss:w0rd"));
    }

    #[test]
    fn test_serialize_json_value_escapes_control_characters() {
        let payload = serialize_json_value(&json!({
            "success": false,
            "error": "Daemon process exited during startup:\nline \"quoted\"\u{001b}[2mansi\u{001b}[22m",
        }));

        let parsed: serde_json::Value = serde_json::from_str(&payload).unwrap();
        assert_eq!(parsed["success"], false);
        assert_eq!(
            parsed["error"],
            "Daemon process exited during startup:\nline \"quoted\"\u{001b}[2mansi\u{001b}[22m"
        );
    }

    #[test]
    fn test_slugify_baseline_name() {
        assert_eq!(
            slugify_baseline_name("Poro Shell Review"),
            Some("poro-shell-review".to_string())
        );
        assert_eq!(
            slugify_baseline_name("sidebar/main@v2"),
            Some("sidebar-main-v2".to_string())
        );
    }

    #[test]
    fn test_parse_baseline_compare_command() {
        let command = parse_baseline_command(&[
            "baseline".to_string(),
            "compare".to_string(),
            "poro-shell".to_string(),
            "--selector".to_string(),
            "main".to_string(),
            "--threshold".to_string(),
            "0.05".to_string(),
            "--full".to_string(),
        ])
        .expect("baseline compare should parse");

        assert!(matches!(command.mode, BaselineMode::Compare));
        assert_eq!(command.name, "poro-shell");
        assert_eq!(command.slug, "poro-shell");
        assert_eq!(command.selector.as_deref(), Some("main"));
        assert!(command.full_page);
        assert_eq!(command.threshold, Some(0.05));
    }

    #[test]
    fn test_parse_baseline_unknown_subcommand() {
        let error = parse_baseline_command(&[
            "baseline".to_string(),
            "ship".to_string(),
            "poro-shell".to_string(),
        ])
        .expect_err("unknown baseline subcommand should fail");

        assert!(matches!(error, ParseError::UnknownSubcommand { .. }));
    }
}
