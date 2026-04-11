<script lang="ts">
  import TranscriptPanel from "$components/desktop/TranscriptPanel.svelte";
  import { cloneInitialWorkspaces } from "$lib/desktop";
  import type { WorkspaceGitState } from "$lib/gitRuntime";
  import {
    permissionModes,
    providerModelOptions,
    type ActivityItem,
    type PermissionMode,
    type SessionRecord,
  } from "$lib/mockDesktopData";

  const workspace = cloneInitialWorkspaces()[0];
  const baseSession = workspace.sessions[0];

  const session: SessionRecord = {
    ...baseSession,
    title: "Browser debug session",
    status: "Live",
    updatedAt: "Just now",
    model: "gemma4:e2b",
    provider: "Ollama Local",
    permission: "workspace-write",
    cwd: "/Users/cyberowl/Repos/poro",
    goal: "Tune the session composer and verify the browser-debug route renders the same desktop surface reliably.",
    transcript: [
      {
        id: "debug-user-1",
        role: "user",
        title: "You",
        body: "Update the composer so it feels more like a document and less like an input box.",
        meta: "Prompt",
      },
      {
        id: "debug-tool-1",
        role: "tool",
        title: "Tool `read_file`",
        body: "Opened src/ui/components/desktop/DocumentComposer.svelte and inspected the current hybrid input layer.",
        meta: "Tool finished",
      },
      {
        id: "debug-tool-2",
        role: "tool",
        title: "Tool `apply_patch`",
        body: "Adjusted the split, scrollbar treatment, and pretext-backed document composer.",
        meta: "Tool finished",
      },
      {
        id: "debug-assistant-1",
        role: "assistant",
        title: "Poro",
        body: "I pushed the first pass. The composer is now using `pretext` for visible layout, but the cursor feel still needs refinement. The next step is a browser-debug route plus native screenshot verification so we can inspect the real rendered surface instead of guessing.",
        meta: "Working summary",
      },
    ],
    activity: [
      {
        id: "debug-activity-1",
        label: "Thinking",
        status: "complete",
        summary: "Mapped the current session surface and identified the browser route gap.",
        timestamp: "Now",
      },
      {
        id: "debug-activity-2",
        label: "Tool • apply_patch",
        status: "complete",
        summary: "Updated the session shell and composer structure for browser-debug use.",
        timestamp: "Now",
      },
      {
        id: "debug-activity-3",
        label: "Verification running",
        status: "active",
        summary: "Preparing a deterministic browser route for screenshot and interaction checks.",
        timestamp: "Now",
      },
    ],
  };

  const gitState: WorkspaceGitState = {
    workspacePath: "/Users/cyberowl/Repos/poro",
    isGitRepo: true,
    branch: "codex/browser-debug-route",
    clean: false,
    summary: "2 modified files, 1 new route",
    stagedCount: 0,
    unstagedCount: 3,
    untrackedCount: 1,
    changedFiles: [
      {
        path: "src/ui/routes/debug/browser/+page.svelte",
        summary: "Adds a deterministic browser-debug session surface.",
        stagedStatus: null,
        unstagedStatus: "A",
        additions: 122,
        deletions: 0,
      },
      {
        path: "src/ui/components/desktop/DocumentComposer.svelte",
        summary: "Keeps pretext rendering while preserving native input semantics.",
        stagedStatus: null,
        unstagedStatus: "M",
        additions: 34,
        deletions: 12,
      },
      {
        path: "notes/plan.md",
        summary: "Adds browser-debug and native screenshot verification follow-ups.",
        stagedStatus: null,
        unstagedStatus: "M",
        additions: 14,
        deletions: 0,
      },
    ],
  };

  const runtimeActivity: ActivityItem[] = [
    {
      id: "debug-live-1",
      label: "Thinking",
      status: "active",
      summary: "Checking the browser-visible route before making more visual claims.",
      timestamp: "Now",
    },
    {
      id: "debug-live-2",
      label: "Tool • browser_snapshot",
      status: "queued",
      summary: "Waiting to capture the fresh screenshot.",
      timestamp: "Now",
    },
  ];

  const browserActivity: ActivityItem[] = [
    {
      id: "debug-browser-1",
      label: "Browser open",
      status: "complete",
      summary: "Opened the deterministic browser-debug route in a local session.",
      timestamp: "Now",
    },
  ];

  let composerText =
    "Write the next UI refinement here.\n\nThis browser-debug route is deterministic, safe outside Tauri, and meant for screenshot verification.";
  let selectedModel = "gemma4:e2b";
  let selectedPermission: PermissionMode = "workspace-write";
  let selectedGitPath = gitState.changedFiles[0]?.path ?? null;
  const gitDiffText = `diff --git a/src/ui/routes/debug/browser/+page.svelte b/src/ui/routes/debug/browser/+page.svelte\n+<TranscriptPanel ... />\n+// deterministic browser-debug state`;

  function noop() {}
</script>

<svelte:head>
  <title>Poro Browser Debug</title>
  <meta
    name="description"
    content="Deterministic browser-debug surface for validating the Poro session UI outside Tauri."
  />
</svelte:head>

<div class="min-h-screen bg-obsidian text-soft-ivory lg:h-screen lg:overflow-hidden">
  <div class="flex min-h-screen w-full flex-col lg:h-screen">
    <main class="flex min-h-0 min-w-0 flex-1 flex-col lg:overflow-hidden">
      <section class="flex min-h-0 flex-1">
        <TranscriptPanel
          {session}
          {gitState}
          {selectedGitPath}
          {gitDiffText}
          gitDiffLoading={false}
          gitError={null}
          {runtimeActivity}
          {browserActivity}
          {selectedModel}
          {selectedPermission}
          modelOptions={providerModelOptions.local}
          {permissionModes}
          {composerText}
          runtimeActive={true}
          runtimeBusy={true}
          runtimeStatusLine="Browser-debug mode is rendering deterministic mock state for screenshot verification."
          onSelectModel={(model) => (selectedModel = model)}
          onSelectPermission={(mode) => (selectedPermission = mode)}
          onComposerInput={(value) => (composerText = value)}
          onSubmitPrompt={noop}
          onSelectGitPath={(path) => (selectedGitPath = path)}
          onRefreshGit={noop}
          onOpenWorkspaceSwitcher={noop}
          onOpenSettings={noop}
          onStopRuntime={noop}
        />
      </section>
    </main>
  </div>
</div>
