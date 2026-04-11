import type { PermissionMode, WorkspaceRecord } from "$lib/mockDesktopData";
import { workspaces } from "$lib/mockDesktopData";

export interface DesktopSettings {
  backendPath: string;
  recentWorkspaces: string[];
  selectedProviderId: string;
  selectedModel: string;
  selectedPermission: PermissionMode;
}

export const defaultDesktopSettings: DesktopSettings = {
  backendPath: "harness-server",
  recentWorkspaces: workspaces.map((workspace) => workspace.path),
  selectedProviderId: "local",
  selectedModel: "gemma4:e2b",
  selectedPermission: "workspace-write",
};

declare global {
  interface Window {
    __TAURI__?: unknown;
    __TAURI_INTERNALS__?: unknown;
  }
}

export function isDesktopEnvironment(): boolean {
  if (typeof window === "undefined") {
    return false;
  }

  return "__TAURI_INTERNALS__" in window || "__TAURI__" in window;
}

export function cloneInitialWorkspaces(): WorkspaceRecord[] {
  return structuredClone(workspaces);
}

export function createWorkspaceRecord(path: string): WorkspaceRecord {
  const segments = path.split("/").filter(Boolean);
  const name = segments.at(-1) ?? "workspace";
  const id = path
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "");

  return {
    id,
    name,
    path,
    branch: "detecting…",
    status: "Ready",
    lastOpened: "Just now",
    summary:
      "Opened from the native folder picker. This workspace is ready for local Ollama + Gemma runtime wiring.",
    accent: "soft-ivory",
    sessions: [
      {
        id: `${id}-new-session`,
        title: "New desktop session",
        branch: "detecting…",
        status: "Ready",
        updatedAt: "Created just now",
        model: "gemma4:e2b",
        provider: "Ollama Local",
        permission: "workspace-write",
        tokenUsage: "Local session",
        cost: "On-device",
        cwd: path,
        goal: "Set up a real workspace selected from the native desktop picker.",
        draft: "",
        transcript: [
          {
            id: `${id}-system`,
            role: "system",
            title: "Workspace added",
            body: "This workspace was opened through the native desktop picker and stored in local settings.",
            meta: "Desktop bridge",
          },
          {
            id: `${id}-assistant`,
            role: "assistant",
            title: "Poro",
            body: "The shell is ready. The next step is connecting this workspace to a real local runtime session.",
            meta: "Phase 2",
          },
        ],
        activity: [
          {
            id: `${id}-activity`,
            label: "Native workspace pick",
            status: "complete",
            summary: "Folder selected from the Tauri dialog and inserted into recent workspaces.",
            timestamp: "Just now",
          },
        ],
        changes: [
          {
            path: "No file changes yet",
            summary: "Diff review will populate once the runtime layer is connected.",
            additions: 0,
            deletions: 0,
          },
        ],
      },
    ],
  };
}

export async function loadDesktopSettings(): Promise<DesktopSettings> {
  if (!isDesktopEnvironment()) {
    return defaultDesktopSettings;
  }

  try {
    const { invoke } = await import("@tauri-apps/api/core");
    const settings = await invoke<DesktopSettings>("load_desktop_settings");
    return {
      ...defaultDesktopSettings,
      ...settings,
      recentWorkspaces:
        settings.recentWorkspaces?.length ? settings.recentWorkspaces : defaultDesktopSettings.recentWorkspaces,
    };
  } catch (error) {
    console.error("Failed to load desktop settings:", error);
    return defaultDesktopSettings;
  }
}

export async function saveDesktopSettings(settings: DesktopSettings): Promise<void> {
  if (!isDesktopEnvironment()) {
    return;
  }

  try {
    const { invoke } = await import("@tauri-apps/api/core");
    await invoke("save_desktop_settings", { settings });
  } catch (error) {
    console.error("Failed to save desktop settings:", error);
  }
}

export async function pickWorkspaceDirectory(): Promise<string | null> {
  if (!isDesktopEnvironment()) {
    return null;
  }

  try {
    const { open } = await import("@tauri-apps/plugin-dialog");
    const selection = await open({
      directory: true,
      multiple: false,
      title: "Choose a workspace for Poro",
    });

    return typeof selection === "string" ? selection : null;
  } catch (error) {
    console.error("Failed to open workspace picker:", error);
    return null;
  }
}
