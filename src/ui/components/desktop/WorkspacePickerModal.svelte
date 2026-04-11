<script lang="ts">
  import Button from "$components/basic_elems/Button.svelte";
  import type { WorkspaceRecord } from "$lib/mockDesktopData";
  import { fade, fly, scale } from "svelte/transition";

  interface Props {
    open: boolean;
    workspaces: WorkspaceRecord[];
    selectedWorkspaceId: string;
    onSelectWorkspace: (id: string) => void;
    onOpenFromDisk: () => void;
    onClose: () => void;
  }

  let {
    open,
    workspaces,
    selectedWorkspaceId,
    onSelectWorkspace,
    onOpenFromDisk,
    onClose,
  }: Props = $props();
</script>

{#if open}
  <button
    class="ui-overlay fixed inset-0 z-40"
    onclick={onClose}
    aria-label="Close workspace picker"
    type="button"
    transition:fade={{ duration: 140 }}
  ></button>

  <div
    class="fixed left-1/2 top-1/2 z-50 w-[min(960px,calc(100vw-2rem))] -translate-x-1/2 -translate-y-1/2"
    transition:fly={{ duration: 180, y: 18 }}
  >
    <section
      class="ui-sheet p-5"
      transition:scale={{ duration: 160, start: 0.98 }}
    >
      <div class="flex flex-col gap-5 xl:flex-row xl:items-start xl:justify-between">
        <div>
          <div class="ui-section-label">Workspace Switcher</div>
          <h3 class="mt-3 type-heading-1 tracking-[-0.05em] text-soft-ivory">
            Open another codebase without losing the session flow
          </h3>
          <p class="mt-3 max-w-2xl type-body-4 text-fog/68">
            This modal is now styled closer to an editor switcher: recent repos,
            branch context, and just enough summary to choose quickly.
          </p>
        </div>

        <div class="flex gap-2">
          <Button label="Open from Disk" variant="gold" height="h-10" onclick={onOpenFromDisk} />
          <Button label="Close" variant="outline" height="h-10" onclick={onClose} />
        </div>
      </div>

      <div class="mt-6 grid gap-3 xl:grid-cols-2">
        {#each workspaces as workspace}
          <button
            type="button"
            class={`ui-panel p-4 text-left transition ${
              workspace.id === selectedWorkspaceId
                ? "border-accent-gold/35 bg-accent-gold/10"
                : "hover:border-white/14 hover:bg-white/[0.05]"
            }`}
            onclick={() => onSelectWorkspace(workspace.id)}
          >
            <div class="flex items-start justify-between gap-4">
              <div class="min-w-0">
                <div class="type-heading-2 leading-none text-soft-ivory">{workspace.name}</div>
                <div class="code-font mt-2 text-[0.68rem] uppercase tracking-[0.18em] text-fog/42">
                  {workspace.branch}
                </div>
              </div>
              <span class="ui-chip ui-chip-neutral">
                {workspace.status}
              </span>
            </div>

            <p class="mt-4 type-body-4 text-fog/68">{workspace.summary}</p>

            <div class="code-font mt-4 flex flex-wrap gap-3 text-[0.66rem] uppercase tracking-[0.16em] text-fog/40">
              <span>{workspace.lastOpened}</span>
              <span>{workspace.sessions.length} session(s)</span>
            </div>

            <div class="code-font mt-4 truncate text-[0.72rem] text-fog/46">{workspace.path}</div>
          </button>
        {/each}
      </div>
    </section>
  </div>
{/if}
