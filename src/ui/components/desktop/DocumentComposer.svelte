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
  let selectionStart = $state(0);
  let selectionEnd = $state(0);
  let isFocused = $state(false);

  let pretextModule: typeof import("@chenglou/pretext") | null = null;
  let preparedText: PreparedTextWithSegments | null = null;
  let preparedSource = "";
  let measureContext: CanvasRenderingContext2D | null = null;

  const horizontalPadding = 72;
  const verticalPadding = 104;
  const lineHeight = 40;
  const minHeight = 420;
  const font = '400 24px "Noto Sans"';

  type RenderedLineInfo = LayoutLine & {
    rawStart: number;
    rawEnd: number;
    top: number;
  };

  type SelectionRect = {
    top: number;
    left: number;
    width: number;
  };

  type CaretRect = {
    top: number;
    left: number;
  };

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

  function getMeasureContext() {
    if (typeof document === "undefined") {
      return null;
    }

    if (!measureContext) {
      const canvas = document.createElement("canvas");
      measureContext = canvas.getContext("2d");
    }

    if (measureContext) {
      measureContext.font = font;
    }

    return measureContext;
  }

  function measureWidth(text: string) {
    const context = getMeasureContext();
    if (!context || !text.length) {
      return 0;
    }

    return context.measureText(text).width;
  }

  function syncSelectionFromTarget(target: HTMLTextAreaElement) {
    selectionStart = target.selectionStart ?? 0;
    selectionEnd = target.selectionEnd ?? selectionStart;
  }

  function normalizeRenderedLines(lines: LayoutLine[], source: string): RenderedLineInfo[] {
    if (!lines.length) {
      return [];
    }

    let sourceCursor = 0;

    return lines.map((line, index) => {
      let rawStart = sourceCursor;

      if (line.text.length && !source.startsWith(line.text, rawStart)) {
        const fallback = source.indexOf(line.text, rawStart);
        if (fallback >= rawStart) {
          rawStart = fallback;
        }
      }

      const rawEnd = rawStart + line.text.length;
      sourceCursor = rawEnd;

      if (source[sourceCursor] === "\n") {
        sourceCursor += 1;
      }

      return {
        ...line,
        rawStart,
        rawEnd,
        top: index * lineHeight,
      };
    });
  }

  let normalizedLines = $derived(normalizeRenderedLines(renderedLines, value));

  let selectionRects = $derived.by(() => {
    if (!isFocused || selectionStart === selectionEnd || !normalizedLines.length) {
      return [] as SelectionRect[];
    }

    return normalizedLines.flatMap((line) => {
      const start = Math.max(selectionStart, line.rawStart);
      const end = Math.min(selectionEnd, line.rawEnd);

      if (end <= start) {
        return [];
      }

      const localStart = start - line.rawStart;
      const localEnd = end - line.rawStart;
      const prefix = line.text.slice(0, localStart);
      const selected = line.text.slice(localStart, localEnd);
      const left = measureWidth(prefix);
      const width = Math.max(8, measureWidth(selected));

      return [
        {
          top: line.top,
          left,
          width,
        },
      ];
    });
  });

  let caretRect = $derived.by(() => {
    if (!isFocused || selectionStart !== selectionEnd) {
      return null as CaretRect | null;
    }

    if (!normalizedLines.length) {
      return {
        top: 0,
        left: 0,
      };
    }

    const cursor = selectionStart;
    const line =
      normalizedLines.find((candidate) => cursor >= candidate.rawStart && cursor <= candidate.rawEnd) ??
      normalizedLines[normalizedLines.length - 1];

    if (!line) {
      return null;
    }

    const localOffset = Math.max(0, Math.min(cursor - line.rawStart, line.text.length));
    const prefix = line.text.slice(0, localOffset);

    return {
      top: line.top,
      left: measureWidth(prefix),
    };
  });

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
    class="pointer-events-none absolute inset-0 overflow-hidden px-9 pb-10 pt-6 text-[24px] leading-[40px] tracking-[-0.024em] text-fog/84"
    aria-hidden="true"
  >
    {#each selectionRects as rect}
      <div
        class="absolute rounded-md bg-accent-gold/[0.14]"
        style={`top: ${24 + rect.top}px; left: ${36 + rect.left}px; width: ${rect.width}px; height: ${lineHeight}px;`}
      ></div>
    {/each}

    {#if value.length}
      <div class="space-y-0">
        {#each renderedLines as line}
          <div class="min-h-[40px] whitespace-pre text-fog/84">
            {line.text || "\u00a0"}
          </div>
        {/each}
      </div>
    {:else}
      <div class="text-fog/16">{placeholder}</div>
    {/if}

    {#if caretRect}
      <div
        class="ui-document-caret absolute w-[2px] rounded-full bg-accent-gold"
        style={`top: ${24 + caretRect.top + 4}px; left: ${36 + caretRect.left}px; height: 30px;`}
      ></div>
    {/if}
  </div>

  <textarea
    class="ui-document-input ui-scrollbar-hidden min-h-0 w-full flex-1 resize-none overflow-hidden bg-transparent px-9 pb-10 pt-6 text-[24px] leading-[40px] tracking-[-0.024em] text-transparent outline-none placeholder:text-transparent"
    style={`height: ${composerHeight}px;`}
    {disabled}
    placeholder=""
    value={value}
    spellcheck="true"
    autocapitalize="sentences"
    onfocus={(event) => {
      isFocused = true;
      syncSelectionFromTarget(event.currentTarget as HTMLTextAreaElement);
    }}
    onblur={() => {
      isFocused = false;
    }}
    onselect={(event) => syncSelectionFromTarget(event.currentTarget as HTMLTextAreaElement)}
    onkeyup={(event) => syncSelectionFromTarget(event.currentTarget as HTMLTextAreaElement)}
    onclick={(event) => syncSelectionFromTarget(event.currentTarget as HTMLTextAreaElement)}
    oninput={(event) => {
      const target = event.currentTarget as HTMLTextAreaElement;
      syncSelectionFromTarget(target);
      onInput(target.value);
    }}
  ></textarea>
</div>
