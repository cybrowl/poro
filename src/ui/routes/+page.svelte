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

<div class="bg-deep-charcoal grid grid-cols-12 min-h-screen gap-y-2 pt-6">
  <div class="col-start-2 col-end-12 w-full">
    <!-- no h-screen here -->
    <MarkdownEditor bind:value {carta} mode="tabs" placeholder="Type here!" />
  </div>
</div>

<style lang="postcss">
  :global(.carta-font-code) {
    font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
    font-size: 1.1rem;
    line-height: 1.1rem;
  }
</style>
