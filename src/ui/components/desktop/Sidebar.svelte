<script lang="ts">
  import Button from "$components/basic_elems/Button.svelte";
  import type { WorkspaceRecord } from "$lib/mockDesktopData";

  interface Props {
    workspaces: WorkspaceRecord[];
    selectedWorkspaceId: string;
    selectedSessionId: string;
    onSelectWorkspace: (id: string) => void;
    onSelectSession: (id: string) => void;
    onPickWorkspace: () => void;
    onOpenWorkspacePicker: () => void;
    onOpenSettings: () => void;
  }

  let {
    workspaces,
    selectedWorkspaceId,
    selectedSessionId,
    onSelectWorkspace,
    onSelectSession,
    onPickWorkspace,
    onOpenWorkspacePicker,
    onOpenSettings,
  }: Props = $props();

  let selectedWorkspace = $derived(
    workspaces.find((workspace) => workspace.id === selectedWorkspaceId) ?? workspaces[0]
  );
</script>

<aside
  class="flex min-h-0 w-full shrink-0 flex-col rounded-[22px] border border-white/8 bg-dark-slate/92 p-3 shadow-[0_24px_80px_rgba(0,0,0,0.42)] backdrop-blur-md sm:p-4 lg:w-[318px]"
>
  <div class="rounded-[18px] border border-white/8 bg-carbon-black/72 p-4">
    <div class="flex items-center justify-between gap-3">
      <div class="flex items-center gap-3">
        <div class="h-2.5 w-2.5 rounded-full bg-signal-blue shadow-[0_0_18px_rgba(78,161,255,0.85)]"></div>
        <div class="text-[0.72rem] uppercase tracking-[0.34em] text-fog/70">
          Poro
        </div>
      </div>
      <span class="rounded-full border border-white/8 bg-white/4 px-2.5 py-1 text-[0.62rem] uppercase tracking-[0.24em] text-fog/52">
        Desktop
      </span>
    </div>

    <h1 class="mt-5 max-w-[16ch] text-[1.7rem] font-medium leading-[1.04] tracking-[-0.04em] text-soft-ivory sm:text-[1.95rem]">
      Calm, local-first coding sessions with visible state.
    </h1>

    <p class="mt-3 max-w-[29ch] text-sm leading-6 text-fog/70">
      Harness-backed sessions, thinner chrome, and enough runtime detail to trust
      the agent without living in a terminal pane.
    </p>

    <div class="mt-5 grid grid-cols-2 gap-2">
      <Button
        label="Open Workspace"
        variant="gold"
        class="w-full"
        height="h-11"
        onclick={onPickWorkspace}
      />
      <Button
        label="Settings"
        variant="outline"
        class="w-full"
        height="h-11"
        onclick={onOpenSettings}
      />
    </div>
  </div>

  <div class="mt-4 flex items-center justify-between px-1">
    <div class="text-[0.66rem] font-medium uppercase tracking-[0.34em] text-fog/48">
      Workspaces
    </div>
    <button
      type="button"
      class="text-[0.68rem] font-medium uppercase tracking-[0.22em] text-fog/44 transition hover:text-fog/78"
      onclick={onOpenWorkspacePicker}
    >
      Browse
    </button>
  </div>

  <div class="mt-3 grid gap-2 md:grid-cols-2 lg:grid-cols-1">
    {#each workspaces as workspace}
      <button
        type="button"
        class={`w-full rounded-[16px] border p-4 text-left transition ${
          workspace.id === selectedWorkspaceId
            ? "border-signal-blue/35 bg-signal-blue/10 shadow-[inset_0_0_0_1px_rgba(78,161,255,0.12)]"
            : "border-white/7 bg-white/[0.025] hover:border-white/12 hover:bg-white/[0.045]"
        }`}
        onclick={() => onSelectWorkspace(workspace.id)}
      >
        <div class="flex items-start justify-between gap-3">
          <div class="min-w-0">
            <div class="truncate text-[0.96rem] font-medium text-soft-ivory">
              {workspace.name}
            </div>
            <div class="mt-1 font-mono text-[0.68rem] uppercase tracking-[0.2em] text-fog/45">
              {workspace.branch}
            </div>
          </div>
          <span class="rounded-full border border-white/8 bg-white/5 px-2.5 py-1 text-[0.58rem] uppercase tracking-[0.22em] text-fog/52">
            {workspace.status}
          </span>
        </div>
        <p class="mt-3 text-sm leading-6 text-fog/68">{workspace.summary}</p>
        <div class="mt-3 truncate font-mono text-[0.7rem] text-fog/40">{workspace.path}</div>
      </button>
    {/each}
  </div>

  <div class="mt-5 flex items-center justify-between px-1">
    <div class="text-[0.66rem] font-medium uppercase tracking-[0.34em] text-fog/48">
      Sessions
    </div>
    <div class="truncate font-mono text-[0.68rem] uppercase tracking-[0.18em] text-fog/38">
      {selectedWorkspace.name}
    </div>
  </div>

  <div class="mt-3 flex-1 space-y-2 overflow-y-auto pr-1">
    {#each selectedWorkspace.sessions as session}
      <button
        type="button"
        class={`w-full rounded-[15px] border px-4 py-3 text-left transition ${
          session.id === selectedSessionId
            ? "border-white/14 bg-white/[0.065]"
            : "border-white/7 bg-white/[0.02] hover:border-white/12 hover:bg-white/[0.04]"
        }`}
        onclick={() => onSelectSession(session.id)}
      >
        <div class="flex items-center justify-between gap-3">
          <div class="min-w-0">
            <div class="truncate text-sm font-medium text-soft-ivory">{session.title}</div>
            <div class="mt-1 flex flex-wrap gap-2 font-mono text-[0.63rem] uppercase tracking-[0.18em] text-fog/42">
              <span>{session.model}</span>
              <span>{session.permission}</span>
            </div>
          </div>
          <span
            class={`rounded-full px-2.5 py-1 text-[0.58rem] uppercase tracking-[0.22em] ${
              session.status === "Live"
                ? "bg-misty-green/12 text-misty-green"
                : session.status === "Paused"
                  ? "bg-white/6 text-fog/62"
                  : "bg-signal-blue/12 text-signal-blue"
            }`}
          >
            {session.status}
          </span>
        </div>
        <div class="mt-2 text-[0.72rem] text-fog/48">{session.updatedAt}</div>
      </button>
    {/each}
  </div>

  <div class="mt-4 rounded-[16px] border border-white/8 bg-carbon-black/75 p-4">
    <div class="text-[0.66rem] uppercase tracking-[0.34em] text-fog/48">
      Runtime Stack
    </div>
    <div class="mt-3 flex flex-wrap gap-2">
      <span class="rounded-full border border-signal-blue/22 bg-signal-blue/10 px-2.5 py-1 font-mono text-[0.62rem] uppercase tracking-[0.18em] text-signal-blue">
        harness
      </span>
      <span class="rounded-full border border-white/8 bg-white/5 px-2.5 py-1 font-mono text-[0.62rem] uppercase tracking-[0.18em] text-fog/58">
        ollama
      </span>
      <span class="rounded-full border border-white/8 bg-white/5 px-2.5 py-1 font-mono text-[0.62rem] uppercase tracking-[0.18em] text-fog/58">
        gemma4
      </span>
    </div>
    <p class="mt-3 text-sm leading-6 text-fog/66">
      The sidebar now behaves more like an editor rail: quick switching,
      compact context, and persistent workspace state.
    </p>
  </div>
</aside>
