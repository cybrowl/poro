<script>
  import { Carta, MarkdownEditor } from "$lib/carta/index";
  import dracula from "shiki/dist/themes/dracula.mjs";
  import "$lib/carta/default.css";
  import DOMPurify from "isomorphic-dompurify";

  const carta = new Carta({
    /* 🔑 point at the theme’s *name* (must match dracula.name) */
    theme: { light: "github-light", dark: "dracula" },
    /* XSS protection */
    sanitizer: DOMPurify.sanitize,
    /* Load the theme + languages into Shiki */
    shikiOptions: {
      themes: [dracula], // array is fine
      langs: ["js", "ts", "bash", "json"], // 'md'/'markdown' already covered
    },
  });

  let value = "";
</script>

<div class="bg-deep-charcoal grid grid-cols-12 min-h-screen">
  <!-- Left half: Conversation (chat/responses from AI and user) - Placeholder for now -->
  <div class="col-start-1 col-end-7 w-full p-4">
    <!-- Add your conversation/chat component here later -->
    <div class="bg-graphite h-full rounded-lg p-4 text-white">
      <h2 class="text-xl font-bold mb-4">Conversation</h2>
      <p>Placeholder for AI and user messages.</p>
      <!-- Future: Message bubbles, input field, etc. -->
    </div>
  </div>

  <!-- Right half: MarkdownEditor -->
  <div class="col-start-7 col-end-13 w-full h-screen">
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
</style>
