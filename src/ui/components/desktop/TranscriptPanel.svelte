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
    onOpenBrowserInspector: () => void;
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
    onOpenBrowserInspector,
  }: Props = $props();

  function messageStyle(message: TranscriptMessage) {
    if (message.role === "assistant") {
      return "bg-transparent text-fog/88";
    }

    if (message.role === "user") {
      return "rounded-xl border border-white/8 bg-white/[0.045] px-4 py-3 text-fog/84";
    }

    if (message.role === "tool") {
      return "rounded-xl border border-misty-green/14 bg-misty-green/[0.055] px-4 py-3 text-fog/80";
    }

    return "rounded-xl border border-white/8 bg-white/[0.03] px-4 py-3 text-fog/68";
  }

  function roleLabel(message: TranscriptMessage) {
    if (message.role === "assistant") return "Assistant";
    if (message.role === "tool") return "Action";
    if (message.role === "user") return "You";
    return "Runtime";
  }

  function activityTone(item: ActivityItem) {
    if (item.status === "complete") return "border-misty-green/16 bg-misty-green/[0.06]";
    if (item.status === "queued") return "border-amber-300/16 bg-amber-300/[0.055]";
    return "border-signal-blue/16 bg-signal-blue/[0.055]";
  }

  function sourceTone(source: "agent" | "browser") {
    return source === "browser"
      ? "bg-signal-blue/[0.08] text-signal-blue"
      : "bg-white/[0.045] text-fog/58";
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

    return merged.slice(0, 10);
  }

  let visibleActivity = $derived(interleaveActivity(session.activity, browserActivity));
</script>

<section class="flex min-h-0 flex-1 flex-col bg-[#111418]">
  <div class="border-b border-white/8 px-6 py-3">
    <div class="mx-auto flex w-full max-w-[760px] items-center justify-between gap-4">
      <div class="min-w-0">
        <div class="truncate text-[0.88rem] font-medium text-soft-ivory">
          {session.id} <span class="text-fog/36">{session.branch}</span>
        </div>
        <div class="mt-1 truncate text-[0.78rem] text-fog/42">{session.title}</div>
      </div>

      <div class="flex shrink-0 items-center gap-2">
        <span class="rounded-md border border-white/8 bg-white/[0.03] px-2.5 py-1 text-[0.68rem] text-fog/56">
          {selectedProviderLabel}
        </span>
        <span class="rounded-md border border-white/8 bg-white/[0.03] px-2.5 py-1 text-[0.68rem] text-fog/56">
          {selectedModel}
        </span>
        <span class="rounded-md border border-white/8 bg-white/[0.03] px-2.5 py-1 text-[0.68rem] text-fog/56">
          {selectedPermission}
        </span>
        <Button label="Browser" variant="ghost" height="h-9" onclick={onOpenBrowserInspector} />
        <Button label="Settings" variant="outline" height="h-9" onclick={onRefreshRuntime} />
      </div>
    </div>
  </div>

  <div class="min-h-0 flex-1 overflow-y-auto">
    <div class="mx-auto flex w-full max-w-[760px] flex-col gap-5 px-6 py-8">
      {#if visibleActivity.length}
        <section>
          <div class="mb-3 text-[0.72rem] uppercase tracking-[0.16em] text-fog/34">Agent activity</div>
          <div class="space-y-2">
            {#each visibleActivity as item}
              <article class={`rounded-lg border px-3 py-3 ${activityTone(item)}`}>
                <div class="flex items-start justify-between gap-3">
                  <div class="min-w-0">
                    <div class="flex flex-wrap items-center gap-2">
                      <div class="text-[0.78rem] font-medium text-soft-ivory">{item.label}</div>
                      <span class={`rounded-md px-2 py-0.5 text-[0.66rem] ${sourceTone(item.source)}`}>
                        {item.source}
                      </span>
                    </div>
                    <div class="mt-1 text-[0.86rem] leading-6 text-fog/72">{item.summary}</div>
                  </div>
                  <div class="shrink-0 text-[0.68rem] text-fog/34">{item.timestamp}</div>
                </div>
              </article>
            {/each}
          </div>
        </section>
      {/if}

      <div class="space-y-5">
        {#each session.transcript as message}
          <article class={messageStyle(message)}>
            <div class="mb-2 text-[0.7rem] uppercase tracking-[0.14em] text-fog/34">
              {roleLabel(message)}
            </div>
            {#if message.role !== "assistant"}
              <div class="text-[0.9rem] font-medium text-soft-ivory">{message.title}</div>
            {/if}
            <div class={`text-[0.96rem] leading-8 ${message.role === "assistant" ? "text-fog/88" : "mt-2"}`}>
              {message.body}
            </div>
            <div class="mt-3 text-[0.72rem] text-fog/36">{message.meta}</div>
          </article>
        {/each}
      </div>
    </div>
  </div>

  <div class="border-t border-white/8 bg-[#12171d]">
    <div class="mx-auto flex w-full max-w-[760px] flex-col gap-4 px-6 py-5">
      <div class="rounded-xl border border-white/8 bg-[#171c22] p-2">
        <textarea
          class="min-h-28 w-full resize-none bg-transparent px-3 py-3 font-['SF_Mono','JetBrains_Mono','IBM_Plex_Mono',Menlo,monospace] text-[0.92rem] leading-7 text-fog/86 outline-none"
          value={composerText}
          placeholder="Ask Poro to edit code, inspect the repo, or use the browser when needed."
          oninput={(event) => onComposerInput((event.currentTarget as HTMLTextAreaElement).value)}
        ></textarea>

        <div class="flex flex-wrap items-center justify-between gap-3 border-t border-white/8 px-3 pt-3">
          <div class="text-[0.8rem] text-fog/46">{runtimeStatusLine}</div>
          <div class="flex flex-wrap gap-2">
            <Button
              label="Model"
              variant="outline"
              height="h-9"
              onclick={() =>
                onSelectModel(
                  modelOptions[(modelOptions.indexOf(selectedModel) + 1) % modelOptions.length] ??
                    selectedModel
                )}
            />
            <Button
              label="Permission"
              variant="outline"
              height="h-9"
              onclick={() =>
                onSelectPermission(
                  permissionModes[
                    (permissionModes.indexOf(selectedPermission) + 1) % permissionModes.length
                  ] ?? selectedPermission
                )}
            />
            <Button
              label={runtimeBusy ? "Working..." : runtimeActive ? "Send" : "Launch + Send"}
              variant="gold"
              disabled={runtimeBusy || !composerText.trim()}
              height="h-9"
              onclick={onSubmitPrompt}
            />
            <Button
              label={runtimeActive ? "Stop" : "Refresh"}
              variant="ghost"
              height="h-9"
              onclick={runtimeActive ? onStopRuntime : onRefreshRuntime}
            />
          </div>
        </div>
      </div>
    </div>
  </div>
</section>
