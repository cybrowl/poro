<script lang="ts">
  import { fade, fly, scale } from "svelte/transition";
  import type { WorkspaceRecord } from "$lib/mockDesktopData";

  interface Props {
    open: boolean;
    workspaces: WorkspaceRecord[];
    selectedWorkspaceId: string;
    selectedSessionId: string;
    onSelectWorkspace: (id: string) => void | Promise<void>;
    onSelectSession: (workspaceId: string, sessionId: string) => void | Promise<void>;
    onDeleteSession: (sessionId: string) => void | Promise<void>;
    onOpenFromDisk: () => void | Promise<void>;
    onClose: () => void;
  }

  let {
    open,
    workspaces,
    selectedWorkspaceId,
    selectedSessionId,
    onSelectWorkspace,
    onSelectSession,
    onDeleteSession,
    onOpenFromDisk,
    onClose,
  }: Props = $props();
</script>

{#if open}
  <button
    class="ui-overlay fixed inset-0 z-40"
    onclick={onClose}
    aria-label="Close session switcher"
    type="button"
    transition:fade={{ duration: 140 }}
  ></button>

  <div
    class="fixed left-1/2 top-1/2 z-50 w-[min(940px,calc(100vw-2rem))] -translate-x-1/2 -translate-y-1/2"
    transition:fly={{ duration: 180, y: 18 }}
  >
    <section
      class="ui-sheet overflow-hidden p-5"
      transition:scale={{ duration: 160, start: 0.98 }}
    >
      <div class="flex items-start justify-between gap-4">
        <div>
          <div class="ui-section-label">Workspace Switcher</div>
          <h3 class="mt-3 type-heading-1 tracking-[-0.05em] text-soft-ivory">
            Open a codebase or jump between recent sessions without crowding the rail
          </h3>
          <p class="mt-3 max-w-2xl type-body-4 text-fog/68">
            Open a workspace, resume a prior session, or remove an older local session from the list.
          </p>
        </div>

        <div class="flex items-center gap-2">
          <button
            type="button"
            class="rounded-md border border-accent-gold/24 bg-accent-gold/10 px-3 py-2 text-sm text-accent-gold transition hover:bg-accent-gold/16"
            onclick={onOpenFromDisk}
          >
            Open from disk
          </button>
          <button
            type="button"
            class="rounded-md px-3 py-2 text-sm text-fog/62 transition hover:bg-white/[0.04] hover:text-soft-ivory"
            onclick={onClose}
          >
            Close
          </button>
        </div>
      </div>

      <div class="mt-6 max-h-[70vh] space-y-4 overflow-y-auto pr-1">
        {#each workspaces as workspace}
          <section class="ui-panel p-4">
            <div class="flex items-start justify-between gap-4">
              <button
                type="button"
                class={`min-w-0 text-left transition ${
                  workspace.id === selectedWorkspaceId
                    ? "text-soft-ivory"
                    : "text-fog/72 hover:text-soft-ivory"
                }`}
                onclick={() => onSelectWorkspace(workspace.id)}
              >
                <div class="type-heading-2 leading-none">{workspace.name}</div>
                <div class="mt-2 code-font text-[0.68rem] uppercase tracking-[0.18em] text-fog/42">
                  {workspace.branch}
                </div>
              </button>

              <div class="flex items-center gap-2">
                <span class="ui-chip ui-chip-neutral">
                  {workspace.lastOpened}
                </span>
                {#if workspace.id === selectedWorkspaceId}
                  <span class="ui-chip ui-chip-accent">
                    Active
                  </span>
                {/if}
              </div>
            </div>

            <div class="mt-4 space-y-2">
              {#each workspace.sessions as session}
                {@const canDelete =
                  !!session.sessionPath ||
                  session.source !== "mock" ||
                  workspace.sessions.length > 1}
                <div
                  class={`group flex items-center gap-2 rounded-xl border px-3 py-3 transition ${
                    workspace.id === selectedWorkspaceId && session.id === selectedSessionId
                      ? "border-accent-gold/25 bg-accent-gold/[0.08]"
                      : "border-white/6 bg-white/[0.02] hover:border-white/12 hover:bg-white/[0.04]"
                  }`}
                >
                  <button
                    type="button"
                    class="flex min-w-0 flex-1 items-center justify-between gap-4 text-left"
                    onclick={() => onSelectSession(workspace.id, session.id)}
                  >
                    <div class="min-w-0">
                      <div class="truncate type-body-4 text-soft-ivory">{session.title}</div>
                      <div class="mt-1 truncate text-[11px] text-fog/38">
                        {session.model}
                      </div>
                    </div>
                    <div class="shrink-0 text-[11px] text-fog/34">{session.updatedAt}</div>
                  </button>

                  {#if canDelete}
                    <button
                      type="button"
                      class="flex h-8 w-8 shrink-0 items-center justify-center rounded-md text-sm text-fog/28 transition hover:bg-white/[0.05] hover:text-soft-ivory"
                      aria-label={`Delete ${session.title}`}
                      title={`Delete ${session.title}`}
                      onclick={(event) => {
                        event.stopPropagation();
                        void onDeleteSession(session.id);
                      }}
                    >
                      ×
                    </button>
                  {/if}
                </div>
              {/each}
            </div>
          </section>
        {/each}
      </div>
    </section>
  </div>
{/if}
