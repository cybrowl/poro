<script>
  import { Carta, MarkdownEditor } from "$lib/carta/index";
  import "$lib/carta/default.css";
  import DOMPurify from "isomorphic-dompurify";
  import { onMount } from "svelte";

  const carta = new Carta({
    theme: { light: "solarized-light", dark: "monokai" },
    sanitizer: DOMPurify.sanitize,
    shikiOptions: {
      langs: ["js", "ts", "bash", "json"],
    },
  });
  let value = "";
  let rightContainer;

  onMount(() => {
    rightContainer.addEventListener("click", (e) => {
      // Avoid interfering with toolbar clicks
      if (e.target.closest(".carta-toolbar")) return;

      const input = rightContainer.querySelector(".carta-input");
      if (input) {
        input.focus();
        // Place cursor at the end
        const range = document.createRange();
        range.selectNodeContents(input);
        range.collapse(false);
        const sel = window.getSelection();
        sel.removeAllRanges();
        sel.addRange(range);
      }
    });
  });
</script>

<div class="bg-deep-charcoal grid grid-cols-12 min-h-screen">
  <!-- Left half: Conversation (chat/responses from AI and user) - Placeholder for now -->
  <div class="col-start-1 col-end-7 w-full p-4">
    <!-- Add your conversation/chat component here later -->
    <div class="bg-graphite h-full rounded-lg p-10 text-white">
      <h2 class="text-xl font-bold mb-4">Conversation</h2>
      <p>Placeholder for AI and user messages.</p>
      <!-- Future: Message bubbles, input field, etc. -->
    </div>
  </div>
  <!-- Right half: MarkdownEditor -->
  <div
    class="col-start-7 col-end-13 w-full h-screen"
    bind:this={rightContainer}
  >
    <div class="h-full">
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
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  :global(.carta-toolbar) {
    flex-shrink: 0;
  }

  :global(.carta-wrapper) {
    margin-top: 2rem;
    flex-grow: 1;
    overflow: auto;
  }

  :global(.carta-container) {
    height: 100%;
  }

  :global(.carta-input, .carta-renderer) {
    margin-bottom: 1rem;
    margin-top: 1rem;
    padding-bottom: 200px;
    box-sizing: border-box;
  }
</style>
