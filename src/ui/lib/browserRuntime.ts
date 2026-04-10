import { isDesktopEnvironment } from "$lib/desktop";

export interface BrowserLaunchRequest {
  browserPath?: string | null;
  session?: string | null;
  headless?: boolean | null;
}

export interface BrowserRuntimeLaunch {
  runtimeId: string;
  session: string;
  browserPath: string;
  headless: boolean;
  message: string;
}

export interface BrowserCommandRequest {
  runtimeId: string;
  command: string[];
  session?: string | null;
  requestId?: string | null;
}

export interface BrowserCommandResult {
  id: string | null;
  session: string;
  success: boolean;
  action: string | null;
  data: Record<string, unknown> | null;
  error: string | null;
  warning: string | null;
  errorType: string | null;
}

export type BrowserRuntimeEvent =
  | {
      type: "started";
      launch: BrowserRuntimeLaunch;
    }
  | {
      type: "output";
      runtimeId: string;
      line: string;
      stream: string;
    }
  | {
      type: "response";
      runtimeId: string;
      response: BrowserCommandResult;
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

async function invokeCommand<T>(command: string, args: Record<string, unknown>) {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<T>(command, args);
}

export async function startBrowserRuntime(
  request: BrowserLaunchRequest = {}
): Promise<BrowserRuntimeLaunch> {
  if (!isDesktopEnvironment()) {
    throw new Error("Browser runtime launch is available only inside the desktop build.");
  }

  return invokeCommand<BrowserRuntimeLaunch>("start_browser_runtime", { request });
}

export async function sendBrowserCommand(
  request: BrowserCommandRequest
): Promise<BrowserCommandResult> {
  if (!isDesktopEnvironment()) {
    throw new Error("Browser commands are available only inside the desktop build.");
  }

  return invokeCommand<BrowserCommandResult>("send_browser_command", { request });
}

export async function stopBrowserRuntime(runtimeId: string): Promise<void> {
  if (!isDesktopEnvironment()) {
    return;
  }

  await invokeCommand("stop_browser_runtime", { runtimeId });
}

export async function listenToBrowserRuntimeEvents(
  handler: (event: BrowserRuntimeEvent) => void
): Promise<() => void> {
  if (!isDesktopEnvironment()) {
    return () => {};
  }

  const { listen } = await import("@tauri-apps/api/event");
  return listen<BrowserRuntimeEvent>("browser-runtime", (event) => {
    handler(event.payload);
  });
}
