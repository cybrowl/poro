export type ActivityStatus = "active" | "complete" | "queued";
export type MessageRole = "system" | "user" | "assistant" | "tool";
export type PermissionMode = "read-only" | "workspace-write" | "danger-full-access";

export interface ActivityItem {
  id: string;
  label: string;
  status: ActivityStatus;
  summary: string;
  timestamp: string;
}

export interface FileChange {
  path: string;
  summary: string;
  additions: number;
  deletions: number;
}

export interface TranscriptMessage {
  id: string;
  role: MessageRole;
  title: string;
  body: string;
  meta: string;
}

export interface SessionRecord {
  id: string;
  sessionPath?: string;
  runtimeId?: string | null;
  source?: "mock" | "claw";
  title: string;
  branch: string;
  status: "Live" | "Paused" | "Ready";
  updatedAt: string;
  model: string;
  provider: string;
  permission: PermissionMode;
  tokenUsage: string;
  cost: string;
  cwd: string;
  goal: string;
  draft: string;
  transcript: TranscriptMessage[];
  activity: ActivityItem[];
  changes: FileChange[];
}

export interface WorkspaceRecord {
  id: string;
  name: string;
  path: string;
  branch: string;
  status: string;
  lastOpened: string;
  summary: string;
  accent: string;
  sessions: SessionRecord[];
}

export interface ProviderRecord {
  id: string;
  label: string;
  endpoint: string;
  status: string;
}

export const providers: ProviderRecord[] = [
  {
    id: "local",
    label: "Ollama Local",
    endpoint: "127.0.0.1:11434 • Gemma 4",
    status: "Recommended",
  },
  {
    id: "anthropic",
    label: "Anthropic",
    endpoint: "api.anthropic.com",
    status: "Optional",
  },
  {
    id: "grok",
    label: "xAI / Grok",
    endpoint: "api.x.ai • Grok 4.20",
    status: "Optional",
  },
];

export const providerModelOptions: Record<string, string[]> = {
  local: ["gemma4:e2b", "gemma4:e4b"],
  grok: ["grok-4.20-0309-reasoning", "grok-4-1-fast-reasoning", "grok-code-fast-1"],
  anthropic: ["claude-sonnet-4-6"],
};

export const providerDefaultModel: Record<string, string> = {
  local: "gemma4:e2b",
  grok: "grok-4.20-0309-reasoning",
  anthropic: "claude-sonnet-4-6",
};

export const modelOptions = Object.values(providerModelOptions).flat();

export const permissionModes: PermissionMode[] = [
  "read-only",
  "workspace-write",
  "danger-full-access",
];

