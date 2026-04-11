<script lang="ts">
  import { onMount } from "svelte";
  import BrowserPanel from "$components/desktop/BrowserPanel.svelte";
  import SettingsSheet from "$components/desktop/SettingsSheet.svelte";
  import Sidebar from "$components/desktop/Sidebar.svelte";
  import TranscriptPanel from "$components/desktop/TranscriptPanel.svelte";
  import WorkspacePickerModal from "$components/desktop/WorkspacePickerModal.svelte";
  import {
    listenToBrowserRuntimeEvents,
    sendBrowserCommand,
    startBrowserRuntime,
    stopBrowserRuntime,
    type BrowserRuntimeEvent,
    type BrowserRuntimeLaunch,
  } from "$lib/browserRuntime";
  import {
    checkClawBackend,
    deleteClawSession,
    listenToClawRuntimeEvents,
    listClawSessions,
    loadClawSession,
    sendClawInput,
    startClawRuntime,
    stopClawRuntime,
    type BackendHealth,
    type ClawActivityItem,
    type ClawSessionSnapshot,
    type ClawSessionSummary,
    type RuntimeEvent,
    type RuntimeLaunch,
  } from "$lib/clawRuntime";
  import {
    loadWorkspaceGitDiff,
    loadWorkspaceGitState,
    type WorkspaceGitState,
  } from "$lib/gitRuntime";
  import {
    permissionModes,
    providers,
    providerDefaultModel,
    providerModelOptions,
    type ActivityItem,
    type PermissionMode,
    type SessionRecord,
    type TranscriptMessage,
    type WorkspaceRecord,
  } from "$lib/mockDesktopData";
  import {
    cloneInitialWorkspaces,
    createWorkspaceRecord,
    defaultDesktopSettings,
    isDesktopEnvironment,
    loadDesktopSettings,
    pickWorkspaceDirectory,
    saveDesktopSettings,
    type DesktopSettings,
  } from "$lib/desktop";

  function normalizeSelectedModel(providerId: string, model: string) {
    const validModels = providerModelOptions[providerId] ?? providerModelOptions.local;
    return validModels.includes(model)
      ? model
      : providerDefaultModel[providerId] ?? providerDefaultModel.local;
  }

  const initialWorkspaces = cloneInitialWorkspaces();
  let workspaceList = $state(initialWorkspaces);
  let selectedWorkspaceId = $state(initialWorkspaces[0].id);
  let selectedSessionId = $state(initialWorkspaces[0].sessions[0].id);
  let selectedProviderId = $state(defaultDesktopSettings.selectedProviderId);
  let selectedModel = $state(defaultDesktopSettings.selectedModel);
  let selectedPermission = $state<PermissionMode>("workspace-write");
  let backendPath = $state(defaultDesktopSettings.backendPath);
  let composerText = $state(initialWorkspaces[0].sessions[0].draft);
  let desktopReady = $state(false);
  let showWorkspacePicker = $state(false);
  let showSettings = $state(false);
  let showBrowserInspector = $state(false);
  let backendHealth = $state<BackendHealth | null>(null);
  let healthCheckPending = $state(false);
  let runtimeError = $state<string | null>(null);
  let gitStateByWorkspace = $state<Record<string, WorkspaceGitState | null>>({});
  let gitSelectedPathByWorkspace = $state<Record<string, string | null>>({});
  let gitDiffByWorkspace = $state<Record<string, string>>({});
  let gitDiffLoadingByWorkspace = $state<Record<string, boolean>>({});
  let gitErrorByWorkspace = $state<Record<string, string | null>>({});
  let runtimeFeedByWorkspace = $state<Record<string, ActivityItem[]>>({});
  let activeRuntimeByWorkspace = $state<Record<string, RuntimeLaunch>>({});
  let runtimeWorkspaceById = $state<Record<string, string>>({});
  let turnInFlightByWorkspace = $state<Record<string, boolean>>({});
  let browserUrlByWorkspace = $state<Record<string, string>>({});
  let browserFeedByWorkspace = $state<Record<string, ActivityItem[]>>({});
  let activeBrowserByWorkspace = $state<Record<string, BrowserRuntimeLaunch>>({});
  let browserWorkspaceById = $state<Record<string, string>>({});
  let browserBusyByWorkspace = $state<Record<string, boolean>>({});
  let browserHeadlessByWorkspace = $state<Record<string, boolean>>({});
  let browserLatestSnapshotByWorkspace = $state<Record<string, string>>({});
  let browserLatestPayloadByWorkspace = $state<Record<string, string>>({});
  let browserErrorByWorkspace = $state<Record<string, string | null>>({});

  let selectedWorkspace = $derived(
    workspaceList.find((workspace) => workspace.id === selectedWorkspaceId) ?? workspaceList[0]
  );
  let selectedSession = $derived(
    selectedWorkspace.sessions.find((session) => session.id === selectedSessionId) ??
      selectedWorkspace.sessions[0]
  );
  let activeProviderLabel = $derived(
    providers.find((provider) => provider.id === selectedProviderId)?.label ?? providers[0].label
  );
  let visibleModelOptions = $derived(
    providerModelOptions[selectedProviderId] ?? providerModelOptions.local
  );
  let selectedRuntime = $derived(activeRuntimeByWorkspace[selectedWorkspace.path] ?? null);
  let runtimeBusy = $derived(turnInFlightByWorkspace[selectedWorkspace.path] ?? false);
  let runtimeActivity = $derived(runtimeFeedByWorkspace[selectedWorkspace.path] ?? []);
  let gitState = $derived(gitStateByWorkspace[selectedWorkspace.path] ?? null);
  let selectedGitPath = $derived(gitSelectedPathByWorkspace[selectedWorkspace.path] ?? null);
  let gitDiffText = $derived(gitDiffByWorkspace[selectedWorkspace.path] ?? "");
  let gitDiffLoading = $derived(gitDiffLoadingByWorkspace[selectedWorkspace.path] ?? false);
  let gitError = $derived(gitErrorByWorkspace[selectedWorkspace.path] ?? null);
  let selectedBrowserRuntime = $derived(activeBrowserByWorkspace[selectedWorkspace.path] ?? null);
  let browserBusy = $derived(browserBusyByWorkspace[selectedWorkspace.path] ?? false);
  let browserUrl = $derived(
    browserUrlByWorkspace[selectedWorkspace.path] ?? "https://example.com"
  );
  let browserHeadless = $derived(browserHeadlessByWorkspace[selectedWorkspace.path] ?? false);
  let browserActivity = $derived(browserFeedByWorkspace[selectedWorkspace.path] ?? []);
  let browserLatestSnapshot = $derived(
    browserLatestSnapshotByWorkspace[selectedWorkspace.path] ?? ""
  );
  let browserLatestPayload = $derived(
    browserLatestPayloadByWorkspace[selectedWorkspace.path] ?? ""
  );
  let runtimeStatusLine = $derived(
    runtimeError ??
      (runtimeBusy
        ? selectedProviderId === "local"
          ? `Harness is working on this turn now. Local Gemma can take a bit, but the app is still alive and will update as soon as the turn finishes.`
          : `Harness is working on this turn now. Hosted frontier models still take time on tool-heavy turns, but the app is alive and will update as soon as the turn finishes.`
        : null) ??
      (selectedRuntime
        ? selectedRuntime.providerId === "local"
          ? `Connected to the local Harness runtime in ${selectedWorkspace.name}. Saved sessions sync from the desktop session store after each turn.`
          : `Connected to ${selectedRuntime.model} in ${selectedWorkspace.name}. Saved sessions sync from the desktop session store after each turn.`
        : backendHealth?.message ??
          "Launch a Harness runtime to begin working in this workspace.")
  );
  let browserStatusLine = $derived(
    browserErrorByWorkspace[selectedWorkspace.path] ??
      (browserBusy
        ? "Brave is handling the current browser command now. The panel will update as soon as the browser sidecar responds."
        : null) ??
      (selectedBrowserRuntime
        ? selectedBrowserRuntime.headless
          ? `Connected to the browser sidecar in headless mode for ${selectedWorkspace.name}.`
          : `Connected to a visible Brave session for ${selectedWorkspace.name}.`
        : "Launch Brave to inspect a page, capture snapshots, and validate the browser sidecar path inside Poro.")
  );

  onMount(() => {
    let stopListening = () => {};
    let stopBrowserListening = () => {};

    void (async () => {
      desktopReady = isDesktopEnvironment();

      const settings = await loadDesktopSettings();
      applyDesktopSettings(settings);
      composerText = workspaceList[0]?.sessions[0]?.draft ?? "";

      if (desktopReady) {
        stopListening = await listenToClawRuntimeEvents((event) => {
          void handleRuntimeEvent(event);
        });
        stopBrowserListening = await listenToBrowserRuntimeEvents((event) => {
          void handleBrowserRuntimeEvent(event);
        });
      }

      await refreshSelectedWorkspace();
    })();

    return () => {
      stopListening();
      stopBrowserListening();
    };
  });

  function updateWorkspace(workspaceId: string, updater: (workspace: WorkspaceRecord) => WorkspaceRecord) {
    workspaceList = workspaceList.map((workspace) =>
      workspace.id === workspaceId ? updater(workspace) : workspace
    );
  }

  function applyWorkspaceGitState(workspaceId: string, gitState: WorkspaceGitState) {
    updateWorkspace(workspaceId, (currentWorkspace) => ({
      ...currentWorkspace,
      branch: gitState.branch,
      sessions: currentWorkspace.sessions.map((session) => ({
        ...session,
        branch: gitState.branch,
      })),
    }));
  }

  function findWorkspaceByPath(path: string) {
    return workspaceList.find((workspace) => workspace.path === path);
  }

  function findWorkspaceIdByPath(path: string) {
    return findWorkspaceByPath(path)?.id ?? null;
  }

  function createPlaceholderSession(workspace: WorkspaceRecord): SessionRecord {
    const runtime = activeRuntimeByWorkspace[workspace.path];

    return {
      id: `${workspace.id}-ready`,
      source: "mock",
      title: runtime ? "Runtime warming up" : "New harness session",
      branch: workspace.branch,
      status: runtime ? "Live" : "Ready",
      updatedAt: runtime ? "Starting now" : "Waiting to start",
      model: selectedModel,
      provider: activeProviderLabel,
      permission: selectedPermission,
      tokenUsage: runtime ? "Runtime active" : "0 messages",
      cost: selectedProviderId === "local" ? "On-device" : "BYO provider",
      cwd: workspace.path,
      goal: runtime
        ? runtime.message
        : "Launch a real Harness session to replace the placeholder shell state.",
      draft:
        "Describe the coding task you want to run. Poro will launch the selected Harness runtime and sync transcript state from the desktop session store.",
      transcript: [
        {
          id: `${workspace.id}-ready-system`,
          role: "system",
          title: "Runtime ready",
          body: runtime
            ? "Poro launched a runtime for this workspace. The saved session file will appear as soon as Harness persists its first turn."
            : "No saved Harness sessions were found for this workspace yet.",
          meta: runtime ? "Launching" : "Waiting",
        },
      ],
      activity: runtimeFeedByWorkspace[workspace.path] ?? [],
      changes: [],
    };
  }

  function toActivityStatus(status: string): ActivityItem["status"] {
    if (status === "complete" || status === "queued") {
      return status;
    }

    return "active";
  }

  function toTranscriptRole(role: string): TranscriptMessage["role"] {
    if (role === "assistant" || role === "user" || role === "tool") {
      return role;
    }

    return "system";
  }

  function mergeRuntimeActivity(
    workspacePath: string,
    activity: ClawActivityItem[]
  ): ActivityItem[] {
    const runtimeFeed = runtimeFeedByWorkspace[workspacePath] ?? [];
    const mapped = [...activity].reverse().map((item) => ({
      id: item.id,
      label: item.label,
      status: toActivityStatus(item.status),
      summary: item.summary,
      timestamp: item.timestamp,
    }));

    const deduped = [...runtimeFeed, ...mapped].filter(
      (item, index, items) => items.findIndex((candidate) => candidate.id === item.id) === index
    );

    return deduped.slice(0, 12);
  }

  function mapSnapshotToSession(
    workspace: WorkspaceRecord,
    snapshot: ClawSessionSnapshot,
    runtimeId: string | null = null
  ): SessionRecord {
    const runtime = activeRuntimeByWorkspace[workspace.path];
    const isLive =
      !!runtime &&
      (runtime.runtimeId === runtimeId ||
        runtime.sessionPath === snapshot.path ||
        runtime.sessionId === snapshot.id);

    const tokenUsage = snapshot.latestUsage
      ? `${snapshot.latestUsage.inputTokens} in • ${snapshot.latestUsage.outputTokens} out`
      : `${snapshot.messageCount} messages`;

    return {
      id: snapshot.id,
      sessionPath: snapshot.path,
      runtimeId,
      source: "claw",
      title: snapshot.preview || snapshot.id,
      branch: workspace.branch,
      status: isLive ? "Live" : "Paused",
      updatedAt: snapshot.modifiedLabel,
      model: runtime?.model ?? selectedModel,
      provider: activeProviderLabel,
      permission: selectedPermission,
      tokenUsage,
      cost: runtime?.providerId === "local" ? "On-device" : "BYO provider",
      cwd: workspace.path,
      goal: snapshot.preview,
      draft: composerText || snapshot.preview,
      transcript: snapshot.transcript.map((message) => ({
        id: message.id,
        role: toTranscriptRole(message.role),
        title: message.title,
        body: message.body,
        meta: message.meta,
      })),
      activity: mergeRuntimeActivity(workspace.path, snapshot.activity),
      changes: snapshot.changes.map((change) => ({
        path: change.path,
        summary: change.summary,
        additions: change.additions,
        deletions: change.deletions,
      })),
    };
  }

  function mapSummaryToSession(
    workspace: WorkspaceRecord,
    summary: ClawSessionSummary
  ): SessionRecord {
    const existing = workspace.sessions.find(
      (session) => session.sessionPath === summary.path || session.id === summary.id
    );
    const runtime = activeRuntimeByWorkspace[workspace.path];
    const isLive =
      !!runtime &&
      (runtime.sessionPath === summary.path ||
        runtime.sessionId === summary.id ||
        (!runtime.sessionPath && existing?.runtimeId === runtime.runtimeId));

    return {
      id: summary.id,
      sessionPath: summary.path,
      runtimeId: isLive ? runtime?.runtimeId ?? null : null,
      source: "claw",
      title: existing?.title ?? summary.preview,
      branch: workspace.branch,
      status: isLive ? "Live" : existing?.status ?? "Paused",
      updatedAt: summary.modifiedLabel,
      model: runtime?.model ?? existing?.model ?? selectedModel,
      provider: existing?.provider ?? activeProviderLabel,
      permission: existing?.permission ?? selectedPermission,
      tokenUsage: existing?.tokenUsage ?? `${summary.messageCount} messages`,
      cost: existing?.cost ?? (selectedProviderId === "local" ? "On-device" : "BYO provider"),
      cwd: workspace.path,
      goal: existing?.goal ?? summary.preview,
      draft: existing?.draft ?? (composerText || summary.preview),
      transcript:
        existing?.transcript ??
        [
          {
            id: `${summary.id}-summary`,
            role: "system",
            title: "Saved session",
            body: "Open this session to load the full transcript, tool activity, and diff summary from the local desktop session store.",
            meta: `${summary.messageCount} messages`,
          },
        ],
      activity: existing?.activity ?? runtimeFeedByWorkspace[workspace.path] ?? [],
      changes: existing?.changes ?? [],
    };
  }

  function applyDesktopSettings(settings: DesktopSettings) {
    backendPath = settings.backendPath;
    selectedProviderId = settings.selectedProviderId;
    selectedModel = normalizeSelectedModel(settings.selectedProviderId, settings.selectedModel);
    selectedPermission = settings.selectedPermission;

    let updatedWorkspaces = cloneInitialWorkspaces();
    for (const path of settings.recentWorkspaces) {
      const existing = updatedWorkspaces.find((workspace) => workspace.path === path);
      if (!existing) {
        updatedWorkspaces = [createWorkspaceRecord(path), ...updatedWorkspaces];
      } else {
        updatedWorkspaces = [
          existing,
          ...updatedWorkspaces.filter((workspace) => workspace.id !== existing.id),
        ];
      }
    }

    workspaceList = updatedWorkspaces;
    selectedWorkspaceId = updatedWorkspaces[0].id;
    selectedSessionId = updatedWorkspaces[0].sessions[0].id;
  }

  async function persistDesktopSettings() {
    const settings: DesktopSettings = {
      backendPath,
      recentWorkspaces: workspaceList.map((workspace) => workspace.path).slice(0, 8),
      selectedProviderId,
      selectedModel,
      selectedPermission,
    };

    await saveDesktopSettings(settings);
  }

  function moveWorkspaceToFront(id: string) {
    const workspace = workspaceList.find((item) => item.id === id);
    if (!workspace) return;

    workspaceList = [workspace, ...workspaceList.filter((item) => item.id !== id)];
  }

  async function hydrateSession(session: SessionRecord) {
    if (!desktopReady || !session.sessionPath) {
      return;
    }

    try {
      const snapshot = await loadClawSession(selectedWorkspace.path, session.sessionPath);
      applySnapshot(snapshot);
    } catch (error) {
      runtimeError = error instanceof Error ? error.message : String(error);
    }
  }

  function applySnapshot(snapshot: ClawSessionSnapshot, runtimeId: string | null = null) {
    const workspace = findWorkspaceByPath(snapshot.workspacePath);
    if (!workspace) {
      return;
    }

    const session = mapSnapshotToSession(workspace, snapshot, runtimeId);
    updateWorkspace(workspace.id, (currentWorkspace) => {
      const existingSessions = currentWorkspace.sessions.filter(
        (item) => item.sessionPath !== snapshot.path && item.id !== snapshot.id
      );

      return {
        ...currentWorkspace,
        status: session.status === "Live" ? "Active session" : currentWorkspace.status,
        lastOpened: "Just now",
        summary: snapshot.preview || currentWorkspace.summary,
        sessions: [session, ...existingSessions],
      };
    });

    if (selectedWorkspaceId === workspace.id) {
      selectedSessionId = session.id;
    }

    void refreshWorkspaceGitState(workspace.id);
  }

  async function loadGitDiffForWorkspace(workspacePath: string, filePath: string | null) {
    if (!desktopReady) {
      return;
    }

    gitSelectedPathByWorkspace = {
      ...gitSelectedPathByWorkspace,
      [workspacePath]: filePath,
    };
    gitErrorByWorkspace = {
      ...gitErrorByWorkspace,
      [workspacePath]: null,
    };

    if (!filePath) {
      gitDiffByWorkspace = {
        ...gitDiffByWorkspace,
        [workspacePath]: "",
      };
      gitDiffLoadingByWorkspace = {
        ...gitDiffLoadingByWorkspace,
        [workspacePath]: false,
      };
      return;
    }

    gitDiffLoadingByWorkspace = {
      ...gitDiffLoadingByWorkspace,
      [workspacePath]: true,
    };

    try {
      const diff = await loadWorkspaceGitDiff(workspacePath, filePath);
      gitDiffByWorkspace = {
        ...gitDiffByWorkspace,
        [workspacePath]: diff,
      };
    } catch (error) {
      gitErrorByWorkspace = {
        ...gitErrorByWorkspace,
        [workspacePath]: error instanceof Error ? error.message : String(error),
      };
      gitDiffByWorkspace = {
        ...gitDiffByWorkspace,
        [workspacePath]: "",
      };
    } finally {
      gitDiffLoadingByWorkspace = {
        ...gitDiffLoadingByWorkspace,
        [workspacePath]: false,
      };
    }
  }

  async function refreshWorkspaceGitState(workspaceId: string) {
    const workspace = workspaceList.find((item) => item.id === workspaceId);
    if (!workspace || !desktopReady) {
      return;
    }

    try {
      const gitState = await loadWorkspaceGitState(workspace.path);
      if (!gitState) {
        return;
      }

      applyWorkspaceGitState(workspaceId, gitState);

      const currentSelection = gitSelectedPathByWorkspace[workspace.path];
      const nextSelectedPath =
        currentSelection && gitState.changedFiles.some((file) => file.path === currentSelection)
          ? currentSelection
          : gitState.changedFiles[0]?.path ?? null;

      await loadGitDiffForWorkspace(workspace.path, nextSelectedPath);
    } catch (error) {
      gitErrorByWorkspace = {
        ...gitErrorByWorkspace,
        [workspace.path]: error instanceof Error ? error.message : String(error),
      };
      gitDiffByWorkspace = {
        ...gitDiffByWorkspace,
        [workspace.path]: "",
      };
      gitDiffLoadingByWorkspace = {
        ...gitDiffLoadingByWorkspace,
        [workspace.path]: false,
      };
    }
  }

  async function refreshWorkspaceSessions(workspaceId: string, focusSelected = false) {
    const workspace = workspaceList.find((item) => item.id === workspaceId);
    if (!workspace || !desktopReady) {
      return;
    }

    try {
      const summaries = await listClawSessions(workspace.path);
      const mapped = summaries.length
        ? summaries.map((summary) => mapSummaryToSession(workspace, summary))
        : [createPlaceholderSession(workspace)];

      updateWorkspace(workspaceId, (currentWorkspace) => ({
        ...currentWorkspace,
        status:
          activeRuntimeByWorkspace[currentWorkspace.path] || summaries.length
            ? "Active session"
            : "Ready",
        lastOpened: "Just now",
        sessions: mapped,
      }));

      const selectedStillExists = mapped.some((session) => session.id === selectedSessionId);
      if (focusSelected || !selectedStillExists) {
        selectedSessionId = mapped[0].id;
      }

      const currentSelection =
        mapped.find((session) => session.id === selectedSessionId) ?? mapped[0];
      if (currentSelection?.sessionPath) {
        await hydrateSession(currentSelection);
      }
    } catch (error) {
      runtimeError = error instanceof Error ? error.message : String(error);
    }
  }

  async function runHealthCheck() {
    healthCheckPending = true;

    try {
      const health = await checkClawBackend(
        backendPath,
        selectedWorkspace.path,
        selectedProviderId,
        selectedModel
      );
      backendHealth = health;

      if (health.resolvedPath && health.resolvedPath !== backendPath) {
        backendPath = health.resolvedPath;
        await persistDesktopSettings();
      }
    } catch (error) {
      runtimeError = error instanceof Error ? error.message : String(error);
    } finally {
      healthCheckPending = false;
    }
  }

  async function refreshSelectedWorkspace() {
    await Promise.all([
      refreshWorkspaceSessions(selectedWorkspaceId, true),
      refreshWorkspaceGitState(selectedWorkspaceId),
      runHealthCheck(),
    ]);
  }

  function setBackendPath(path: string) {
    backendPath = path;
    void persistDesktopSettings();
  }

  function setComposerText(value: string) {
    composerText = value;
  }

  function pushRuntimeFeed(
    workspacePath: string,
    label: string,
    summary: string,
    status: ActivityItem["status"] = "active",
    timestamp = "Now"
  ) {
    const nextItem: ActivityItem = {
      id: `${workspacePath}-${Date.now()}-${Math.random().toString(16).slice(2, 8)}`,
      label,
      status,
      summary,
      timestamp,
    };

    runtimeFeedByWorkspace = {
      ...runtimeFeedByWorkspace,
      [workspacePath]: [nextItem, ...(runtimeFeedByWorkspace[workspacePath] ?? [])].slice(0, 8),
    };
  }

  function describeRuntimeOutput(
    line: string,
    stream: string
  ): Pick<ActivityItem, "label" | "summary" | "status"> {
    const text = line.trim();
    if (!text) {
      return {
        label: stream === "stderr" ? "Runtime warning" : "Agent update",
        summary: "Harness emitted an empty runtime line.",
        status: stream === "stderr" ? "queued" : "active",
      };
    }

    if (text.startsWith("Running verification: ")) {
      return {
        label: "Verification running",
        summary: text.replace("Running verification: ", ""),
        status: "active",
      };
    }

    if (text.startsWith("Verification passed: ")) {
      return {
        label: "Verification passed",
        summary: text.replace("Verification passed: ", ""),
        status: "complete",
      };
    }

    if (text.startsWith("Verification failed: ")) {
      return {
        label: "Verification failed",
        summary: text.replace("Verification failed: ", ""),
        status: "queued",
      };
    }

    if (text.startsWith("Runtime critique started: ")) {
      return {
        label: "Thinking",
        summary: text.replace("Runtime critique started: ", ""),
        status: "active",
      };
    }

    if (text.startsWith("Runtime critique finished: ")) {
      return {
        label: "Thinking complete",
        summary: text.replace("Runtime critique finished: ", ""),
        status: "complete",
      };
    }

    if (text.startsWith("Patch prepared for ")) {
      return {
        label: "Patch prepared",
        summary: text.replace("Patch prepared for ", ""),
        status: "complete",
      };
    }

    if (text.startsWith("Approval requested for `")) {
      return {
        label: "Approval needed",
        summary: text,
        status: "queued",
      };
    }

    if (text.startsWith("Approved `") || text.startsWith("Rejected `")) {
      return {
        label: "Approval resolved",
        summary: text,
        status: text.startsWith("Approved `") ? "complete" : "queued",
      };
    }

    if (text.startsWith("Harness session started")) {
      return {
        label: "Session started",
        summary: text,
        status: "complete",
      };
    }

    if (text.includes(":")) {
      const [head, ...tail] = text.split(":");
      const prefix = head.trim();
      const summary = tail.join(":").trim();
      if (/^[a-z_][a-z0-9_]*$/i.test(prefix)) {
        return {
          label: stream === "stderr" ? `Tool issue • ${prefix}` : `Tool • ${prefix}`,
          summary: summary || text,
          status: stream === "stderr" ? "queued" : "active",
        };
      }
    }

    return {
      label: stream === "stderr" ? "Runtime warning" : "Agent update",
      summary: text,
      status: stream === "stderr" ? "queued" : "active",
    };
  }

  function pushBrowserFeed(
    workspacePath: string,
    label: string,
    summary: string,
    status: ActivityItem["status"] = "active",
    timestamp = "Now"
  ) {
    const nextItem: ActivityItem = {
      id: `${workspacePath}-browser-${Date.now()}-${Math.random().toString(16).slice(2, 8)}`,
      label,
      status,
      summary,
      timestamp,
    };

    browserFeedByWorkspace = {
      ...browserFeedByWorkspace,
      [workspacePath]: [nextItem, ...(browserFeedByWorkspace[workspacePath] ?? [])].slice(0, 8),
    };
  }

  async function selectWorkspace(id: string) {
    const workspace = workspaceList.find((item) => item.id === id);
    if (!workspace) return;

    moveWorkspaceToFront(workspace.id);
    selectedWorkspaceId = workspace.id;
    selectedSessionId = workspace.sessions[0].id;
    composerText = workspace.sessions[0].draft;
    showWorkspacePicker = false;
    await persistDesktopSettings();
    await refreshSelectedWorkspace();
  }

  async function selectSession(id: string) {
    const session = selectedWorkspace.sessions.find((item) => item.id === id);
    if (!session) {
      return;
    }

    selectedSessionId = id;
    composerText = session.draft;
    await hydrateSession(session);
  }

  async function selectGitPath(path: string) {
    await loadGitDiffForWorkspace(selectedWorkspace.path, path);
  }

  async function deleteSession(id: string) {
    const workspace = selectedWorkspace;
    const session = workspace.sessions.find((item) => item.id === id);
    if (!session) {
      return;
    }

    const message = session.sessionPath
      ? `Delete "${session.title}" from this workspace? This removes its saved local session.`
      : `Delete "${session.title}" from the current thread list?`;
    const confirmed =
      typeof globalThis.confirm === "function" ? globalThis.confirm(message) : true;

    if (!confirmed) {
      return;
    }

    runtimeError = null;

    try {
      if (desktopReady && session.sessionPath) {
        await deleteClawSession(workspace.path, session.sessionPath);
        await refreshWorkspaceSessions(workspace.id, selectedSessionId === id);

        const refreshedWorkspace = workspaceList.find((item) => item.id === workspace.id);
        const nextSession =
          refreshedWorkspace?.sessions.find((item) => item.id === selectedSessionId) ??
          refreshedWorkspace?.sessions[0];
        if (nextSession) {
          composerText = nextSession.draft;
        }
        return;
      }

      const remainingSessions = workspace.sessions.filter((item) => item.id !== id);
      const nextSessions = remainingSessions.length
        ? remainingSessions
        : [createPlaceholderSession(workspace)];
      const nextSelectedSession =
        selectedSessionId === id
          ? nextSessions[0]
          : nextSessions.find((item) => item.id === selectedSessionId) ?? nextSessions[0];

      updateWorkspace(workspace.id, (currentWorkspace) => ({
        ...currentWorkspace,
        status: nextSessions.some((item) => item.status === "Live")
          ? "Active session"
          : "Ready",
        sessions: nextSessions,
      }));

      if (selectedWorkspaceId === workspace.id) {
        selectedSessionId = nextSelectedSession.id;
        composerText = nextSelectedSession.draft;
      }
    } catch (error) {
      runtimeError = error instanceof Error ? error.message : String(error);
    }
  }

  async function setProvider(id: string) {
    selectedProviderId = id;
    selectedModel = providerDefaultModel[id] ?? providerDefaultModel.local;
    await persistDesktopSettings();
    if (desktopReady) {
      await runHealthCheck();
    }
  }

  async function setModel(model: string) {
    selectedModel = model;
    await persistDesktopSettings();
    if (desktopReady) {
      await runHealthCheck();
    }
  }

  async function setPermission(mode: PermissionMode) {
    selectedPermission = mode;
    await persistDesktopSettings();
  }

  async function openWorkspaceFromDisk() {
    const path = await pickWorkspaceDirectory();
    if (!path) {
      showWorkspacePicker = true;
      return;
    }

    let workspace = workspaceList.find((item) => item.path === path);
    if (!workspace) {
      workspace = createWorkspaceRecord(path);
      workspaceList = [workspace, ...workspaceList];
    }

    await selectWorkspace(workspace.id);
  }

  async function ensureRuntime() {
    if (!desktopReady) {
      throw new Error("Runtime launch is available only inside the desktop build.");
    }

    if (selectedRuntime) {
      return selectedRuntime;
    }

    const launch = await startClawRuntime({
      backendPath,
      workspacePath: selectedWorkspace.path,
      providerId: selectedProviderId,
      model: selectedModel,
      permissionMode: selectedPermission,
      resumeSessionPath: selectedSession.sessionPath ?? null,
    });

    activeRuntimeByWorkspace = {
      ...activeRuntimeByWorkspace,
      [selectedWorkspace.path]: launch,
    };
    runtimeWorkspaceById = {
      ...runtimeWorkspaceById,
      [launch.runtimeId]: selectedWorkspace.path,
    };

    return launch;
  }

  function normalizePromptInput(input: string) {
    return input
      .split(/\r?\n+/)
      .map((line) => line.trim())
      .filter(Boolean)
      .join(" ");
  }

  async function submitPrompt() {
    const normalizedInput = normalizePromptInput(composerText);
    if (!normalizedInput) {
      return;
    }

    runtimeError = null;

    try {
      const runtime = await ensureRuntime();
      turnInFlightByWorkspace = {
        ...turnInFlightByWorkspace,
        [selectedWorkspace.path]: true,
      };
      pushRuntimeFeed(
        selectedWorkspace.path,
        "Prompt queued",
        truncatePrompt(normalizedInput),
        "active"
      );
      await sendClawInput(runtime.runtimeId, normalizedInput);
      composerText = "";
    } catch (error) {
      turnInFlightByWorkspace = {
        ...turnInFlightByWorkspace,
        [selectedWorkspace.path]: false,
      };
      runtimeError = error instanceof Error ? error.message : String(error);
    }
  }

  async function stopSelectedRuntime() {
    if (!selectedRuntime) {
      await refreshSelectedWorkspace();
      return;
    }

    runtimeError = null;

    try {
      await stopClawRuntime(selectedRuntime.runtimeId);
      pushRuntimeFeed(
        selectedWorkspace.path,
        "Runtime stopping",
        "Poro asked the local session to stop.",
        "queued"
      );
    } catch (error) {
      runtimeError = error instanceof Error ? error.message : String(error);
    }
  }

  function truncatePrompt(value: string) {
    return value.length > 200 ? `${value.slice(0, 200)}…` : value;
  }

  function formatBrowserPayload(value: unknown) {
    return JSON.stringify(value, null, 2);
  }

  function setBrowserUrl(value: string) {
    browserUrlByWorkspace = {
      ...browserUrlByWorkspace,
      [selectedWorkspace.path]: value,
    };
  }

  function toggleBrowserHeadless() {
    browserHeadlessByWorkspace = {
      ...browserHeadlessByWorkspace,
      [selectedWorkspace.path]: !browserHeadless,
    };
  }

  async function ensureBrowserRuntime() {
    if (!desktopReady) {
      throw new Error("Browser runtime launch is available only inside the desktop build.");
    }

    const existing = activeBrowserByWorkspace[selectedWorkspace.path];
    if (existing) {
      return existing;
    }

    const launch = await startBrowserRuntime({
      session: `poro-${selectedWorkspace.id}`,
      headless: browserHeadless,
    });

    activeBrowserByWorkspace = {
      ...activeBrowserByWorkspace,
      [selectedWorkspace.path]: launch,
    };
    browserWorkspaceById = {
      ...browserWorkspaceById,
      [launch.runtimeId]: selectedWorkspace.path,
    };
    browserErrorByWorkspace = {
      ...browserErrorByWorkspace,
      [selectedWorkspace.path]: null,
    };
    pushBrowserFeed(selectedWorkspace.path, "Browser launched", launch.message, "complete");

    return launch;
  }

  async function launchBrowserOnly() {
    browserBusyByWorkspace = {
      ...browserBusyByWorkspace,
      [selectedWorkspace.path]: true,
    };
    browserErrorByWorkspace = {
      ...browserErrorByWorkspace,
      [selectedWorkspace.path]: null,
    };

    try {
      await ensureBrowserRuntime();
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      browserErrorByWorkspace = {
        ...browserErrorByWorkspace,
        [selectedWorkspace.path]: message,
      };
    } finally {
      browserBusyByWorkspace = {
        ...browserBusyByWorkspace,
        [selectedWorkspace.path]: false,
      };
    }
  }

  async function runBrowserCommand(command: string[]) {
    const runtime = await ensureBrowserRuntime();
    const response = await sendBrowserCommand({
      runtimeId: runtime.runtimeId,
      command,
    });

    if (!response.success) {
      throw new Error(response.error ?? "Browser command failed.");
    }

    return response;
  }

  async function openAndSnapshotBrowser() {
    const url = browserUrl.trim();
    if (!url) {
      return;
    }

    browserBusyByWorkspace = {
      ...browserBusyByWorkspace,
      [selectedWorkspace.path]: true,
    };
    browserErrorByWorkspace = {
      ...browserErrorByWorkspace,
      [selectedWorkspace.path]: null,
    };

    try {
      await runBrowserCommand(["open", url]);
      await runBrowserCommand(["snapshot", "-i"]);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      browserErrorByWorkspace = {
        ...browserErrorByWorkspace,
        [selectedWorkspace.path]: message,
      };
    } finally {
      browserBusyByWorkspace = {
        ...browserBusyByWorkspace,
        [selectedWorkspace.path]: false,
      };
    }
  }

  async function snapshotBrowser() {
    browserBusyByWorkspace = {
      ...browserBusyByWorkspace,
      [selectedWorkspace.path]: true,
    };
    browserErrorByWorkspace = {
      ...browserErrorByWorkspace,
      [selectedWorkspace.path]: null,
    };

    try {
      await runBrowserCommand(["snapshot", "-i"]);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      browserErrorByWorkspace = {
        ...browserErrorByWorkspace,
        [selectedWorkspace.path]: message,
      };
    } finally {
      browserBusyByWorkspace = {
        ...browserBusyByWorkspace,
        [selectedWorkspace.path]: false,
      };
    }
  }

  async function stopSelectedBrowser() {
    if (!selectedBrowserRuntime) {
      return;
    }

    browserBusyByWorkspace = {
      ...browserBusyByWorkspace,
      [selectedWorkspace.path]: true,
    };
    browserErrorByWorkspace = {
      ...browserErrorByWorkspace,
      [selectedWorkspace.path]: null,
    };

    try {
      await stopBrowserRuntime(selectedBrowserRuntime.runtimeId);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      browserErrorByWorkspace = {
        ...browserErrorByWorkspace,
        [selectedWorkspace.path]: message,
      };
      browserBusyByWorkspace = {
        ...browserBusyByWorkspace,
        [selectedWorkspace.path]: false,
      };
    }
  }

  async function handleRuntimeEvent(event: RuntimeEvent) {
    if (event.type === "started") {
      const launch = event.launch;
      activeRuntimeByWorkspace = {
        ...activeRuntimeByWorkspace,
        [launch.workspacePath]: launch,
      };
      runtimeWorkspaceById = {
        ...runtimeWorkspaceById,
        [launch.runtimeId]: launch.workspacePath,
      };
      pushRuntimeFeed(
        launch.workspacePath,
        "Runtime launched",
        launch.message,
        "active"
      );
      const workspaceId = findWorkspaceIdByPath(launch.workspacePath);
      if (workspaceId) {
        await refreshWorkspaceSessions(workspaceId, workspaceId === selectedWorkspaceId);
      }
      return;
    }

    if (event.type === "output") {
      const workspacePath = runtimeWorkspaceById[event.runtimeId];
      if (workspacePath) {
        const details = describeRuntimeOutput(event.line, event.stream);
        pushRuntimeFeed(
          workspacePath,
          details.label,
          details.summary,
          details.status,
          event.timestamp
        );
      }
      return;
    }

    if (event.type === "turnStarted") {
      const workspacePath = runtimeWorkspaceById[event.runtimeId];
      if (workspacePath) {
        turnInFlightByWorkspace = {
          ...turnInFlightByWorkspace,
          [workspacePath]: true,
        };
        pushRuntimeFeed(
          workspacePath,
          "Thinking",
          `Planning the next steps for: ${event.inputPreview}`,
          "active"
        );
      }
      return;
    }

    if (event.type === "sessionAttached") {
      const workspacePath = runtimeWorkspaceById[event.runtimeId];
      if (!workspacePath) {
        return;
      }

      const currentRuntime = activeRuntimeByWorkspace[workspacePath];
      if (currentRuntime) {
        activeRuntimeByWorkspace = {
          ...activeRuntimeByWorkspace,
          [workspacePath]: {
            ...currentRuntime,
            sessionId: event.sessionId,
            sessionPath: event.sessionPath,
          },
        };
      }

      pushRuntimeFeed(
        workspacePath,
        event.resumed ? "Session resumed" : "Session attached",
        event.sessionPath,
        "complete"
      );

      const workspaceId = findWorkspaceIdByPath(workspacePath);
      if (workspaceId) {
        await refreshWorkspaceSessions(workspaceId, workspaceId === selectedWorkspaceId);
      }
      return;
    }

    if (event.type === "snapshot") {
      const workspacePath = runtimeWorkspaceById[event.runtimeId];
      if (workspacePath) {
        turnInFlightByWorkspace = {
          ...turnInFlightByWorkspace,
          [workspacePath]: false,
        };
      }
      applySnapshot(event.snapshot, event.runtimeId);
      return;
    }

    if (event.type === "turnFinished") {
      const workspacePath = runtimeWorkspaceById[event.runtimeId];
      if (workspacePath) {
        turnInFlightByWorkspace = {
          ...turnInFlightByWorkspace,
          [workspacePath]: false,
        };
        pushRuntimeFeed(
          workspacePath,
          event.success ? "Turn finished" : "Turn failed",
          event.success
            ? `Harness finished this turn in ${Math.max(1, Math.round(event.durationMs / 1000))}s.`
            : `Harness stopped with an error after ${Math.max(1, Math.round(event.durationMs / 1000))}s.`,
          event.success ? "complete" : "queued"
        );
        const workspaceId = findWorkspaceIdByPath(workspacePath);
        if (workspaceId) {
          await refreshWorkspaceGitState(workspaceId);
        }
      }
      return;
    }

    if (event.type === "stopped") {
      const workspacePath = runtimeWorkspaceById[event.runtimeId];
      if (workspacePath) {
        turnInFlightByWorkspace = {
          ...turnInFlightByWorkspace,
          [workspacePath]: false,
        };
        pushRuntimeFeed(
          workspacePath,
          "Runtime stopped",
          event.message,
          "complete"
        );

        const nextActive = { ...activeRuntimeByWorkspace };
        delete nextActive[workspacePath];
        activeRuntimeByWorkspace = nextActive;

        const nextWorkspaceById = { ...runtimeWorkspaceById };
        delete nextWorkspaceById[event.runtimeId];
        runtimeWorkspaceById = nextWorkspaceById;

        const workspaceId = findWorkspaceIdByPath(workspacePath);
        if (workspaceId) {
          await Promise.all([
            refreshWorkspaceSessions(workspaceId, workspaceId === selectedWorkspaceId),
            refreshWorkspaceGitState(workspaceId),
          ]);
        }
      }
      return;
    }

    if (event.type === "error") {
      runtimeError = event.message;
      const workspacePath = runtimeWorkspaceById[event.runtimeId];
      if (workspacePath) {
        turnInFlightByWorkspace = {
          ...turnInFlightByWorkspace,
          [workspacePath]: false,
        };
        pushRuntimeFeed(workspacePath, "Runtime error", event.message, "queued");
      }
    }
  }

  async function handleBrowserRuntimeEvent(event: BrowserRuntimeEvent) {
    if (event.type === "started") {
      return;
    }

    if (event.type === "output") {
      const workspacePath = browserWorkspaceById[event.runtimeId];
      if (workspacePath) {
        pushBrowserFeed(
          workspacePath,
          event.stream === "stderr" ? "Browser stderr" : "Browser output",
          event.line,
          event.stream === "stderr" ? "queued" : "active"
        );
      }
      return;
    }

    if (event.type === "response") {
      const workspacePath = browserWorkspaceById[event.runtimeId];
      if (!workspacePath) {
        return;
      }

      browserLatestPayloadByWorkspace = {
        ...browserLatestPayloadByWorkspace,
        [workspacePath]: formatBrowserPayload(event.response),
      };

      if (
        event.response.action === "snapshot" &&
        typeof event.response.data?.snapshot === "string"
      ) {
        browserLatestSnapshotByWorkspace = {
          ...browserLatestSnapshotByWorkspace,
          [workspacePath]: event.response.data.snapshot,
        };
      }

      pushBrowserFeed(
        workspacePath,
        event.response.success
          ? event.response.action === "snapshot"
            ? "Snapshot captured"
            : `Browser ${event.response.action ?? "response"}`
          : "Browser command failed",
        event.response.success
          ? event.response.action === "snapshot"
            ? "Captured the current page accessibility snapshot."
            : `Completed ${event.response.action ?? "the browser command"} successfully.`
          : event.response.error ?? "Browser command failed.",
        event.response.success ? "complete" : "queued"
      );

      browserBusyByWorkspace = {
        ...browserBusyByWorkspace,
        [workspacePath]: false,
      };

      if (event.response.success) {
        browserErrorByWorkspace = {
          ...browserErrorByWorkspace,
          [workspacePath]: null,
        };
      }
      return;
    }

    if (event.type === "stopped") {
      const workspacePath = browserWorkspaceById[event.runtimeId];
      if (workspacePath) {
        pushBrowserFeed(workspacePath, "Browser stopped", event.message, "complete");
        browserBusyByWorkspace = {
          ...browserBusyByWorkspace,
          [workspacePath]: false,
        };

        const nextActive = { ...activeBrowserByWorkspace };
        delete nextActive[workspacePath];
        activeBrowserByWorkspace = nextActive;

        const nextWorkspaceById = { ...browserWorkspaceById };
        delete nextWorkspaceById[event.runtimeId];
        browserWorkspaceById = nextWorkspaceById;
      }
      return;
    }

    if (event.type === "error") {
      const workspacePath = browserWorkspaceById[event.runtimeId];
      if (workspacePath) {
        browserErrorByWorkspace = {
          ...browserErrorByWorkspace,
          [workspacePath]: event.message,
        };
        browserBusyByWorkspace = {
          ...browserBusyByWorkspace,
          [workspacePath]: false,
        };
        pushBrowserFeed(workspacePath, "Browser error", event.message, "queued");
      }
    }
  }
