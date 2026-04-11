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
    runtimeActivity: ActivityItem[];
    browserActivity: ActivityItem[];
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
    runtimeActivity,
    browserActivity,
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

  type ActivityKind =
    | "planning"
    | "editing"
    | "verification"
    | "review"
    | "browser"
    | "blocked"
    | "runtime";

  type SurfaceActivity = ActivityItem & {
    source: "agent" | "browser";
    kind: ActivityKind;
    live: boolean;
  };

  function messageStyle(message: TranscriptMessage) {
    if (message.role === "assistant") {
      return "bg-transparent text-fog/92";
    }

    if (message.role === "user") {
      return "rounded-2xl border border-white/6 bg-white/[0.03] px-4 py-3 text-fog/84";
    }

    if (message.role === "tool") {
      return "rounded-xl border border-white/6 bg-white/[0.02] px-3 py-3 text-fog/76";
    }

    return "rounded-xl border border-white/6 bg-white/[0.02] px-3 py-3 text-fog/68";
  }

  function roleLabel(message: TranscriptMessage) {
    if (message.role === "assistant") return "Assistant";
    if (message.role === "tool") return "Action";
    if (message.role === "user") return "You";
    return "Runtime";
  }

  function activityKind(source: "agent" | "browser", item: ActivityItem): ActivityKind {
    if (source === "browser") return "browser";

    const haystack = `${item.label} ${item.summary}`.toLowerCase();
    if (
      haystack.includes("verify") ||
      haystack.includes("check") ||
      haystack.includes("test") ||
      haystack.includes("smoke")
    ) {
      return "verification";
    }
    if (
      haystack.includes("review") ||
      haystack.includes("diff") ||
      haystack.includes("change")
    ) {
      return "review";
    }
    if (
      haystack.includes("edit") ||
      haystack.includes("patch") ||
      haystack.includes("rename") ||
      haystack.includes("write") ||
      haystack.includes("update")
    ) {
      return "editing";
    }
    if (
      haystack.includes("plan") ||
      haystack.includes("clarif") ||
      haystack.includes("goal") ||
      haystack.includes("mission")
    ) {
      return "planning";
    }
    if (
      haystack.includes("approval") ||
      haystack.includes("permission") ||
      haystack.includes("blocked") ||
      haystack.includes("error")
    ) {
      return "blocked";
    }
    return "runtime";
  }

  function activityTone(item: SurfaceActivity) {
    if (item.status === "queued") return "border-accent-gold/18 bg-accent-gold/[0.055]";
    if (item.kind === "verification") return "border-misty-green/16 bg-misty-green/[0.055]";
    if (item.kind === "blocked") return "border-warning-amber/16 bg-warning-amber/[0.055]";
    if (item.status === "complete") return "border-white/8 bg-white/[0.03]";
    return "border-white/6 bg-white/[0.02]";
  }

  function sourceTone(source: "agent" | "browser") {
    return source === "browser"
      ? "bg-accent-gold/[0.08] text-accent-gold"
      : "bg-white/[0.04] text-fog/54";
  }

  function activityKindLabel(kind: ActivityKind) {
    if (kind === "planning") return "planning";
    if (kind === "editing") return "editing";
    if (kind === "verification") return "verification";
    if (kind === "review") return "review";
    if (kind === "browser") return "browser";
    if (kind === "blocked") return "attention";
    return "runtime";
  }

  function activityKindBadge(kind: ActivityKind) {
    if (kind === "planning") return "bg-white/[0.04] text-fog/58";
    if (kind === "editing") return "bg-accent-gold/[0.08] text-accent-gold";
    if (kind === "verification") return "bg-misty-green/[0.12] text-misty-green";
    if (kind === "review") return "bg-white/[0.04] text-fog/58";
    if (kind === "browser") return "bg-accent-gold/[0.08] text-accent-gold";
    if (kind === "blocked") return "bg-warning-amber/[0.12] text-warning-amber";
    return "bg-white/[0.04] text-fog/54";
  }

  function sessionStateLabel() {
    if (runtimeBusy) return "Working";
    if (runtimeActive) return "Ready";
    return "Not launched";
  }

  function sessionStateSummary() {
    if (runtimeBusy) return runtimeStatusLine;
    if (runtimeActive) {
      return "Session is live. Send the next instruction, review the latest work, or stop the runtime.";
    }
    return runtimeStatusLine;
  }

  function sessionStateTone() {
    if (runtimeBusy) return "border-accent-gold/18 bg-accent-gold/[0.055] text-accent-gold";
    if (runtimeActive) return "border-misty-green/16 bg-misty-green/[0.055] text-misty-green";
    return "border-white/6 bg-white/[0.03] text-fog/62";
  }

  function isCompactMessage(message: TranscriptMessage) {
    return message.role === "system" || message.role === "tool";
  }

  function interleaveActivity(
    primary: ActivityItem[],
    secondary: ActivityItem[]
  ): SurfaceActivity[] {
    const merged: SurfaceActivity[] = [];
    const seen = new Set<string>();
    const max = Math.max(primary.length, secondary.length);

    for (let index = 0; index < max; index += 1) {
      const nextPrimary = primary[index];
      const nextSecondary = secondary[index];

      if (nextPrimary && !seen.has(nextPrimary.id)) {
        merged.push({
          ...nextPrimary,
          source: "agent",
          kind: activityKind("agent", nextPrimary),
          live: true,
        });
        seen.add(nextPrimary.id);
      }

      if (nextSecondary && !seen.has(nextSecondary.id)) {
        merged.push({
          ...nextSecondary,
          source: "browser",
          kind: activityKind("browser", nextSecondary),
          live: true,
        });
        seen.add(nextSecondary.id);
      }
    }

    return merged.slice(0, 10);
  }

  function mapRecordedActivity(items: ActivityItem[]): SurfaceActivity[] {
    return items.slice(0, 6).map((item) => ({
      ...item,
      source: "agent",
      kind: activityKind("agent", item),
      live: false,
    }));
  }

  let liveActivity = $derived(interleaveActivity(runtimeActivity, browserActivity));
  let recordedActivity = $derived(mapRecordedActivity(session.activity));
  let visibleActivity = $derived(liveActivity.length ? liveActivity : recordedActivity);
  let currentActivity = $derived(
    visibleActivity.find((item) => item.status !== "complete") ?? visibleActivity[0] ?? null
  );
  let recentActivity = $derived(
    currentActivity
      ? visibleActivity.filter((item) => item.id !== currentActivity.id).slice(0, 4)
      : visibleActivity.slice(0, 4)
  );
  let historyActivity = $derived(recordedActivity.slice(0, 4));