export const workspaces: WorkspaceRecord[] = [
  {
    id: "poro",
    name: "poro",
    path: "/Users/name/Repos/poro",
    branch: "codex/phase-1-shell",
    status: "Active session",
    lastOpened: "2 minutes ago",
    summary: "Designing the UI shell and the first-run workflow for the desktop app.",
    accent: "marigold",
    sessions: [
      {
        id: "phase-1-shell",
        title: "Phase 1 shell and mock state",
        branch: "codex/phase-1-shell",
        status: "Live",
        updatedAt: "Updated just now",
        model: "gemma4:e2b",
        provider: "Ollama Local",
        permission: "workspace-write",
        tokenUsage: "Local session",
        cost: "On-device",
        cwd: "/Users/name/Repos/poro",
        goal: "Turn the placeholder page into a credible desktop coding workspace.",
        draft:
          "Build the session shell with a workspace picker, activity timeline, diff panel, and settings sheet. Keep the interface calm and premium.",
        transcript: [
          {
            id: "poro-system",
            role: "system",
            title: "Workspace loaded",
            body: "Mock backend diagnostics passed. The repo is ready for a local-first desktop session.",
            meta: "Backend health • Ready",
          },
          {
            id: "poro-user",
            role: "user",
            title: "You",
            body: "Build Phase 1 of the desktop shell and make it feel like a real product before runtime integration.",
            meta: "Prompt",
          },
          {
            id: "poro-assistant",
            role: "assistant",
            title: "Poro",
            body: "I’m splitting the screen into four clear surfaces: workspace rail, session canvas, tool activity, and review. That gives us a strong base for Tauri and `claw-code` later.",
            meta: "Plan",
          },
          {
            id: "poro-tool",
            role: "tool",
            title: "Tool event",
            body: "Mock diff prepared for 3 changed files. Review panel updated with additions, deletions, and summaries.",
            meta: "ui-review",
          },
        ],
        activity: [
          {
            id: "poro-activity-1",
            label: "Workspace selected",
            status: "complete",
            summary: "Recent workspace reopened from local history with previous branch context.",
            timestamp: "12:41 PM",
          },
          {
            id: "poro-activity-2",
            label: "Session resumed",
            status: "complete",
            summary: "Recovered the last working context and restored the main shell state.",
            timestamp: "12:42 PM",
          },
          {
            id: "poro-activity-3",
            label: "UI shell layout",
            status: "active",
            summary: "Composing mock transcript, activity, and review surfaces for the Phase 1 demo.",
            timestamp: "12:44 PM",
          },
          {
            id: "poro-activity-4",
            label: "Tauri bridge",
            status: "queued",
            summary: "Queued for Phase 2 once the shell and interaction model feel right.",
            timestamp: "Next",
          },
        ],
        changes: [
          {
            path: "src/ui/routes/+page.svelte",
            summary: "Replaced the placeholder layout with a multi-surface desktop shell.",
            additions: 184,
            deletions: 32,
          },
          {
            path: "src/ui/components/desktop/SettingsSheet.svelte",
            summary: "Added the first-run settings sheet for provider and backend preferences.",
            additions: 118,
            deletions: 0,
          },
          {
            path: "src/ui/lib/mockDesktopData.ts",
            summary: "Defined reusable mock workspace, session, transcript, and activity data.",
            additions: 196,
            deletions: 0,
          },
        ],
      },
      {
        id: "notes-cleanup",
        title: "Product note rewrite",
        branch: "codex/product-reset",
        status: "Paused",
        updatedAt: "25 minutes ago",
        model: "gemma4:e2b",
        provider: "Ollama Local",
        permission: "read-only",
        tokenUsage: "Local session",
        cost: "On-device",
        cwd: "/Users/name/Repos/poro",
        goal: "Align the repo, roadmap, and business model with the desktop pivot.",
        draft: "Review the planning docs and tighten the launch story.",
        transcript: [
          {
            id: "notes-system",
            role: "system",
            title: "Notes updated",
            body: "Spec, business, integration, and MVP planning notes now reflect the desktop app direction.",
            meta: "Completed",
          },
        ],
        activity: [
          {
            id: "notes-activity",
            label: "Note rewrite",
            status: "complete",
            summary: "Legacy ICP and payment framing removed from planning docs.",
            timestamp: "Earlier today",
          },
        ],
        changes: [
          {
            path: "notes/spec.md",
            summary: "Rewrote the product spec around the desktop workspace direction.",
            additions: 120,
            deletions: 95,
          },
        ],
      },
    ],
  },
  {
    id: "claw-code",
    name: "claw-code",
    path: "/Users/name/Repos/claw-code",
    branch: "main",
    status: "Paused",
    lastOpened: "14 minutes ago",
    summary: "Reference runtime for permission modes, sessions, and local provider support.",
    accent: "misty-green",
    sessions: [
      {
        id: "runtime-boundary",
        title: "Adapter and process boundary",
        branch: "main",
        status: "Paused",
        updatedAt: "Updated 14 minutes ago",
        model: "gemma4:e2b",
        provider: "Ollama Local",
        permission: "read-only",
        tokenUsage: "Local session",
        cost: "On-device",
        cwd: "/Users/name/Repos/claw-code",
        goal: "Understand the cleanest interface between Poro and the runtime.",
        draft:
          "List the minimum Tauri commands needed to supervise a local session without leaking backend details into the UI.",
        transcript: [
          {
            id: "claw-system",
            role: "system",
            title: "Runtime survey",
            body: "Structured output and local session persistence look like the best anchor points for the UI bridge.",
            meta: "Analysis",
          },
          {
            id: "claw-assistant",
            role: "assistant",
            title: "Poro",
            body: "The healthiest contract is thin: launch session, resume session, stream events, and fetch metadata.",
            meta: "Architecture",
          },
        ],
        activity: [
          {
            id: "claw-activity-1",
            label: "CLI capability audit",
            status: "complete",
            summary: "Confirmed sessions, permissions, and provider routing already exist in the runtime.",
            timestamp: "11:58 AM",
          },
          {
            id: "claw-activity-2",
            label: "UI adapter notes",
            status: "queued",
            summary: "Ready to turn into Tauri commands during Phase 2.",
            timestamp: "Queued",
          },
        ],
        changes: [
          {
            path: "notes/integration.md",
            summary: "Captured the runtime boundary and first integration targets.",
            additions: 74,
            deletions: 0,
          },
        ],
      },
    ],
  },
  {
    id: "job-raptor",
    name: "job_raptor",
    path: "/Users/name/Repos/job_raptor",
    branch: "main",
    status: "Ready",
    lastOpened: "Yesterday",
    summary: "Desktop reference app for the Tauri, packaging, and local storage setup.",
    accent: "soft-ivory",
    sessions: [
      {
        id: "desktop-reference",
        title: "Tauri shell reference",
        branch: "main",
        status: "Ready",
        updatedAt: "Updated yesterday",
        model: "deepseek-r1",
        provider: "Local Runtime",
        permission: "read-only",
        tokenUsage: "9k / 64k",
        cost: "$0.00",
        cwd: "/Users/name/Repos/job_raptor",
        goal: "Borrow the right desktop shell ideas without dragging in unrelated product logic.",
        draft:
          "Use `job_raptor` as the structural reference for Tauri setup, not as a visual template.",
        transcript: [
          {
            id: "raptor-system",
            role: "system",
            title: "Reference app",
            body: "The Tauri layout, plugins, and config shape are viable starting points for Phase 2.",
            meta: "Reference",
          },
        ],
        activity: [
          {
            id: "raptor-activity",
            label: "Desktop pattern review",
            status: "complete",
            summary: "Confirmed the right packaging and config path for a local-first desktop app.",
            timestamp: "Yesterday",
          },
        ],
        changes: [
          {
            path: "tauri/tauri.conf.json",
            summary: "Reference only. No changes planned in this repo yet.",
            additions: 0,
            deletions: 0,
          },
        ],
      },
    ],
  },
];
