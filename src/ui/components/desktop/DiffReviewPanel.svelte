<script lang="ts">
  import Button from "$components/basic_elems/Button.svelte";
  import type { FileChange } from "$lib/mockDesktopData";

  interface Props {
    changes: FileChange[];
  }

  let { changes }: Props = $props();
</script>

<section
  class="flex min-h-0 flex-1 flex-col rounded-[30px] border border-white/8 bg-carbon-black/80 p-4 shadow-2xl backdrop-blur-md sm:p-5"
>
  <div class="flex items-center justify-between gap-3">
    <div>
      <div class="text-[0.68rem] uppercase tracking-[0.35em] text-fog/55">
        Review
      </div>
      <div class="mt-2 text-xl">Diff panel</div>
    </div>
    <div class="text-xs uppercase tracking-[0.22em] text-fog/45">
      {changes.length} files
    </div>
  </div>

  <div class="mt-5 flex-1 space-y-3 overflow-y-auto">
    {#if changes.length}
      {#each changes as change}
        <article class="rounded-[22px] border border-white/6 bg-white/3 p-4">
          <div class="flex flex-col gap-3 sm:flex-row sm:items-start sm:justify-between">
            <div class="min-w-0">
              <div class="break-all text-sm text-soft-ivory sm:truncate">{change.path}</div>
              <p class="mt-2 text-sm leading-6 text-fog/72">{change.summary}</p>
            </div>
            <div class="shrink-0 text-left text-xs uppercase tracking-[0.2em] sm:text-right">
              <div class="text-misty-green">+{change.additions}</div>
              <div class="mt-1 text-red-200/80">-{change.deletions}</div>
            </div>
          </div>
        </article>
      {/each}
    {:else}
      <div class="rounded-[22px] border border-dashed border-white/10 bg-white/2 p-4 text-sm leading-7 text-fog/65">
        No working-tree diff yet. Once `claw` edits files in this workspace, the review panel will summarize the changed paths here.
      </div>
    {/if}
  </div>

  <div class="mt-5 grid grid-cols-1 gap-3 sm:grid-cols-2">
    <Button label="Approve Changes" variant="gold" class="w-full" />
    <Button label="Keep Reviewing" variant="outline" class="w-full" />
  </div>
</section>
