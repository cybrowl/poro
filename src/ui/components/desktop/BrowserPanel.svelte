<script lang="ts">
  import Button from "$components/basic_elems/Button.svelte";
  interface BrowserRuntimeInfo {
    session: string;
    browserPath: string;
    headless: boolean;
  }

  interface Props {
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
  }

  let {
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
  }: Props = $props();

</script>

<aside
  class="flex min-h-0 min-w-0 flex-col overflow-hidden rounded-[16px] border border-white/8 bg-[#0d1117] shadow-[0_18px_60px_rgba(0,0,0,0.28)]"
>
  <div class="border-b border-white/8 bg-[linear-gradient(180deg,rgba(255,255,255,0.02),rgba(255,255,255,0.01))] px-4 py-3 sm:px-5">
    <div class="flex flex-wrap items-center gap-2">
      <span class="rounded-md border border-signal-blue/22 bg-signal-blue/10 px-2.5 py-1 font-mono text-[0.62rem] uppercase tracking-[0.22em] text-signal-blue">
        Browser Inspector
      </span>
      <span class="rounded-md border border-white/8 bg-white/4 px-2.5 py-1 font-mono text-[0.62rem] uppercase tracking-[0.2em] text-fog/52">
        {runtimeActive ? "brave live" : "runtime idle"}
      </span>
      <button
        type="button"
        class={`rounded-md border px-2.5 py-1 font-mono text-[0.62rem] uppercase tracking-[0.2em] transition ${
          headless
            ? "border-white/10 bg-white/6 text-fog/60 hover:border-white/16 hover:bg-white/8"
            : "border-misty-green/24 bg-misty-green/10 text-misty-green hover:border-misty-green/32"
        }`}
        onclick={onToggleHeadless}
      >
        {headless ? "Headless" : "Visible Brave"}
      </button>
    </div>

    <p class="mt-3 text-sm leading-6 text-fog/66">
      Supporting pane for Brave snapshots and browser-side sanity checks.
    </p>

    <div class="mt-4 space-y-3">
      <label class="block">
        <span class="mb-2 block font-mono text-[0.62rem] uppercase tracking-[0.28em] text-fog/46">
          URL
        </span>
        <input
          class="w-full rounded-[12px] border border-white/8 bg-[#0f141b] px-4 py-3 font-['SF_Mono','JetBrains_Mono','IBM_Plex_Mono',Menlo,monospace] text-[0.88rem] text-fog/84 outline-none transition focus:border-signal-blue/35 focus:bg-[#111722]"
          value={browserUrl}
          placeholder="https://example.com"
          oninput={(event) => onBrowserUrlInput((event.currentTarget as HTMLInputElement).value)}
        />
      </label>

      <div class="rounded-[10px] border border-white/8 bg-white/[0.03] px-4 py-3 text-sm leading-6 text-fog/66">
        {statusLine}
      </div>

      <div class="flex flex-wrap gap-2">
        <Button
          label={runtimeBusy ? "Working..." : runtimeActive ? "Open + Snapshot" : "Launch + Open"}
          variant="gold"
          height="h-10"
          disabled={runtimeBusy || !browserUrl.trim()}
          onclick={onOpenAndSnapshot}
        />
        <Button
          label={runtimeActive ? "Snapshot" : "Launch Brave"}
          variant="outline"
          height="h-10"
          disabled={runtimeBusy}
          onclick={runtimeActive ? onSnapshotBrowser : onLaunchBrowser}
        />
        <Button
          label="Stop Browser"
          variant="ghost"
          height="h-10"
          disabled={runtimeBusy || !runtimeActive}
          onclick={onStopBrowser}
        />
      </div>
    </div>
  </div>

  <div class="min-h-0 flex-1 overflow-y-auto px-4 py-4 sm:px-5">
    {#if runtimeInfo}
      <div class="mb-3 rounded-[10px] border border-white/8 bg-[#0a0d12] p-3">
        <div class="font-mono text-[0.62rem] uppercase tracking-[0.28em] text-fog/42">
          Runtime
        </div>
        <div class="mt-3 grid gap-2 text-sm leading-6 text-fog/70">
          <div><span class="text-fog/46">Session:</span> {runtimeInfo.session}</div>
          <div><span class="text-fog/46">Mode:</span> {runtimeInfo.headless ? "Headless" : "Visible Brave"}</div>
          <div class="break-all"><span class="text-fog/46">Binary:</span> {runtimeInfo.browserPath}</div>
        </div>
      </div>
    {/if}

    <div class="space-y-3">
      <section class="rounded-[10px] border border-white/8 bg-[#0a0d12] p-3">
        <div class="font-mono text-[0.62rem] uppercase tracking-[0.28em] text-fog/42">
          Latest Snapshot
        </div>
        <pre class="mt-3 max-h-[220px] overflow-auto whitespace-pre-wrap rounded-[10px] border border-white/8 bg-[#0f141b] p-3 font-['SF_Mono','JetBrains_Mono','IBM_Plex_Mono',Menlo,monospace] text-[0.8rem] leading-6 text-fog/78">{latestSnapshot || "No snapshot yet. Open a URL or capture the current page."}</pre>
      </section>

      <section class="rounded-[10px] border border-white/8 bg-[#0a0d12] p-3">
        <div class="font-mono text-[0.62rem] uppercase tracking-[0.28em] text-fog/42">
          Latest Response
        </div>
        <pre class="mt-3 max-h-[220px] overflow-auto whitespace-pre-wrap rounded-[10px] border border-white/8 bg-[#0f141b] p-3 font-['SF_Mono','JetBrains_Mono','IBM_Plex_Mono',Menlo,monospace] text-[0.76rem] leading-6 text-fog/72">{latestPayload || "Waiting for a browser action."}</pre>
      </section>
    </div>
  </div>
</aside>
