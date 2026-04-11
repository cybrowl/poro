<script lang="ts">
  import { fade, fly, scale } from "svelte/transition";
  import type { WorkspaceRecord } from "$lib/mockDesktopData";

  function sessionLabel(title: string) {
    const trimmed = title.trim();
    return trimmed.length ? trimmed : "New session";
  }

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
    class="fixed left-1/2 top-1/2 z-50 w-[min(900px,calc(100vw-2rem))] -translate-x-1/2 -translate-y-1/2"
    transition:fly={{ duration: 180, y: 18 }}
  >
    <section
      class="ui-sheet overflow-hidden px-5 pb-5 pt-4"
      style="background-color: #111113; border-color: rgb(255 255 255 / 0.08);"
      transition:scale={{ duration: 160, start: 0.98 }}
    >
      <div class="flex items-start justify-between gap-4 border-b border-white/[0.05] pb-4">
        <div class="max-w-[34rem]">
          <div class="ui-section-label">Workspace Switcher</div>
          <h3 class="mt-2 text-[1.1rem] font-medium leading-[1.2] tracking-[-0.03em] text-soft-ivory">
            Workspaces and recent sessions
          </h3>
          <p class="mt-2 text-[0.875rem] leading-6 text-fog/56">
            Open a workspace, jump into a recent session, or remove older local session history.
          </p>
        </div>

        <div class="flex items-center gap-2">
          <button
            type="button"
            class="rounded-xl border border-white/10 bg-white/[0.02] px-3 py-2 text-[0.8125rem] text-fog/76 transition hover:border-white/14 hover:bg-white/[0.04] hover:text-soft-ivory"
            onclick={onOpenFromDisk}
          >
            Open from disk
          </button>
          <button
            type="button"
            class="rounded-xl px-3 py-2 text-[0.8125rem] text-fog/52 transition hover:bg-white/[0.03] hover:text-soft-ivory"
            onclick={onClose}
          >
            Close
          </button>
        </div>
      </div>

      <div class="ui-scrollbar-hidden mt-4 max-h-[72vh] space-y-3 overflow-y-auto pr-1">
        {#each workspaces as workspace}
          <section
            class="rounded-[22px] border border-white/[0.05] p-4"
            style="background-color: #0c0c0d;"
          >
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
                <div class="text-[1rem] font-medium leading-none tracking-[-0.02em]">{workspace.name}</div>
                <div class="mt-2 code-font text-[0.68rem] uppercase tracking-[0.18em] text-fog/34">
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
                  class={`group flex items-center gap-2 rounded-[16px] border px-3 py-3 transition ${
                    workspace.id === selectedWorkspaceId && session.id === selectedSessionId
                      ? "border-accent-gold/18 bg-accent-gold/[0.05]"
                      : "border-white/[0.05] bg-[#111113] hover:border-white/[0.1] hover:bg-white/[0.025]"
                  }`}
                >
                  <button
                    type="button"
                    class="flex min-w-0 flex-1 items-center justify-between gap-4 text-left"
                    onclick={() => onSelectSession(workspace.id, session.id)}
                  >
                    <div class="min-w-0">
                      <div class="truncate text-[0.875rem] leading-6 text-soft-ivory">
                        {sessionLabel(session.title)}
                      </div>
                      <div class="mt-1 truncate text-[11px] text-fog/34">
                        {session.model}
                      </div>
                    </div>
                    <div class="shrink-0 text-[11px] text-fog/30">{session.updatedAt}</div>
                  </button>

                  {#if canDelete}
                    <button
                      type="button"
                      class="flex h-8 w-8 shrink-0 items-center justify-center rounded-md text-sm text-fog/24 opacity-0 transition hover:bg-white/[0.05] hover:text-soft-ivory group-hover:opacity-100"
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
