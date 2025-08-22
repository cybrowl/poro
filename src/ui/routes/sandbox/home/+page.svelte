<script>
  import { Carta, MarkdownEditor } from "$lib/carta/index";
  import Icon from "$components/basic_elems/Icon.svelte";
  import Button from "$components/basic_elems/Button.svelte";
  import TopToolbar from "$components/home/TopToolbar.svelte";
  import "$lib/carta/default.css";
  import DOMPurify from "isomorphic-dompurify";
  import { onMount, mount } from "svelte";
  import { page } from "$app/stores";

  export let isLoading = false;
  export let error = null;
  export let userName = "";
  export let isSignedIn = false;

  let effectiveIsLoading = isLoading;
  let effectiveError = error;
  let effectiveUserName = userName;
  let effectiveIsSignedIn = isSignedIn;

  $: {
    const variant = $page.url.searchParams.get("variant") || "default";
    effectiveIsLoading =
      isLoading ||
      variant === "loading" ||
      $page.url.searchParams.get("isLoading") === "true";
    effectiveError =
      error ||
      (variant === "error"
        ? "Failed to load"
        : $page.url.searchParams.get("error"));
    effectiveUserName =
      userName || $page.url.searchParams.get("userName") || "";
    effectiveIsSignedIn =
      isSignedIn ||
      variant === "signed_in" ||
      $page.url.searchParams.get("isSignedIn") === "true";
  }

  const carta = new Carta({
    theme: { light: "solarized-light", dark: "monokai" },
    sanitizer: DOMPurify.sanitize,
    shikiOptions: {
      langs: ["js", "ts", "bash", "json"],
    },
  });
  let value = `Ask me anything!`;
  let rightContainer;
  let loginRow;
  let toolbarRow;
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
  $: if (loginRow && toolbarRow) {
    loginRow.style.display = effectiveIsSignedIn ? "none" : "flex";
    toolbarRow.style.display = effectiveIsSignedIn ? "flex" : "none";
    updatePadding();
  }
</script>

<div class="bg-deep-charcoal grid grid-cols-12 min-h-screen">
  <div class="col-start-1 col-end-7 w-full p-4">
    <div class="h-full rounded-lg p-10 text-white">
      {#if effectiveIsLoading}
        <p class="text-xl font-bold mb-4">Loading...</p>
      {:else if effectiveError}
        <h2 class="text-xl font-bold mb-4">Error</h2>
        <p>{effectiveError}</p>
      {:else}
        <h2 class="text-xl font-bold mb-4">
          Conversation for {effectiveUserName || "User"}
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
      {#if effectiveIsLoading}
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
    padding: 0.5rem 1rem 2rem;
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
