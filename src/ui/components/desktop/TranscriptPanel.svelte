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
      return "bg-transparent text-fog/92";
    }

    if (message.role === "user") {
      return "rounded-2xl border border-white/6 bg-white/[0.03] px-4 py-3 text-fog/84";
    }

    if (message.role === "tool") {
      return "rounded-2xl border border-signal-blue/14 bg-signal-blue/[0.04] px-4 py-3 text-fog/82";
    }

    return "rounded-2xl border border-white/6 bg-white/[0.02] px-4 py-3 text-fog/68";
  }

  function roleLabel(message: TranscriptMessage) {
    if (message.role === "assistant") return "Assistant";
    if (message.role === "tool") return "Action";
    if (message.role === "user") return "You";
    return "Runtime";
  }

  function activityTone(item: ActivityItem) {
    if (item.status === "queued") return "border-signal-blue/14 bg-signal-blue/[0.045]";
    if (item.status === "complete") return "border-white/8 bg-white/[0.03]";
    return "border-white/6 bg-white/[0.02]";
  }

  function sourceTone(source: "agent" | "browser") {
    return source === "browser"
      ? "bg-signal-blue/[0.08] text-signal-blue"
      : "bg-white/[0.04] text-fog/54";
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

<section class="flex min-h-0 flex-1 flex-col bg-obsidian">
  <div class="border-b border-white/6 px-6 py-3">
    <div class="mx-auto flex w-full max-w-[760px] items-center justify-between gap-4">
      <div class="min-w-0">
        <div class="truncate type-heading-4 text-soft-ivory">
          {session.id} <span class="text-fog/36">{session.branch}</span>
        </div>
        <div class="mt-1 type-body-4 truncate text-fog/42">{session.title}</div>
      </div>

      <div class="flex shrink-0 items-center gap-2">
        <span class="rounded-md border border-white/6 bg-white/[0.025] px-2.5 py-1 text-[11px] text-fog/52">
          {selectedProviderLabel}
        </span>
        <span class="rounded-md border border-white/6 bg-white/[0.025] px-2.5 py-1 text-[11px] text-fog/52">
          {selectedModel}
        </span>
        <span class="rounded-md border border-white/6 bg-white/[0.025] px-2.5 py-1 text-[11px] text-fog/52">
          {selectedPermission}
        </span>
        <Button label="Browser" variant="ghost" height="h-8" onclick={onOpenBrowserInspector} />
        <Button label="Settings" variant="ghost" height="h-8" onclick={onRefreshRuntime} />
      </div>
    </div>
  </div>

  <div class="min-h-0 flex-1 overflow-y-auto">
    <div class="mx-auto flex w-full max-w-[760px] flex-col gap-5 px-6 py-8">
      {#if visibleActivity.length}
        <section>
          <div class="mb-3 type-label text-fog/34">Agent activity</div>
          <div class="space-y-2">
            {#each visibleActivity as item}
              <article class={`rounded-xl border px-3 py-3 ${activityTone(item)}`}>
                <div class="flex items-start justify-between gap-3">
                  <div class="min-w-0">
                    <div class="flex flex-wrap items-center gap-2">
                      <div class="type-heading-4 text-soft-ivory">{item.label}</div>
                      <span class={`rounded-md px-2 py-0.5 text-[11px] ${sourceTone(item.source)}`}>
                        {item.source}
                      </span>
                    </div>
                    <div class="mt-1 type-body-4 text-fog/72">{item.summary}</div>
                  </div>
                  <div class="shrink-0 type-body-5 text-fog/34">{item.timestamp}</div>
                </div>
              </article>
            {/each}
          </div>
        </section>
      {/if}

      <div class="space-y-5">
        {#each session.transcript as message}
          <article class={messageStyle(message)}>
            <div class="mb-2 type-label text-fog/34">
              {roleLabel(message)}
            </div>
            {#if message.role !== "assistant"}
              <div class="type-heading-4 text-soft-ivory">{message.title}</div>
            {/if}
            <div class={`type-body-3 ${message.role === "assistant" ? "text-fog/88" : "mt-2 text-fog/82"}`}>
              {message.body}
            </div>
            <div class="mt-3 type-body-5 text-fog/36">{message.meta}</div>
          </article>
        {/each}
      </div>
    </div>
  </div>

  <div class="border-t border-white/6 bg-deep-charcoal">
    <div class="mx-auto flex w-full max-w-[760px] flex-col gap-4 px-6 py-5">
      <div class="rounded-2xl border border-white/6 bg-dark-slate p-2">
        <textarea
          class="type-body-3 min-h-28 w-full resize-none bg-transparent px-3 py-3 text-fog/88 outline-none placeholder:text-fog/34"
          value={composerText}
          placeholder="Ask Poro to edit code, inspect the repo, or use the browser when needed."
          oninput={(event) => onComposerInput((event.currentTarget as HTMLTextAreaElement).value)}
        ></textarea>

        <div class="flex flex-wrap items-center justify-between gap-3 border-t border-white/6 px-3 pt-3">
          <div class="type-body-5 text-fog/46">{runtimeStatusLine}</div>
          <div class="flex flex-wrap gap-2">
            <Button
              label="Model"
              variant="outline"
              height="h-8"
              onclick={() =>
                onSelectModel(
                  modelOptions[(modelOptions.indexOf(selectedModel) + 1) % modelOptions.length] ??
                    selectedModel
                )}
            />
            <Button
              label="Permission"
              variant="outline"
              height="h-8"
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
              height="h-8"
              onclick={onSubmitPrompt}
            />
            <Button
              label={runtimeActive ? "Stop" : "Refresh"}
              variant="ghost"
              height="h-8"
              onclick={runtimeActive ? onStopRuntime : onRefreshRuntime}
            />
          </div>
        </div>
      </div>
    </div>
  </div>
</section>