</script>

<svelte:head>
  <title>Poro Desktop</title>
  <meta
    name="description"
    content="Poro is a local-first desktop AI coding workspace."
  />
</svelte:head>

<div class="min-h-screen bg-obsidian text-soft-ivory lg:h-screen lg:overflow-hidden">
  <div class="flex min-h-screen w-full flex-col lg:h-screen lg:flex-row">
    <Sidebar
      workspaces={workspaceList}
      {selectedWorkspaceId}
      {selectedSessionId}
      onSelectWorkspace={selectWorkspace}
      onSelectSession={selectSession}
      onDeleteSession={deleteSession}
      onPickWorkspace={openWorkspaceFromDisk}
      onOpenWorkspacePicker={() => (showWorkspacePicker = true)}
      onOpenSettings={() => (showSettings = true)}
    />

    <main class="flex min-h-0 min-w-0 flex-1 flex-col lg:overflow-hidden">
      <section class="flex min-h-0 flex-1">
        <TranscriptPanel
          session={selectedSession}
          {gitState}
          {selectedGitPath}
          {gitDiffText}
          {gitDiffLoading}
          {gitError}
          {runtimeActivity}
          browserActivity={browserActivity}
          {selectedModel}
          {selectedPermission}
          modelOptions={visibleModelOptions}
          {permissionModes}
          {composerText}
          runtimeActive={!!selectedRuntime}
          {runtimeBusy}
          {runtimeStatusLine}
          onSelectModel={setModel}
          onSelectPermission={setPermission}
          onComposerInput={setComposerText}
          onSubmitPrompt={submitPrompt}
          onSelectGitPath={selectGitPath}
          onRefreshGit={() => refreshWorkspaceGitState(selectedWorkspaceId)}
          onOpenSettings={() => (showSettings = true)}
          onStopRuntime={stopSelectedRuntime}
          onRefreshRuntime={refreshSelectedWorkspace}
        />
      </section>
    </main>
  </div>

  <WorkspacePickerModal
    open={showWorkspacePicker}
    workspaces={workspaceList}
    {selectedWorkspaceId}
    onSelectWorkspace={selectWorkspace}
    onOpenFromDisk={openWorkspaceFromDisk}
    onClose={() => (showWorkspacePicker = false)}
  />

  <SettingsSheet
    open={showSettings}
    {backendPath}
    {backendHealth}
    {healthCheckPending}
    isDesktop={desktopReady}
    recentWorkspaceCount={workspaceList.length}
    providers={providers}
    {selectedProviderId}
    {selectedModel}
    {selectedPermission}
    onSelectProvider={setProvider}
    onSelectModel={setModel}
    onSelectPermission={setPermission}
    onBackendPathChange={setBackendPath}
    onRunHealthCheck={runHealthCheck}
    modelOptions={visibleModelOptions}
    {permissionModes}
    onClose={() => (showSettings = false)}
  />

  <BrowserPanel
    open={showBrowserInspector}
    {browserUrl}
    runtimeActive={!!selectedBrowserRuntime}
    runtimeBusy={browserBusy}
    headless={browserHeadless}
    runtimeInfo={selectedBrowserRuntime
      ? {
          session: selectedBrowserRuntime.session,
          browserPath: selectedBrowserRuntime.browserPath,
          headless: selectedBrowserRuntime.headless,
        }
      : null}
    statusLine={browserStatusLine}
    latestSnapshot={browserLatestSnapshot}
    latestPayload={browserLatestPayload}
    onBrowserUrlInput={setBrowserUrl}
    onToggleHeadless={toggleBrowserHeadless}
    onLaunchBrowser={launchBrowserOnly}
    onOpenAndSnapshot={openAndSnapshotBrowser}
    onSnapshotBrowser={snapshotBrowser}
    onStopBrowser={stopSelectedBrowser}
    onClose={() => (showBrowserInspector = false)}
  />
</div>
