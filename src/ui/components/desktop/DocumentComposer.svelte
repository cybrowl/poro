<script lang="ts">
  import { onMount } from "svelte";
  import type { LayoutLine, LayoutLinesResult, PreparedTextWithSegments } from "@chenglou/pretext";

  interface Props {
    value: string;
    placeholder?: string;
    disabled?: boolean;
    onInput: (value: string) => void;
  }

  let {
    value,
    placeholder = "Write what you want Poro to do...",
    disabled = false,
    onInput,
  }: Props = $props();

  let frameEl: HTMLDivElement | null = null;
  let contentWidth = $state(0);
  let composerHeight = $state(420);
  let renderedLines = $state<LayoutLine[]>([]);
  let isFocused = $state(false);

  let pretextModule: typeof import("@chenglou/pretext") | null = null;
  let preparedText: PreparedTextWithSegments | null = null;
  let preparedSource = "";

  const horizontalPadding = 72;
  const verticalPadding = 96;
  const lineHeight = 36;
  const minHeight = 420;
  const font = '400 20px "Noto Sans"';

  function ensurePrepared(text: string) {
    if (!pretextModule) return null;

    const nextSource = text.length ? text : " ";
    if (preparedText && preparedSource === nextSource) {
      return preparedText;
    }

    preparedText = pretextModule.prepareWithSegments(nextSource, font, {
      whiteSpace: "pre-wrap",
    });
    preparedSource = nextSource;
    return preparedText;
  }

  function recomputeLayout() {
    if (!pretextModule || contentWidth <= 0) {
      return;
    }

    const prepared = ensurePrepared(value);
    if (!prepared) {
      return;
    }

    const availableWidth = Math.max(120, contentWidth - horizontalPadding);
    const layoutResult: LayoutLinesResult = pretextModule.layoutWithLines(
      prepared,
      availableWidth,
      lineHeight
    );
    renderedLines = value.length ? layoutResult.lines : [];
    composerHeight = Math.max(minHeight, Math.ceil(layoutResult.height + verticalPadding));
  }

  onMount(() => {
    let resizeObserver: ResizeObserver | null = null;
    let cancelled = false;

    void import("@chenglou/pretext").then((module) => {
      if (cancelled) return;
      pretextModule = module;
      recomputeLayout();
    });

    if (frameEl) {
      resizeObserver = new ResizeObserver((entries) => {
        const entry = entries[0];
        if (!entry) return;
        contentWidth = entry.contentRect.width;
        recomputeLayout();
      });
      resizeObserver.observe(frameEl);
      contentWidth = frameEl.clientWidth;
    }

    return () => {
      cancelled = true;
      resizeObserver?.disconnect();
    };
  });

  $effect(() => {
    value;
    recomputeLayout();
  });
</script>

<div bind:this={frameEl} class="relative mx-auto flex w-full max-w-[620px] flex-1">
  <div
    class={`pointer-events-none absolute inset-0 overflow-hidden px-9 pb-10 pt-6 text-[20px] leading-[36px] text-fog/84 transition-opacity duration-100 ${
      isFocused ? "opacity-0" : "opacity-100"
    }`}
    aria-hidden="true"
  >
    {#if value.length}
      <div class="space-y-0">
        {#each renderedLines as line}
          <div class="min-h-[36px] whitespace-pre text-fog/84">
            {line.text || "\u00a0"}
          </div>
        {/each}
      </div>
    {:else}
      <div class="text-fog/16">{placeholder}</div>
    {/if}
  </div>

  <textarea
    class={`ui-scrollbar-hidden min-h-0 w-full flex-1 resize-none overflow-hidden bg-transparent px-9 pb-10 pt-6 text-[20px] leading-[36px] outline-none transition-colors duration-100 placeholder:text-transparent ${
      isFocused ? "text-fog/84 caret-accent-gold" : "text-transparent caret-transparent"
    }`}
    style={`height: ${composerHeight}px;`}
    {disabled}
    placeholder=""
    value={value}
    spellcheck="true"
    autocapitalize="sentences"
    onfocus={() => {
      isFocused = true;
    }}
    onblur={() => {
      isFocused = false;
    }}
    oninput={(event) => onInput((event.currentTarget as HTMLTextAreaElement).value)}
  ></textarea>
</div>
