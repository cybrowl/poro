<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { Editor } from "@tiptap/core";
  import StarterKit from "@tiptap/starter-kit";
  import { Markdown } from "@tiptap/markdown";

  export let value = "";
  export let onAIImprove: ((text: string) => void) | null = null;

  let editor: Editor | null = null;
  let element: HTMLElement;
  let mode: "hybrid" | "source" = "hybrid";

  onMount(() => {
    editor = new Editor({
      element,
      extensions: [
        StarterKit.configure({ heading: { levels: [1, 2, 3] } }),
        Markdown,
      ],
      content: value,
      editorProps: {
        attributes: {
          class:
            "prose prose-zinc dark:prose-invert focus:outline-none min-h-[400px] p-8 text-lg leading-relaxed",
        },
      },
      onUpdate: () => {
        if (editor) value = editor.getMarkdown();
      },
    });
  });

  onDestroy(() => editor?.destroy());

  $: if (editor && mode === "hybrid" && value !== editor.getMarkdown()) {
    editor.commands.setContent(value, { emitUpdate: false });
  }

  function handleAIImprove() {
    if (onAIImprove && editor) onAIImprove(editor.getMarkdown());
  }

  export function focus() {
    editor?.commands.focus();
  }
</script>

{#if mode === "hybrid"}
  <div bind:this={element} class="editor-host"></div>
{:else}
  <textarea
    bind:value
    class="w-full h-125 bg-dark-slate text-[#e5e5e5] font-mono p-8 text-lg resize-none focus:outline-none"
  ></textarea>
{/if}

<style lang="postcss">
  :global(.ProseMirror) {
    background: #111113 !important;
    color: #e5e5e5 !important;
    caret-color: #e5e5e5 !important;
    line-height: 1.75;
  }

  /* iA Writer clean headings — no bright colors, just elegant size & weight */
  :global(.ProseMirror h1) {
    color: #f5f5f5;
    font-size: 2.35rem;
    font-weight: 700;
    margin: 2.2rem 0 1rem;
  }
  :global(.ProseMirror h2) {
    color: #f0f0f0;
    font-size: 1.85rem;
    font-weight: 600;
    margin: 2rem 0 0.9rem;
  }
  :global(.ProseMirror h3) {
    color: #e8e8e8;
    font-size: 1.55rem;
    font-weight: 500;
  }

  /* Subtle blockquote */
  :global(.ProseMirror blockquote) {
    border-left: 4px solid #555555;
    background: #242424;
    padding: 1.4rem 1.8rem;
    color: #d0d0d0;
    font-style: italic;
    margin: 1.8rem 0;
  }

  /* Clean code blocks */
  :global(.ProseMirror code) {
    background: #252525;
    color: #c8c8c8;
    padding: 0.2rem 0.45rem;
    border-radius: 4px;
    font-family: ui-monospace, monospace;
  }

  /* Extra iA Writer polish */
  :global(.ProseMirror p) {
    margin-bottom: 1.35rem;
  }
  :global(.ProseMirror a) {
    color: #a0a0a0;
    text-decoration: underline;
  }
  :global(.ProseMirror a:hover) {
    color: #e5e5e5;
  }
  :global(.ProseMirror strong) {
    color: #f5f5f5;
    font-weight: 600;
  }
  :global(.ProseMirror ul, :global(.ProseMirror ol)) {
    padding-left: 1.6rem;
    margin: 1.2rem 0;
  }
  :global(.ProseMirror hr) {
    border: none;
    border-top: 1px solid #444;
    margin: 2.5rem 0;
  }
</style>
