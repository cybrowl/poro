import { isDesktopEnvironment } from "$lib/desktop";
import type { HarnessMissionState } from "$lib/mockDesktopData";

export interface BackendHealth {
  requestedPath: string;
  resolvedPath: string | null;
  exists: boolean;
  runnable: boolean;
  version: string | null;
  status: string;
  message: string;
  sessionsDirectory: string | null;
  sessionCount: number;
  localRuntime: LocalRuntimeHealth | null;
}

export interface LocalRuntimeHealth {
  reachable: boolean;
  version: string | null;
  hasSelectedModel: boolean;
  availableModels: string[];
  message: string;
}

export interface ClawSessionSummary {
  id: string;
  path: string;
  modifiedAt: number;
  modifiedLabel: string;
  messageCount: number;
  preview: string;
}

export interface ClawUsage {
  inputTokens: number;
  outputTokens: number;
  cacheCreationInputTokens: number;
  cacheReadInputTokens: number;
  totalTokens: number;
}

export interface ClawTranscriptMessage {
  id: string;
  role: string;
  title: string;
  body: string;
  meta: string;
}

export interface ClawActivityItem {
  id: string;
  label: string;
  status: string;
  summary: string;
  timestamp: string;
}

export interface ClawFileChange {
  path: string;
  summary: string;
  additions: number;
  deletions: number;
}

export interface ClawSessionSnapshot {
  id: string;
  path: string;
  workspacePath: string;
  modifiedAt: number;
  modifiedLabel: string;
  messageCount: number;
  preview: string;
  mission: HarnessMissionState | null;
  transcript: ClawTranscriptMessage[];
  activity: ClawActivityItem[];
  changes: ClawFileChange[];
  latestUsage: ClawUsage | null;
}

export interface LaunchRuntimeRequest {
  backendPath: string;
  workspacePath: string;
  providerId: string;
  model: string;
  permissionMode: string;
  resumeSessionPath?: string | null;
}

export interface RuntimeLaunch {
  runtimeId: string;
  workspacePath: string;
  providerId: string;
  model: string;
  permissionMode: string;
  sessionId: string | null;
  sessionPath: string | null;
  resumed: boolean;
  message: string;
}

export type RuntimeEvent =
  | {
      type: "started";
      launch: RuntimeLaunch;
    }
  | {
      type: "output";
      runtimeId: string;
      line: string;
      stream: string;
      timestamp: string;
    }
  | {
      type: "sessionAttached";
      runtimeId: string;
      sessionId: string;
      sessionPath: string;
      resumed: boolean;
    }
  | {
      type: "snapshot";
      runtimeId: string;
      snapshot: ClawSessionSnapshot;
    }
  | {
      type: "turnStarted";
      runtimeId: string;
      inputPreview: string;
    }
  | {
      type: "turnFinished";
      runtimeId: string;
      success: boolean;
      durationMs: number;
    }
  | {
      type: "stopped";
      runtimeId: string;
      code: number | null;
      message: string;
    }
  | {
      type: "error";
      runtimeId: string;
      message: string;
    };

const browserHealth: BackendHealth = {
  requestedPath: "harness-server",
  resolvedPath: null,
  exists: false,
  runnable: false,
  version: null,
  status: "preview",
  message: "Desktop runtime commands are available only inside the Tauri build.",
  sessionsDirectory: null,
  sessionCount: 0,
  localRuntime: null,
};

async function invokeCommand<T>(command: string, args: Record<string, unknown>) {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<T>(command, args);
}

export async function checkClawBackend(
  backendPath: string,
  workspacePath?: string,
  providerId?: string,
  selectedModel?: string
): Promise<BackendHealth> {
  if (!isDesktopEnvironment()) {
    return {
      ...browserHealth,
      requestedPath: backendPath,
      sessionsDirectory: workspacePath ? `${workspacePath}/desktop-session-store` : null,
    };
  }

  return invokeCommand<BackendHealth>("check_claw_backend", {
    backendPath,
    workspacePath: workspacePath ?? null,
    providerId: providerId ?? null,
    selectedModel: selectedModel ?? null,
  });
}

export async function listClawSessions(workspacePath: string): Promise<ClawSessionSummary[]> {
  if (!isDesktopEnvironment()) {
    return [];
  }

  return invokeCommand<ClawSessionSummary[]>("list_claw_sessions", { workspacePath });
}

export async function loadClawSession(
  workspacePath: string,
  sessionPath: string
): Promise<ClawSessionSnapshot> {
  return invokeCommand<ClawSessionSnapshot>("load_claw_session", {
    workspacePath,
    sessionPath,
  });
}

export async function deleteClawSession(
  workspacePath: string,
  sessionPath: string
): Promise<void> {
  if (!isDesktopEnvironment()) {
    return;
  }

  await invokeCommand("delete_claw_session", {
    workspacePath,
    sessionPath,
  });
}

export async function startClawRuntime(
  request: LaunchRuntimeRequest
): Promise<RuntimeLaunch> {
  return invokeCommand<RuntimeLaunch>("start_claw_runtime", { request });
}

export async function sendClawInput(
  runtimeId: string,
  input: string
): Promise<void> {
  if (!isDesktopEnvironment()) {
    return;
  }

  await invokeCommand("send_claw_input", { runtimeId, input });
}

export async function stopClawRuntime(runtimeId: string): Promise<void> {
  if (!isDesktopEnvironment()) {
    return;
  }

  await invokeCommand("stop_claw_runtime", { runtimeId });
}

export async function listenToClawRuntimeEvents(
  handler: (event: RuntimeEvent) => void
): Promise<() => void> {
  if (!isDesktopEnvironment()) {
    return () => {};
  }

  const { listen } = await import("@tauri-apps/api/event");
  return listen<RuntimeEvent>("claw-runtime", (event) => {
    handler(event.payload);
  });
}
