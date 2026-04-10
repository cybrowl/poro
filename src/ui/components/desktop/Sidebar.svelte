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
  class="flex min-h-0 w-full shrink-0 flex-col border-r border-white/8 bg-[#151a1f] lg:w-[248px]"
>
  <div class="flex items-center justify-between px-4 py-3">
    <div class="flex items-center gap-2">
      <div class="h-2 w-2 rounded-full bg-signal-blue shadow-[0_0_14px_rgba(78,161,255,0.75)]"></div>
      <div class="text-[0.76rem] font-medium tracking-[0.18em] text-fog/76">PORO</div>
    </div>
    <span class="rounded-md border border-white/8 bg-white/[0.03] px-2 py-1 text-[0.62rem] tracking-[0.12em] text-fog/46">
      desktop
    </span>
  </div>

  <div class="px-3 pb-2">
    <div class="space-y-1">
      <button
        type="button"
        class="flex w-full items-center rounded-md px-3 py-2 text-left text-[0.92rem] text-fog/76 transition hover:bg-white/[0.04] hover:text-soft-ivory"
        onclick={onPickWorkspace}
      >
        Open workspace
      </button>
      <button
        type="button"
        class="flex w-full items-center rounded-md px-3 py-2 text-left text-[0.92rem] text-fog/76 transition hover:bg-white/[0.04] hover:text-soft-ivory"
        onclick={onOpenWorkspacePicker}
      >
        Search
      </button>
      <button
        type="button"
        class="flex w-full items-center rounded-md px-3 py-2 text-left text-[0.92rem] text-fog/76 transition hover:bg-white/[0.04] hover:text-soft-ivory"
        onclick={onOpenSettings}
      >
        Settings
      </button>
    </div>
  </div>

  <div class="border-t border-white/8"></div>

  <div class="px-4 pb-2 pt-3 text-[0.7rem] uppercase tracking-[0.18em] text-fog/34">
    Threads
  </div>

  <div class="min-h-0 flex-1 overflow-y-auto px-2 pb-3">
    <div class="space-y-1">
      {#each workspaces as workspace}
        <div class="space-y-1">
          <button
            type="button"
            class={`flex w-full items-center justify-between rounded-md px-3 py-2 text-left transition ${
              workspace.id === selectedWorkspaceId
                ? "bg-white/[0.08] text-soft-ivory"
                : "text-fog/68 hover:bg-white/[0.04] hover:text-soft-ivory"
            }`}
            onclick={() => onSelectWorkspace(workspace.id)}
          >
            <div class="min-w-0">
              <div class="truncate text-[0.9rem]">{workspace.name}</div>
              <div class="mt-0.5 truncate text-[0.68rem] uppercase tracking-[0.14em] text-fog/34">
                {workspace.branch}
              </div>
            </div>
            <div class="shrink-0 pl-3 text-[0.72rem] text-fog/36">{workspace.lastOpened}</div>
          </button>

          {#if workspace.id === selectedWorkspaceId}
            <div class="ml-2 space-y-1 border-l border-white/8 pl-2">
              {#each workspace.sessions as session}
                <button
                  type="button"
                  class={`flex w-full items-center justify-between rounded-md px-3 py-2 text-left transition ${
                    session.id === selectedSessionId
                      ? "bg-white/[0.06] text-soft-ivory"
                      : "text-fog/62 hover:bg-white/[0.035] hover:text-soft-ivory"
                  }`}
                  onclick={() => onSelectSession(session.id)}
                >
                  <div class="min-w-0">
                    <div class="truncate text-[0.86rem]">{session.title}</div>
                    <div class="mt-0.5 truncate text-[0.68rem] text-fog/34">{session.model}</div>
                  </div>
                  <div class="shrink-0 pl-3 text-[0.72rem] text-fog/36">{session.updatedAt}</div>
                </button>
              {/each}
            </div>
          {/if}
        </div>
      {/each}
    </div>
  </div>

  <div class="border-t border-white/8 px-3 py-3">
    <Button
      label={`Settings • ${selectedWorkspace.name}`}
      variant="ghost"
      class="w-full justify-start"
      height="h-9"
      onclick={onOpenSettings}
    />
  </div>
</aside>
