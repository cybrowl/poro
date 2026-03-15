<script lang="ts">
  import { Carta, MarkdownEditor } from "$lib/carta/index";
  import dracula from "shiki/dist/themes/dracula.mjs";
  import "$lib/carta/default.css";
  import DOMPurify from "isomorphic-dompurify";
  import Icon from "$components/basic_elems/Icon.svelte";
  import Button from "$components/basic_elems/Button.svelte";
  import TopToolbar from "$components/home/TopToolbar.svelte";
  import { onMount, mount } from "svelte";
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
  let rightContainer;
  let loginRow;
  let toolbarRow;
  let isSignedIn = true;
  onMount(() => {
    rightContainer.addEventListener("click", (e) => {
      if (e.target.closest(".carta-toolbar, .first-row, .second-row")) return;
      const input = rightContainer.querySelector(".carta-input");
      if (input) {
        input.focus();
        const range = document.createRange();
        range.selectNodeContents(input);
        range.collapse(false);
        const sel = window.getSelection();
        sel.removeAllRanges();
        sel.addRange(range);
      }
    });
    const wrapper = rightContainer.querySelector(".carta-wrapper");
    const container = rightContainer.querySelector(".carta-container");
    if (wrapper && container) {
      loginRow = document.createElement("div");
      loginRow.classList.add("first-row");
      loginRow.innerHTML = "";
      mount(Button, {
        target: loginRow,
        props: {
          label: "Login / Signup",
          variant: "dark",
        },
      });

      toolbarRow = document.createElement("div");
      toolbarRow.classList.add("first-row");
      toolbarRow.innerHTML = "";
      mount(TopToolbar, {
        target: toolbarRow,
        props: {},
      });

      const secondRow = document.createElement("div");
      secondRow.classList.add("second-row");
      mount(Icon, {
        target: secondRow,
        props: {
          name: "submit",
          size: "3.6rem",
          viewSize: { width: 59, height: 56 },
        },
      });
      wrapper.insertBefore(secondRow, container);
      wrapper.insertBefore(toolbarRow, secondRow);
      wrapper.insertBefore(loginRow, secondRow);
      updatePadding();
    }
  });
  function updatePadding() {
    let totalHeight = 0;
    rightContainer
      ?.querySelectorAll(".first-row, .second-row")
      .forEach((row) => {
        if (row.style.display !== "none") {
          totalHeight += row.offsetHeight || 0;
        }
      });
    rightContainer
      ?.querySelectorAll(".carta-input, .carta-renderer")
      ?.forEach((el) => {
        el.style.paddingTop = `${totalHeight}px`;
      });
  }

  $: if (loginRow && toolbarRow) {
    loginRow.style.display = isSignedIn ? "none" : "flex";
    toolbarRow.style.display = isSignedIn ? "flex" : "none";
    updatePadding();
  }
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
  <div
    class="col-start-7 col-end-13 w-full h-screen"
    bind:this={rightContainer}
  >
    <div class="h-full relative">
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

  :global(.first-row),
  :global(.second-row) {
    display: flex;
    justify-content: flex-end;
    padding: 0.5rem 1rem 2rem;
    position: sticky;
    z-index: 10;
  }

  :global(.first-row) {
    top: 0;
  }

  :global(.second-row) {
    top: 60px;
  }
</style>
