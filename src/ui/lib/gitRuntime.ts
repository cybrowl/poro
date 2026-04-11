import { isDesktopEnvironment } from "$lib/desktop";

export interface WorkspaceGitFile {
  path: string;
  summary: string;
  stagedStatus: string | null;
  unstagedStatus: string | null;
  additions: number;
  deletions: number;
}

export interface WorkspaceGitState {
  workspacePath: string;
  isGitRepo: boolean;
  branch: string;
  clean: boolean;
  summary: string;
  stagedCount: number;
  unstagedCount: number;
  untrackedCount: number;
  changedFiles: WorkspaceGitFile[];
}

async function invokeCommand<T>(command: string, args: Record<string, unknown>) {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<T>(command, args);
}

export async function loadWorkspaceGitState(
  workspacePath: string
): Promise<WorkspaceGitState | null> {
  if (!isDesktopEnvironment()) {
    return null;
  }

  return invokeCommand<WorkspaceGitState>("load_workspace_git_state", { workspacePath });
}

export async function loadWorkspaceGitDiff(
  workspacePath: string,
  filePath: string
): Promise<string> {
  if (!isDesktopEnvironment()) {
    return "";
  }

  return invokeCommand<string>("load_workspace_git_diff", {
    workspacePath,
    filePath,
  });
}
