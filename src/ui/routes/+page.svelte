<script lang="ts">
  import { Carta, MarkdownEditor } from "$lib/carta/index";
  import dracula from "shiki/dist/themes/dracula.mjs";
  import "$lib/carta/default.css";
  import DOMPurify from "isomorphic-dompurify";
  import Icon from "$components/basic_elems/Icon.svelte";
  import Button from "$components/basic_elems/Button.svelte";
  import TopToolbar from "$components/home/TopToolbar.svelte";

  let value = "";
  let rightContainer: HTMLDivElement;
  let isSignedIn = true;

  const carta = new Carta({
    theme: { light: "github-light", dark: "dracula" },
    sanitizer: DOMPurify.sanitize,
    shikiOptions: {
      themes: [dracula],
      langs: ["js", "ts", "bash", "json"],
    },
  });

  $: topOffset = isSignedIn ? "120px" : "60px";

  function focusEditor(e: MouseEvent) {
    const target = e.target as HTMLElement | null;
    if (!target || !rightContainer) return;

    // Don't steal focus if clicking toolbar, buttons, etc.
    if (
      target.closest(
        "button, a, input, select, textarea, .top-row, .action-row, .carta-toolbar"
      )
    ) {
      return;
    }

    // Simple & reliable — just like your original
    const editor = rightContainer.querySelector(
      ".carta-input textarea"
    ) as HTMLTextAreaElement | null;

    if (editor) {
      editor.focus();
      const len = editor.value.length;
      editor.setSelectionRange(len, len);
    }
  }
</script>

<div
  class="app-shell bg-deep-charcoal grid h-screen grid-cols-12 overflow-hidden"
>
  <div class="col-start-1 col-end-7 h-full min-h-0 p-4">
    <div
      class="bg-graphite h-full min-h-0 overflow-y-auto rounded-lg p-4 text-white"
    >
      <h2 class="mb-4 text-xl font-bold">Conversation</h2>
      <p>Placeholder for AI and user messages.</p>
    </div>
  </div>

  <div class="col-start-7 col-end-13 h-full min-h-0">
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div
      class="editor-host relative h-full min-h-0"
      bind:this={rightContainer}
      on:click={focusEditor}
      role="button"
      tabindex="0"
      style={`--editor-top-padding: ${topOffset};`}
    >
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

      <MarkdownEditor bind:value {carta} mode="tabs" placeholder="Type here!" />
    </div>
  </div>
</div>

<style lang="postcss">
  /* Your original styles — unchanged */
  :global(.carta-font-code) {
    font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
    font-size: 1.1rem;
    line-height: 1.1rem;
  }
  :global(.carta-editor) {
    height: 100%;
    min-height: 0;
    display: flex;
    flex-direction: column;
  }
  :global(.carta-toolbar) {
    flex-shrink: 0;
  }
  :global(.carta-wrapper) {
    flex: 1 1 auto;
    min-height: 0;
    overflow-y: auto;
    overflow-x: hidden;
  }
  :global(.carta-container) {
    min-height: 100%;
  }
  .editor-host :global(.carta-input),
  .editor-host :global(.carta-renderer) {
    margin-top: 1rem;
    margin-bottom: 1rem;
    padding-top: var(--editor-top-padding);
    padding-bottom: 200px;
    box-sizing: border-box;
  }
  .top-row,
  .action-row {
    position: absolute;
    left: 0;
    right: 0;
    z-index: 20;
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
