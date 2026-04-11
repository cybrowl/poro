<script lang="ts">
  import { onMount } from "svelte";
  import type { PreparedText, LayoutResult } from "@chenglou/pretext";

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

  let pretextModule: typeof import("@chenglou/pretext") | null = null;
  let preparedText: PreparedText | null = null;
  let preparedSource = "";

  const horizontalPadding = 72;
  const verticalPadding = 104;
  const lineHeight = 40;
  const minHeight = 420;
  const font = '24px "Noto Sans"';

  function ensurePrepared(text: string) {
    if (!pretextModule) return null;

    const nextSource = text.length ? text : " ";
    if (preparedText && preparedSource === nextSource) {
      return preparedText;
    }

    preparedText = pretextModule.prepare(nextSource, font, { whiteSpace: "pre-wrap" });
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
    const layoutResult: LayoutResult = pretextModule.layout(prepared, availableWidth, lineHeight);
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
  <textarea
    class="ui-scrollbar-hidden min-h-0 w-full flex-1 resize-none overflow-hidden bg-transparent px-9 pb-10 pt-6 text-[24px] leading-[1.62] tracking-[-0.024em] text-fog/84 outline-none placeholder:text-fog/16"
    style={`height: ${composerHeight}px;`}
    {disabled}
    {placeholder}
    value={value}
    spellcheck="true"
    autocapitalize="sentences"
    oninput={(event) => onInput((event.currentTarget as HTMLTextAreaElement).value)}
  ></textarea>
</div>
