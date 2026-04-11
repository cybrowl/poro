<script lang="ts">
  import type { WorkspaceRecord } from "$lib/mockDesktopData";
  import logoUrl from "../../assets/logo.svg";

  interface Props {
    workspaces: WorkspaceRecord[];
    selectedWorkspaceId: string;
    selectedSessionId: string;
    onSelectWorkspace: (id: string) => void;
    onSelectSession: (id: string) => void;
    onDeleteSession: (id: string) => void;
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
    onDeleteSession,
    onPickWorkspace,
    onOpenWorkspacePicker,
    onOpenSettings,
  }: Props = $props();
</script>

<aside
  class="flex min-h-0 w-full shrink-0 flex-col border-r border-white/6 bg-deep-charcoal lg:h-screen lg:w-[248px]"
>
  <div class="px-4 py-3">
    <img src={logoUrl} alt="Poro" class="h-9 w-auto object-contain" />
  </div>

  <div class="px-3 pb-2">
    <div class="space-y-1">
      <button
        type="button"
        class="flex w-full items-center rounded-md px-3 py-2 text-left type-body-4 text-fog/72 transition hover:bg-white/[0.035] hover:text-soft-ivory"
        onclick={onPickWorkspace}
      >
        Open workspace
      </button>
      <button
        type="button"
        class="flex w-full items-center rounded-md px-3 py-2 text-left type-body-4 text-fog/72 transition hover:bg-white/[0.035] hover:text-soft-ivory"
        onclick={onOpenWorkspacePicker}
      >
        Search
      </button>
      <button
        type="button"
        class="flex w-full items-center rounded-md px-3 py-2 text-left type-body-4 text-fog/72 transition hover:bg-white/[0.035] hover:text-soft-ivory"
        onclick={onOpenSettings}
      >
        Settings
      </button>
    </div>
  </div>

  <div class="border-t border-white/6"></div>

  <div class="px-4 pb-2 pt-3 type-label text-fog/32">
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
                ? "bg-white/[0.055] text-soft-ivory"
                : "text-fog/64 hover:bg-white/[0.035] hover:text-soft-ivory"
            }`}
            onclick={() => onSelectWorkspace(workspace.id)}
          >
            <div class="min-w-0">
              <div class="truncate type-body-4">{workspace.name}</div>
              <div class="mt-0.5 truncate text-[11px] text-fog/34">
                {workspace.branch}
              </div>
            </div>
            <div class="shrink-0 pl-3 text-[11px] text-fog/34">{workspace.lastOpened}</div>
          </button>

          {#if workspace.id === selectedWorkspaceId}
            <div class="ml-2 space-y-1 border-l border-white/6 pl-2">
              {#each workspace.sessions as session}
                {@const canDelete =
                  !!session.sessionPath ||
                  session.source !== "mock" ||
                  workspace.sessions.length > 1}
                <div
                  class={`group flex items-center gap-1 rounded-md transition ${
                    session.id === selectedSessionId ? "bg-white/[0.045]" : "hover:bg-white/[0.03]"
                  }`}
                >
                  <button
                    type="button"
                    class={`flex min-w-0 flex-1 items-center justify-between rounded-md px-3 py-2 text-left transition ${
                      session.id === selectedSessionId
                        ? "text-soft-ivory"
                        : "text-fog/58 group-hover:text-soft-ivory"
                    }`}
                    onclick={() => onSelectSession(session.id)}
                  >
                    <div class="min-w-0">
                      <div class="truncate type-body-5 text-fog/84">{session.title}</div>
                      <div class="mt-0.5 truncate text-[11px] text-fog/32">{session.model}</div>
                    </div>
                    <div class="shrink-0 pl-3 text-[11px] text-fog/34">{session.updatedAt}</div>
                  </button>

                  {#if canDelete}
                    <button
                      type="button"
                      class={`mr-1 flex h-8 w-8 shrink-0 items-center justify-center rounded-md text-sm transition ${
                        session.id === selectedSessionId
                          ? "text-fog/44 hover:bg-white/[0.05] hover:text-soft-ivory"
                          : "text-fog/22 opacity-0 group-hover:opacity-100 hover:bg-white/[0.04] hover:text-soft-ivory focus-visible:opacity-100"
                      }`}
                      aria-label={`Delete ${session.title}`}
                      title={`Delete ${session.title}`}
                      onclick={(event) => {
                        event.stopPropagation();
                        onDeleteSession(session.id);
                      }}
                    >
                      ×
                    </button>
                  {/if}
                </div>
              {/each}
            </div>
          {/if}
        </div>
      {/each}
    </div>
  </div>

</aside>
