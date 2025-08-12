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

      const firstRow = createRow("first-row", "Button 1");
      const secondRow = createRow("second-row", "Button 2");

      wrapper.insertBefore(firstRow, container);
      wrapper.insertBefore(secondRow, container);

      const secondHeight = secondRow.offsetHeight;
      rightContainer
        .querySelectorAll(".carta-input, .carta-renderer")
        .forEach((el) => {
          el.style.paddingTop = `${secondHeight}px`;
        });
    }
  });
</script>

<div class="bg-deep-charcoal grid grid-cols-12 min-h-screen">
  <div class="col-start-1 col-end-7 w-full p-4">
    <div class="bg-graphite h-full rounded-lg p-10 text-white">
      <h2 class="text-xl font-bold mb-4">Conversation</h2>
      <p>Placeholder for AI and user messages.</p>
    </div>
  </div>
  <div
    class="col-start-7 col-end-13 w-full h-screen"
    bind:this={rightContainer}
  >
    <div class="h-full">
      <MarkdownEditor
        bind:value
        {carta}
        mode="tabs"
        placeholder="Enter command here"
      />
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
    background: #282a36;
  }

  :global(.second-row) {
    position: sticky;
    top: 0;
    z-index: 10;
  }

  :global(.custom-button) {
    background: #44475a;
    color: #f8f8f2;
    padding: 0.5rem 1rem;
    border-radius: 4px;
    border: none;
    cursor: pointer;
  }

  :global(.custom-button:hover) {
    background: #6272a4;
  }
</style>
