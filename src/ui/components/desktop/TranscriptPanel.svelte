<script lang="ts">
  import Button from "$components/basic_elems/Button.svelte";
  import type {
    ActivityItem,
    PermissionMode,
    SessionRecord,
    TranscriptMessage,
  } from "$lib/mockDesktopData";

  interface Props {
    session: SessionRecord;
    browserActivity: ActivityItem[];
    selectedProviderLabel: string;
    selectedModel: string;
    selectedPermission: PermissionMode;
    modelOptions: string[];
    permissionModes: PermissionMode[];
    composerText: string;
    runtimeActive: boolean;
    runtimeBusy: boolean;
    runtimeStatusLine: string;
    onSelectModel: (model: string) => void;
    onSelectPermission: (mode: PermissionMode) => void;
    onComposerInput: (value: string) => void;
    onSubmitPrompt: () => void;
    onStopRuntime: () => void;
    onRefreshRuntime: () => void;
  }

  let {
    session,
    browserActivity,
    selectedProviderLabel,
    selectedModel,
    selectedPermission,
    modelOptions,
    permissionModes,
    composerText,
    runtimeActive,
    runtimeBusy,
    runtimeStatusLine,
    onSelectModel,
    onSelectPermission,
    onComposerInput,
    onSubmitPrompt,
    onStopRuntime,
    onRefreshRuntime,
  }: Props = $props();

  function messageStyle(message: TranscriptMessage) {
    if (message.role === "assistant") {
      return "border-signal-blue/14 bg-signal-blue/[0.055]";
    }

    if (message.role === "tool") {
      return "border-misty-green/12 bg-misty-green/[0.06]";
    }

    if (message.role === "user") {
      return "border-white/10 bg-white/[0.045]";
    }

    return "border-white/8 bg-carbon-black/78";
  }

  function roleLabel(message: TranscriptMessage) {
    if (message.role === "assistant") return "assistant";
    if (message.role === "tool") return "action";
    if (message.role === "user") return "prompt";
    return "runtime";
  }

  function activityStyle(item: ActivityItem) {
    if (item.status === "complete") {
      return "border-misty-green/20 bg-misty-green/[0.08]";
    }

    if (item.status === "queued") {
      return "border-amber-300/16 bg-amber-300/[0.06]";
    }

    return "border-signal-blue/20 bg-signal-blue/[0.07]";
  }

  function activityDot(status: ActivityItem["status"]) {
    if (status === "complete") return "bg-misty-green";
    if (status === "queued") return "bg-amber-300";
    return "bg-signal-blue";
  }

  function sourceTone(source: "agent" | "browser") {
    return source === "browser"
      ? "border-signal-blue/18 bg-signal-blue/[0.08] text-signal-blue"
      : "border-white/8 bg-white/[0.045] text-fog/64";
  }

  function interleaveActivity(
    primary: ActivityItem[],
    secondary: ActivityItem[]
  ): Array<ActivityItem & { source: "agent" | "browser" }> {
    const merged: Array<ActivityItem & { source: "agent" | "browser" }> = [];
    const seen = new Set<string>();
    const max = Math.max(primary.length, secondary.length);

    for (let index = 0; index < max; index += 1) {
      const nextPrimary = primary[index];
      const nextSecondary = secondary[index];

      if (nextPrimary && !seen.has(nextPrimary.id)) {
        merged.push({ ...nextPrimary, source: "agent" });
        seen.add(nextPrimary.id);
      }

      if (nextSecondary && !seen.has(nextSecondary.id)) {
        merged.push({ ...nextSecondary, source: "browser" });
        seen.add(nextSecondary.id);
      }
    }

    return merged.slice(0, 12);
  }

  let visibleActivity = $derived(interleaveActivity(session.activity, browserActivity));
</script>

<section
  class="flex min-h-0 flex-1 flex-col overflow-hidden rounded-[18px] border border-white/8 bg-[#0d1117] shadow-[0_18px_60px_rgba(0,0,0,0.34)]"
