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

  $: topOffset = isSignedIn ? "200px" : "140px";

  // ──────────────────────────────────────────────────────────────
  // Click ANYWHERE on the right panel → focus editor (even when empty)
  // ──────────────────────────────────────────────────────────────
  function focusEditor(e: MouseEvent) {
    const target = e.target as HTMLElement | null;
    if (!target || !rightContainer) return;

    // Skip toolbar, buttons, action row
    if (
      target.closest(
        "button, a, input, select, textarea, .top-row, .action-row, .carta-toolbar"
      )
    ) {
      return;
    }

    // Official Carta way + fallback (works when empty)
    carta.focus();

    setTimeout(() => {
      const textarea = rightContainer.querySelector(
        ".carta-input textarea"
      ) as HTMLTextAreaElement | null;

      if (textarea) {
        textarea.focus();
        const len = textarea.value.length;
        textarea.setSelectionRange(len, len);
      }
    }, 10);
  }
</script>

<div class="bg-deep-charcoal grid grid-cols-12 min-h-screen overflow-hidden">
  <!-- LEFT PANEL (independent scroll) -->
  <div class="col-start-1 col-end-7 w-full p-4 overflow-y-auto">
    <div class="h-full min-h-150 rounded-lg p-10 text-white bg-dark-slate/50">
      <h2 class="mb-4 text-xl font-bold">Conversation</h2>
      <p>
        Placeholder for AI and user messages. This panel scrolls independently.
      </p>
    </div>
  </div>

  <!-- RIGHT PANEL (fixed toolbar + scrollable editor only) -->
  <!-- svelte-ignore a11y_no_static_element_interactions a11y_click_events_have_key_events -->
  <div
    class="col-start-7 col-end-13 flex flex-col h-screen"
    bind:this={rightContainer}
    on:click={focusEditor}
  >
    <!-- Fixed header (toolbar + submit icon) -->
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

    <!-- Scrollable editor area only -->
    <div
      class="flex-1 overflow-y-auto editor-host"
      style={`--editor-top-padding: ${topOffset};`}
    >
      <MarkdownEditor bind:value {carta} mode="tabs" placeholder="Type here!" />
    </div>
  </div>
</div>

<style lang="postcss">
  :global(.carta-font-code) {
    font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
    font-size: 1.1rem;
    line-height: 1.1rem;
  }

  :global(.carta-editor) {
    height: 100% !important;
    min-height: 0;
    display: flex;
    flex-direction: column;
  }

  :global(.carta-toolbar) {
    flex-shrink: 0;
  }

  :global(.carta-wrapper) {
    flex: 1 1 auto !important;
    min-height: 0;
    overflow-y: auto;
  }

  :global(.carta-container) {
    min-height: 100%;
  }

  .editor-host :global(.carta-input),
  .editor-host :global(.carta-renderer) {
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
