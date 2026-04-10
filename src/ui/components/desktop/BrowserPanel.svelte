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
    class="fixed inset-0 z-40 bg-black/50 backdrop-blur-sm"
    onclick={onClose}
    aria-label="Close browser inspector"
    type="button"
    transition:fade={{ duration: 140 }}
  ></button>

  <aside
    class="fixed right-4 top-4 z-50 h-[calc(100vh-2rem)] w-[min(420px,calc(100vw-2rem))] overflow-y-auto rounded-[18px] border border-white/10 bg-[#0f141c]/98 p-5 shadow-[0_32px_120px_rgba(0,0,0,0.58)]"
    transition:fly={{ duration: 180, x: 24 }}
  >
    <div class="flex items-start justify-between gap-4">
      <div>
        <div class="text-[0.72rem] uppercase tracking-[0.16em] text-fog/38">Browser inspector</div>
        <div class="mt-2 text-[1.3rem] font-medium tracking-[-0.03em] text-soft-ivory">
          Brave sidecar
        </div>
        <div class="mt-2 text-sm leading-6 text-fog/66">
          Secondary panel for snapshots, runtime state, and manual browser sanity checks.
        </div>
      </div>

      <Button label="Close" variant="outline" height="h-9" onclick={onClose} />
    </div>

    <div class="mt-5 space-y-4">
      <div class="flex flex-wrap items-center gap-2">
        <span class="rounded-md border border-white/8 bg-white/[0.03] px-2.5 py-1 text-[0.68rem] text-fog/54">
          {runtimeActive ? "Brave live" : "Runtime idle"}
        </span>
        <button
          type="button"
          class={`rounded-md border px-2.5 py-1 text-[0.68rem] transition ${
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
        <div class="mb-2 text-[0.72rem] uppercase tracking-[0.16em] text-fog/38">URL</div>
        <input
          class="w-full rounded-lg border border-white/8 bg-[#151a20] px-4 py-3 text-[0.9rem] text-fog/84 outline-none transition focus:border-signal-blue/30"
          value={browserUrl}
          placeholder="https://example.com"
          oninput={(event) => onBrowserUrlInput((event.currentTarget as HTMLInputElement).value)}
        />
      </label>

      <div class="rounded-lg border border-white/8 bg-white/[0.03] px-4 py-3 text-sm leading-6 text-fog/66">
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
        <section class="rounded-lg border border-white/8 bg-[#151a20] p-4">
          <div class="text-[0.72rem] uppercase tracking-[0.16em] text-fog/38">Runtime</div>
          <div class="mt-3 space-y-2 text-sm text-fog/70">
            <div><span class="text-fog/42">Session:</span> {runtimeInfo.session}</div>
            <div><span class="text-fog/42">Mode:</span> {runtimeInfo.headless ? "Headless" : "Visible Brave"}</div>
            <div class="break-all"><span class="text-fog/42">Binary:</span> {runtimeInfo.browserPath}</div>
          </div>
        </section>
      {/if}

      <section class="rounded-lg border border-white/8 bg-[#151a20] p-4">
        <div class="text-[0.72rem] uppercase tracking-[0.16em] text-fog/38">Latest snapshot</div>
        <pre class="mt-3 max-h-[260px] overflow-auto whitespace-pre-wrap rounded-lg border border-white/8 bg-[#0f141b] p-3 font-['SF_Mono','JetBrains_Mono','IBM_Plex_Mono',Menlo,monospace] text-[0.78rem] leading-6 text-fog/76">{latestSnapshot || "No snapshot yet."}</pre>
      </section>

      <section class="rounded-lg border border-white/8 bg-[#151a20] p-4">
        <div class="text-[0.72rem] uppercase tracking-[0.16em] text-fog/38">Latest response</div>
        <pre class="mt-3 max-h-[220px] overflow-auto whitespace-pre-wrap rounded-lg border border-white/8 bg-[#0f141b] p-3 font-['SF_Mono','JetBrains_Mono','IBM_Plex_Mono',Menlo,monospace] text-[0.76rem] leading-6 text-fog/72">{latestPayload || "Waiting for a browser action."}</pre>
      </section>
    </div>
  </aside>
{/if}
