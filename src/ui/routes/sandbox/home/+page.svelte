<script>
  import { Carta, MarkdownEditor } from "$lib/carta/index";
  import Icon from "$components/basic_elems/Icon.svelte";
  import Button from "$components/basic_elems/Button.svelte";
  import "$lib/carta/default.css";
  import DOMPurify from "isomorphic-dompurify";
  import { onMount, mount } from "svelte";

  export let isLoading = false;
  export let error = null;
  export let userName = "";
  export let isSignedIn = false;

  const carta = new Carta({
    theme: { light: "solarized-light", dark: "monokai" },
    sanitizer: DOMPurify.sanitize,
    shikiOptions: {
      langs: ["js", "ts", "bash", "json"],
    },
  });
  let value = `Hello, ${userName}!`;
  let rightContainer;
  let firstRow;
  onMount(() => {
    rightContainer.addEventListener("click", (e) => {
      if (e.target.closest(".carta-toolbar")) return;
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
      const createRow = (className, buttonText) => {
        const row = document.createElement("div");
        row.classList.add(className);
        const button = document.createElement("button");
        button.textContent = buttonText;
        button.classList.add("custom-button");
        row.appendChild(button);
        return row;
      };
      firstRow = createRow("first-row", "Button 1");
      firstRow.innerHTML = "";
      mount(Button, {
        target: firstRow,
        props: {
          label: "Login / Signup",
          variant: "dark",
        },
      });
      wrapper.insertBefore(firstRow, container);

      const secondRow = document.createElement("div");
      secondRow.classList.add("second-row");
      const button2 = document.createElement("button");
      button2.classList.add("custom-button");
      secondRow.appendChild(button2);
      mount(Icon, {
        target: button2,
        props: {
          name: "submit",
          size: "3.6rem",
          viewSize: { width: 59, height: 56 },
        },
      });
      wrapper.insertBefore(secondRow, container);
      updatePadding();
    }
  });

  function updatePadding() {
    const secondRow = rightContainer?.querySelector(".second-row");
    if (secondRow) {
      const secondHeight = secondRow.offsetHeight;
      rightContainer
        ?.querySelectorAll(".carta-input, .carta-renderer")
        ?.forEach((el) => {
          el.style.paddingTop = `${secondHeight}px`;
        });
    }
  }

  $: if (firstRow) {
    firstRow.style.display = isSignedIn ? "none" : "flex";
    updatePadding();
  }
</script>

<div class="bg-deep-charcoal grid grid-cols-12 min-h-screen">
  <div class="col-start-1 col-end-7 w-full p-4">
    <div class="h-full rounded-lg p-10 text-white">
      {#if isLoading}
        <p class="text-xl font-bold mb-4">Loading...</p>
      {:else if error}
        <h2 class="text-xl font-bold mb-4">Error</h2>
        <p>{error}</p>
      {:else}
        <h2 class="text-xl font-bold mb-4">
          Conversation for {userName || "User"}
        </h2>
        <p>Placeholder for AI and user messages.</p>
      {/if}
    </div>
  </div>
  <div
    class="col-start-7 col-end-13 w-full h-screen"
    bind:this={rightContainer}
  >
    <div class="h-full relative">
      <MarkdownEditor
        bind:value
        {carta}
        mode="tabs"
        placeholder="Enter command here"
      />
      {#if isLoading}
        <div
          class="absolute inset-0 bg-black/50 flex items-center justify-center text-white"
        >
          <p>Loading editor...</p>
        </div>
      {/if}
    </div>
  </div>
</div>

<style lang="postcss">
  :global(.carta-font-code) {
    font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
    font-size: 1rem;
    line-height: 2rem;
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
  :global(.first-row),
  :global(.second-row) {
    display: flex;
    justify-content: flex-end;
    padding: 0.5rem 1rem;
  }
  :global(.second-row) {
    position: sticky;
    top: 0;
    z-index: 10;
  }
  :global(.custom-button) {
    padding: 0.5rem 1rem;
    border-radius: 4px;
    border: none;
    cursor: pointer;
  }
</style>