</script>

<section class="flex min-h-0 flex-1 flex-col bg-obsidian">
  <div class="min-h-0 flex-1 overflow-y-auto">
    <div class="mx-auto flex w-full max-w-[760px] flex-col gap-6 px-6 py-6">
      <section class="space-y-4">
        <div class="flex flex-col gap-3 md:flex-row md:items-end md:justify-between">
          <div class="min-w-0">
            <div class="type-label text-fog/34">Session</div>
            <div class="mt-2 type-heading-2 text-soft-ivory">{session.title}</div>
            <div class="mt-2 type-body-4 text-fog/62">
              {session.goal}
            </div>
          </div>

          <div class="flex flex-wrap gap-2">
            <span class={`rounded-md border px-2.5 py-1 text-[11px] ${sessionStateTone()}`}>
              {sessionStateLabel()}
            </span>
            <span class="rounded-md border border-white/6 bg-white/[0.03] px-2.5 py-1 text-[11px] text-fog/54">
              {session.model}
            </span>
            <span class="rounded-md border border-white/6 bg-white/[0.03] px-2.5 py-1 text-[11px] text-fog/54">
              {session.permission}
            </span>
          </div>
        </div>

        <div class="type-body-5 text-fog/46">{sessionStateSummary()}</div>

        {#if currentActivity}
          <article class={`rounded-2xl border px-4 py-4 ${activityTone(currentActivity)}`}>
            <div class="flex items-start justify-between gap-3">
              <div class="min-w-0">
                <div class="mb-2 flex flex-wrap items-center gap-2">
                  <span class="type-label text-fog/34">Now</span>
                  <span class={`rounded-md px-2 py-0.5 text-[11px] ${activityKindBadge(currentActivity.kind)}`}>
                    {activityKindLabel(currentActivity.kind)}
                  </span>
                  <span class={`rounded-md px-2 py-0.5 text-[11px] ${sourceTone(currentActivity.source)}`}>
                    {currentActivity.source}
                  </span>
                </div>
                <div class="type-heading-3 text-soft-ivory">{currentActivity.label}</div>
                <div class="mt-2 type-body-4 text-fog/72">{currentActivity.summary}</div>
              </div>
              <div class="shrink-0 type-body-5 text-fog/34">{currentActivity.timestamp}</div>
            </div>
          </article>
        {/if}

        {#if recentActivity.length}
          <section>
            <div class="mb-2 type-label text-fog/34">
              {liveActivity.length ? "Live activity" : "Recent activity"}
            </div>
            <div class="space-y-2">
              {#each recentActivity as item}
                <article class={`rounded-xl border px-3 py-3 ${activityTone(item)}`}>
                  <div class="flex items-start justify-between gap-3">
                    <div class="min-w-0">
                      <div class="flex flex-wrap items-center gap-2">
                        <div class="type-heading-4 text-soft-ivory">{item.label}</div>
                        <span class={`rounded-md px-2 py-0.5 text-[11px] ${activityKindBadge(item.kind)}`}>
                          {activityKindLabel(item.kind)}
                        </span>
                        <span class={`rounded-md px-2 py-0.5 text-[11px] ${sourceTone(item.source)}`}>
                          {item.source}
                        </span>
                      </div>
                      <div class="mt-1 type-body-4 text-fog/70">{item.summary}</div>
                    </div>
                    <div class="shrink-0 type-body-5 text-fog/34">{item.timestamp}</div>
                  </div>
                </article>
              {/each}
            </div>
          </section>
        {/if}

        {#if liveActivity.length && historyActivity.length}
          <section>
            <div class="mb-2 type-label text-fog/34">Session history</div>
            <div class="space-y-2">
              {#each historyActivity as item}
                <article class="rounded-xl border border-white/6 bg-white/[0.02] px-3 py-3">
                  <div class="flex items-start justify-between gap-3">
                    <div class="min-w-0">
                      <div class="flex flex-wrap items-center gap-2">
                        <div class="type-heading-4 text-soft-ivory">{item.label}</div>
                        <span class={`rounded-md px-2 py-0.5 text-[11px] ${activityKindBadge(item.kind)}`}>
                          {activityKindLabel(item.kind)}
                        </span>
                      </div>
                      <div class="mt-1 type-body-4 text-fog/68">{item.summary}</div>
                    </div>
                    <div class="shrink-0 type-body-5 text-fog/34">{item.timestamp}</div>
                  </div>
                </article>
              {/each}
            </div>
          </section>
        {/if}
      </section>

      <div class="space-y-4">
        <div class="type-label text-fog/34">Conversation</div>
        {#each session.transcript as message}
          <article class={messageStyle(message)}>
            <div class="flex items-start justify-between gap-3">
              <div class="min-w-0">
                <div class="mb-2 type-label text-fog/34">
                  {roleLabel(message)}
                </div>
                {#if message.role !== "assistant"}
                  <div class="type-heading-4 text-soft-ivory">{message.title}</div>
                {/if}
                <div class={`${
                  isCompactMessage(message)
                    ? "mt-1 type-body-4 text-fog/72"
                    : message.role === "assistant"
                      ? "type-body-3 text-fog/88"
                      : "mt-2 type-body-3 text-fog/82"
                }`}>
                  {message.body}
                </div>
              </div>
              {#if isCompactMessage(message)}
                <div class="shrink-0 type-body-5 text-fog/34">{message.meta}</div>
              {/if}
            </div>
            {#if !isCompactMessage(message)}
              <div class="mt-3 type-body-5 text-fog/36">{message.meta}</div>
            {/if}
          </article>
        {/each}
      </div>
    </div>
  </div>

  <div class="mt-auto border-t border-white/6 bg-obsidian">
    <div class="mx-auto flex w-full max-w-[760px] flex-col gap-3 px-6 pb-3 pt-4">
      <textarea
        class="type-body-3 min-h-28 w-full resize-none bg-transparent px-0 py-0 text-fog/88 outline-none placeholder:text-fog/34"
        value={composerText}
        placeholder="Ask Poro to edit code, inspect the repo, or use the browser when needed."
        oninput={(event) => onComposerInput((event.currentTarget as HTMLTextAreaElement).value)}
      ></textarea>

      <div class="flex flex-wrap items-center justify-between gap-3 border-t border-white/6 pt-3">
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
</section>
