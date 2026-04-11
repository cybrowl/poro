<script lang="ts">
  import Button from "$components/basic_elems/Button.svelte";
  import { fade, fly } from "svelte/transition";

  interface BrowserRuntimeInfo {
    session: string;
    browserPath: string;
    headless: boolean;
  }

  interface Props {
    open: boolean;
    browserUrl: string;
    runtimeActive: boolean;
    runtimeBusy: boolean;
    headless: boolean;
    runtimeInfo: BrowserRuntimeInfo | null;
    statusLine: string;
    latestSnapshot: string;
    latestPayload: string;
    onBrowserUrlInput: (value: string) => void;
    onToggleHeadless: () => void;
    onLaunchBrowser: () => void;
    onOpenAndSnapshot: () => void;
    onSnapshotBrowser: () => void;
    onStopBrowser: () => void;
    onClose: () => void;
  }

  let {
    open,
    browserUrl,
    runtimeActive,
    runtimeBusy,
    headless,
    runtimeInfo,
    statusLine,
    latestSnapshot,
    latestPayload,
    onBrowserUrlInput,
    onToggleHeadless,
    onLaunchBrowser,
    onOpenAndSnapshot,
    onSnapshotBrowser,
    onStopBrowser,
    onClose,
  }: Props = $props();
</script>

{#if open}
  <button
    class="ui-overlay fixed inset-0 z-40"
    onclick={onClose}
    aria-label="Close browser inspector"
    type="button"
    transition:fade={{ duration: 140 }}
  ></button>

  <aside
    class="ui-sheet ui-sheet-narrow fixed right-4 top-4 z-50 h-[calc(100vh-2rem)] w-[min(420px,calc(100vw-2rem))] overflow-y-auto p-5"
    transition:fly={{ duration: 180, x: 24 }}
  >
    <div class="flex items-start justify-between gap-4">
      <div>
        <div class="ui-section-label">Browser inspector</div>
        <div class="mt-2 type-heading-2 tracking-[-0.03em] text-soft-ivory">
          Brave sidecar
        </div>
        <div class="mt-2 type-body-4 text-fog/66">
          Secondary panel for snapshots, runtime state, and manual browser sanity checks.
        </div>
      </div>

      <Button label="Close" variant="outline" height="h-9" onclick={onClose} />
    </div>

    <div class="mt-5 space-y-4">
      <div class="flex flex-wrap items-center gap-2">
        <span class="ui-chip ui-chip-neutral">
          {runtimeActive ? "Brave live" : "Runtime idle"}
        </span>
        <button
          type="button"
          class={`ui-chip border transition ${
            headless
              ? "border-white/10 bg-white/[0.03] text-fog/54 hover:border-white/16 hover:text-soft-ivory"
              : "border-misty-green/24 bg-misty-green/10 text-misty-green"
          }`}
          onclick={onToggleHeadless}
        >
          {headless ? "Headless" : "Visible Brave"}
        </button>
      </div>

      <label class="block">
        <div class="mb-2 ui-section-label">URL</div>
        <input
          class="ui-input px-4 py-3 text-[0.9rem]"
          value={browserUrl}
          placeholder="https://example.com"
          oninput={(event) => onBrowserUrlInput((event.currentTarget as HTMLInputElement).value)}
        />
      </label>

      <div class="ui-panel-soft px-4 py-3 text-sm leading-6 text-fog/66">
        {statusLine}
      </div>

      <div class="flex flex-wrap gap-2">
        <Button
          label={runtimeBusy ? "Working..." : runtimeActive ? "Open + Snapshot" : "Launch + Open"}
          variant="gold"
          height="h-9"
          disabled={runtimeBusy || !browserUrl.trim()}
          onclick={onOpenAndSnapshot}
        />
        <Button
          label={runtimeActive ? "Snapshot" : "Launch Brave"}
          variant="outline"
          height="h-9"
          disabled={runtimeBusy}
          onclick={runtimeActive ? onSnapshotBrowser : onLaunchBrowser}
        />
        <Button
          label="Stop"
          variant="ghost"
          height="h-9"
          disabled={runtimeBusy || !runtimeActive}
          onclick={onStopBrowser}
        />
      </div>

      {#if runtimeInfo}
        <section class="ui-panel-subtle p-4">
          <div class="ui-section-label">Runtime</div>
          <div class="mt-3 space-y-2 text-sm text-fog/70">
            <div><span class="text-fog/42">Session:</span> {runtimeInfo.session}</div>
            <div><span class="text-fog/42">Mode:</span> {runtimeInfo.headless ? "Headless" : "Visible Brave"}</div>
            <div class="break-all"><span class="text-fog/42">Binary:</span> {runtimeInfo.browserPath}</div>
          </div>
        </section>
      {/if}

      <section class="ui-panel-subtle p-4">
        <div class="ui-section-label">Latest snapshot</div>
        <pre class="ui-code-block mt-3 max-h-[260px] overflow-auto whitespace-pre-wrap p-3 text-[0.78rem] leading-6 text-fog/76">{latestSnapshot || "No snapshot yet."}</pre>
      </section>

      <section class="ui-panel-subtle p-4">
        <div class="ui-section-label">Latest response</div>
        <pre class="ui-code-block mt-3 max-h-[220px] overflow-auto whitespace-pre-wrap p-3 text-[0.76rem] leading-6 text-fog/72">{latestPayload || "Waiting for a browser action."}</pre>
      </section>
    </div>
  </aside>
{/if}
