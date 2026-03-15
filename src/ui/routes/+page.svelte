<script lang="ts">
  import { onMount } from "svelte";
  import Icon from "$components/basic_elems/Icon.svelte";
  import Button from "$components/basic_elems/Button.svelte";
  import TopToolbar from "$components/home/TopToolbar.svelte";
  import PoroMarkdownEditor from "$lib/PoroMarkdownEditor.svelte";

  export const ssr = false; // ← REQUIRED for Tiptap (fixes build error)

  let value = "";
  let rightContainer: HTMLDivElement;
  let isSignedIn = true;
  let poroEditor: any;

  $: topOffset = isSignedIn ? "200px" : "140px";

  function focusEditor(e: MouseEvent) {
    const target = e.target as HTMLElement | null;
    if (!target || !rightContainer) return;
    if (target.closest("button, a, input, select, .top-row, .action-row"))
      return;

    setTimeout(() => {
      if (poroEditor && poroEditor.focus) poroEditor.focus();
    }, 10);
  }
</script>

<div class="bg-deep-charcoal grid grid-cols-12 min-h-screen overflow-hidden">
  <!-- LEFT PANEL (unchanged) -->
  <div class="col-start-1 col-end-7 w-full p-4 overflow-y-auto">
    <div class="h-full min-h-150 rounded-lg p-10 text-white">
      <h2 class="mb-4 text-xl font-bold">Conversation</h2>
      <p>
        Placeholder for AI and user messages. This panel scrolls independently.
      </p>
    </div>
  </div>

  <!-- RIGHT PANEL (your exact layout) -->
  <!-- svelte-ignore a11y_no_static_element_interactions a11y_click_events_have_key_events -->
  <div
    class="col-start-7 bg-dark-slate col-end-13 flex flex-col h-screen"
    bind:this={rightContainer}
    on:click={focusEditor}
  >
    <div class="relative shrink-0 z-30">
      {#if !isSignedIn}
        <div class="top-row">
          <Button label="Login / Signup" variant="dark" />
        </div>
      {:else}
        <div class="top-row">
          <TopToolbar />
        </div>
      {/if}
      <div class="action-row">
        <Icon
          name="submit"
          size="3.6rem"
          viewSize={{ width: 59, height: 56 }}
        />
      </div>
    </div>

    <div
      class="flex-1 overflow-y-auto editor-host"
      style={`--editor-top-padding: ${topOffset};`}
    >
      <PoroMarkdownEditor bind:value bind:this={poroEditor} />
    </div>
  </div>
</div>

<style lang="postcss">
  .editor-host :global(.ProseMirror),
  .editor-host :global(textarea) {
    margin-top: 1rem;
    margin-bottom: 1rem;
    padding-top: var(--editor-top-padding);
    padding-bottom: 220px;
    box-sizing: border-box;
  }
  .top-row,
  .action-row {
    position: absolute;
    left: 0;
    right: 0;
    z-index: 30;
    display: flex;
    justify-content: flex-end;
    padding: 0.5rem 1rem 2rem;
    pointer-events: none;
  }
  .top-row > :global(*),
  .action-row > :global(*) {
    pointer-events: auto;
  }
  .top-row {
    top: 0;
  }
  .action-row {
    top: 60px;
  }
</style>