>
  <div class="border-b border-white/8 bg-[linear-gradient(180deg,rgba(255,255,255,0.03),rgba(255,255,255,0.01))] px-4 py-4 sm:px-5">
    <div class="flex flex-col gap-5 xl:flex-row xl:items-start xl:justify-between">
      <div class="min-w-0">
        <div class="flex flex-wrap items-center gap-2">
          <span class="rounded-md border border-signal-blue/22 bg-signal-blue/10 px-2.5 py-1 font-mono text-[0.62rem] uppercase tracking-[0.22em] text-signal-blue">
            Agent Canvas
          </span>
          <span class="rounded-md border border-white/8 bg-white/4 px-2.5 py-1 font-mono text-[0.62rem] uppercase tracking-[0.2em] text-fog/52">
            {runtimeActive ? "runtime live" : "runtime idle"}
          </span>
        </div>

        <h3 class="mt-3 text-[1.15rem] font-medium tracking-[-0.03em] text-soft-ivory sm:text-[1.32rem]">
          {session.title}
        </h3>
        <div class="mt-2 flex flex-wrap gap-3 font-mono text-[0.7rem] uppercase tracking-[0.18em] text-fog/42">
          <span>{session.status}</span>
          <span>{session.tokenUsage}</span>
          <span>{session.cost}</span>
        </div>
        <div class="mt-3 max-w-4xl text-sm leading-6 text-fog/70">
          {session.goal}
        </div>
      </div>

      <div class="grid gap-4 xl:min-w-[420px] xl:grid-cols-2">
        <div>
          <div class="mb-2 font-mono text-[0.62rem] uppercase tracking-[0.28em] text-fog/42">
            Model
          </div>
          <div class="flex flex-wrap gap-2">
            {#each modelOptions as model}
              <button
                type="button"
                class={`rounded-md border px-3 py-2 font-mono text-[0.68rem] uppercase tracking-[0.18em] transition ${
                  selectedModel === model
                    ? "border-signal-blue/35 bg-signal-blue/10 text-signal-blue"
                    : "border-white/10 bg-white/[0.035] text-fog/58 hover:border-white/16 hover:bg-white/[0.055]"
                }`}
                onclick={() => onSelectModel(model)}
              >
                {model}
              </button>
            {/each}
          </div>
        </div>

        <div>
          <div class="mb-2 font-mono text-[0.62rem] uppercase tracking-[0.28em] text-fog/42">
            Permission
          </div>
          <div class="flex flex-wrap gap-2">
            {#each permissionModes as mode}
              <button
                type="button"
                class={`rounded-md border px-3 py-2 font-mono text-[0.68rem] uppercase tracking-[0.18em] transition ${
                  selectedPermission === mode
                    ? mode === "danger-full-access"
                      ? "border-red-400/30 bg-red-400/10 text-red-200"
                      : mode === "workspace-write"
                        ? "border-misty-green/28 bg-misty-green/10 text-misty-green"
                        : "border-white/16 bg-white/8 text-soft-ivory"
                    : "border-white/10 bg-white/[0.035] text-fog/58 hover:border-white/16 hover:bg-white/[0.055]"
                }`}
                onclick={() => onSelectPermission(mode)}
              >
                {mode}
              </button>
            {/each}
          </div>
        </div>
      </div>
    </div>
  </div>

  <div class="grid min-h-0 flex-1 gap-0 lg:grid-cols-[320px_minmax(0,1fr)]">
    <aside class="min-h-0 border-b border-white/8 bg-[#0a0d12] lg:border-b-0 lg:border-r">
      <div class="flex items-center justify-between border-b border-white/8 px-4 py-3 sm:px-5">
        <div>
          <div class="font-mono text-[0.62rem] uppercase tracking-[0.28em] text-fog/40">
            Agent Feed
          </div>
          <div class="mt-1 text-sm text-fog/64">Live actions, browser steps, and runtime updates.</div>
        </div>
        <div class="font-mono text-[0.62rem] uppercase tracking-[0.18em] text-fog/34">
          {visibleActivity.length}
        </div>
      </div>

      <div class="max-h-full overflow-y-auto px-3 py-3 sm:px-4">
        {#if visibleActivity.length}
          <div class="space-y-2">
            {#each visibleActivity as item}
              <article class={`rounded-[12px] border px-3 py-3 ${activityStyle(item)}`}>
                <div class="flex items-start justify-between gap-3">
                  <div class="min-w-0">
                    <div class="flex flex-wrap items-center gap-2">
                      <span class={`h-2 w-2 rounded-full ${activityDot(item.status)}`}></span>
                      <div class="font-mono text-[0.64rem] uppercase tracking-[0.2em] text-fog/48">
                        {item.label}
                      </div>
                      <span class={`rounded-md border px-2 py-0.5 font-mono text-[0.56rem] uppercase tracking-[0.18em] ${sourceTone(item.source)}`}>
                        {item.source}
                      </span>
                    </div>
                    <p class="mt-2 text-[0.88rem] leading-6 text-fog/78">{item.summary}</p>
                  </div>
                  <div class="shrink-0 font-mono text-[0.6rem] uppercase tracking-[0.18em] text-fog/34">
                    {item.timestamp}
                  </div>
                </div>
              </article>
            {/each}
          </div>
        {:else}
          <div class="rounded-[12px] border border-dashed border-white/10 bg-white/[0.02] px-3 py-4 text-sm leading-6 text-fog/56">
            Session activity will stream here once the runtime starts planning, editing, or using tools.
          </div>
        {/if}
      </div>
    </aside>

    <div class="min-h-0 overflow-y-auto bg-[#0d1117]">
      <div class="border-b border-white/8 px-4 py-3 sm:px-5">
        <div class="font-mono text-[0.62rem] uppercase tracking-[0.28em] text-fog/40">
          Session Stream
        </div>
      </div>

      <div class="space-y-3 px-4 py-4 sm:px-5">
        {#each session.transcript as message}
          <article class={`rounded-[12px] border p-4 ${messageStyle(message)}`}>
            <div class="flex flex-col gap-3 md:flex-row md:items-start md:justify-between">
              <div class="min-w-0 md:max-w-[150px]">
                <div class="font-mono text-[0.62rem] uppercase tracking-[0.26em] text-fog/42">
                  {roleLabel(message)}
                </div>
                <div class="mt-2 text-sm font-medium text-soft-ivory">{message.title}</div>
              </div>

              <div class="min-w-0 flex-1">
                <p class="text-[0.94rem] leading-7 text-fog/82">{message.body}</p>
              </div>

              <div class="shrink-0 font-mono text-[0.62rem] uppercase tracking-[0.18em] text-fog/38">
                {message.meta}
              </div>
            </div>
          </article>
        {/each}
      </div>
    </div>
  </div>

  <div class="border-t border-white/8 bg-[#0a0d12]">
    <div class="border-b border-white/8 px-4 py-3 sm:px-5">
      <div class="flex flex-col gap-3 lg:flex-row lg:items-center lg:justify-between">
        <div>
          <div class="font-mono text-[0.62rem] uppercase tracking-[0.28em] text-fog/40">
            Composer
          </div>
          <div class="mt-2 max-w-4xl text-sm leading-6 text-fog/70">
            Prompts go straight into the local Harness runtime. Poro keeps the composer calm, while the feed above shows the agent's visible work as it happens.
          </div>
        </div>
        <div class="font-mono text-[0.68rem] uppercase tracking-[0.18em] text-fog/42">
          {selectedProviderLabel} • {selectedModel} • {selectedPermission}
        </div>
      </div>
    </div>

    <div class="p-4 sm:p-5">
      <label class="block">
        <span class="sr-only">Session prompt</span>
        <textarea
          class="min-h-32 w-full resize-y rounded-[12px] border border-white/8 bg-[#0f141b] p-4 font-['SF_Mono','JetBrains_Mono','IBM_Plex_Mono',Menlo,monospace] text-[0.9rem] leading-7 text-fog/84 outline-none transition focus:border-signal-blue/35 focus:bg-[#111722] sm:p-5"
          value={composerText}
          placeholder="Describe the task you want Poro to run in this workspace."
          oninput={(event) => onComposerInput((event.currentTarget as HTMLTextAreaElement).value)}
        ></textarea>
      </label>

      <div class="mt-3 rounded-[10px] border border-white/8 bg-white/[0.03] px-4 py-3 text-sm leading-6 text-fog/66">
        {runtimeStatusLine}
      </div>

      <div class="mt-4 flex flex-wrap gap-2">
        <Button
          label={runtimeBusy ? "Working..." : runtimeActive ? "Send Prompt" : "Launch + Send"}
          variant="gold"
          disabled={runtimeBusy || !composerText.trim()}
          height="h-11"
          onclick={onSubmitPrompt}
        />
        <Button
          label={runtimeActive ? "Stop Runtime" : "Refresh Sessions"}
          variant="ghost"
          height="h-11"
          onclick={runtimeActive ? onStopRuntime : onRefreshRuntime}
        />
        <Button
          label="Inspect Runtime"
          variant="outline"
          height="h-11"
          onclick={onRefreshRuntime}
        />
      </div>
    </div>
  </div>
</section>
