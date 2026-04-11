<script lang="ts">
  import Button from "$components/basic_elems/Button.svelte";
  import submitIconUrl from "../../assets/submit.svg";
  import type { WorkspaceGitState } from "$lib/gitRuntime";
  import type {
    ActivityItem,
    PermissionMode,
    SessionRecord,
    TranscriptMessage,
  } from "$lib/mockDesktopData";

  interface Props {
    session: SessionRecord;
    gitState: WorkspaceGitState | null;
    selectedGitPath: string | null;
    gitDiffText: string;
    gitDiffLoading: boolean;
    gitError: string | null;
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
    onSelectGitPath: (path: string) => void;
    onRefreshGit: () => void;
    onStopRuntime: () => void;
    onRefreshRuntime: () => void;
  }

  let {
    session,
    gitState,
    selectedGitPath,
    gitDiffText,
    gitDiffLoading,
    gitError,
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
    onSelectGitPath,
    onRefreshGit,
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

  type ConversationBlock =
    | {
        kind: "message";
        id: string;
        message: TranscriptMessage;
      }
    | {
        kind: "actions";
        id: string;
        messages: TranscriptMessage[];
        defaultOpen: boolean;
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

  function truncateText(value: string, max = 180) {
    return value.length > max ? `${value.slice(0, max)}…` : value;
  }

  function groupConversation(messages: TranscriptMessage[], busy: boolean): ConversationBlock[] {
    const blocks: ConversationBlock[] = [];
    let compactGroup: TranscriptMessage[] = [];

    function flushCompactGroup(nextIsTail = false) {
      if (!compactGroup.length) {
        return;
      }

      blocks.push({
        kind: "actions",
        id: compactGroup[0].id,
        messages: compactGroup,
        defaultOpen: busy && nextIsTail,
      });
      compactGroup = [];
    }

    messages.forEach((message, index) => {
      if (isCompactMessage(message)) {
        compactGroup.push(message);
      } else {
        flushCompactGroup(false);
        blocks.push({
          kind: "message",
          id: message.id,
          message,
        });
      }

      if (index === messages.length - 1) {
        flushCompactGroup(true);
      }
    });

    return blocks;
  }

  function actionGroupSummary(messages: TranscriptMessage[]) {
    const latest = messages[messages.length - 1];
    return `${messages.length} action${messages.length === 1 ? "" : "s"} • ${latest.title}`;
  }

  function actionGroupDetail(messages: TranscriptMessage[]) {
    const latest = messages[messages.length - 1];
    return truncateText(latest.body, 220);
  }

  function actionGroupTone(messages: TranscriptMessage[]) {
    const latest = messages[messages.length - 1];
    const meta = latest.meta.toLowerCase();
    if (meta.includes("failed") || meta.includes("approval")) {
      return "border-warning-amber/16 bg-warning-amber/[0.055]";
    }
    if (meta.includes("verification")) {
      return "border-misty-green/16 bg-misty-green/[0.055]";
    }
    return "border-white/6 bg-white/[0.02]";
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
  let conversationBlocks = $derived(groupConversation(session.transcript, runtimeBusy));
  let submitButtonLabel = $derived(
    runtimeBusy ? "Working..." : runtimeActive ? "Send message" : "Launch and send"
  );
  let gitSummaryTone = $derived(
    gitError
      ? "ui-panel-warning"
      : gitState?.isGitRepo
        ? gitState.clean
          ? "ui-panel-success"
          : "ui-panel-soft"
        : "ui-panel-soft"
  );
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

        <section class="space-y-3">
          <div class="flex flex-col gap-3 md:flex-row md:items-end md:justify-between">
            <div class="min-w-0">
              <div class="type-label text-fog/34">Working tree</div>
              <div class="mt-2 flex flex-wrap items-center gap-2">
                <div class="type-heading-3 text-soft-ivory">{gitState?.branch ?? session.branch}</div>
                {#if gitState?.isGitRepo}
                  <span class="rounded-md bg-white/[0.04] px-2 py-0.5 text-[11px] text-fog/54">
                    {gitState.clean ? "clean" : `${gitState.changedFiles.length} files`}
                  </span>
                  {#if gitState.stagedCount}
                    <span class="rounded-md bg-white/[0.04] px-2 py-0.5 text-[11px] text-fog/54">
                      {gitState.stagedCount} staged
                    </span>
                  {/if}
                  {#if gitState.unstagedCount}
                    <span class="rounded-md bg-white/[0.04] px-2 py-0.5 text-[11px] text-fog/54">
                      {gitState.unstagedCount} unstaged
                    </span>
                  {/if}
                  {#if gitState.untrackedCount}
                    <span class="rounded-md bg-white/[0.04] px-2 py-0.5 text-[11px] text-fog/54">
                      {gitState.untrackedCount} untracked
                    </span>
                  {/if}
                {/if}
              </div>
              <div class="mt-2 type-body-4 text-fog/62">
                {gitError ?? gitState?.summary ?? "Checking the current branch and working tree."}
              </div>
            </div>

            <div class="shrink-0">
              <Button
                label="Refresh git"
                variant="ghost"
                height="h-8"
                onclick={onRefreshGit}
              />
            </div>
          </div>

          <div class={`${gitSummaryTone} rounded-2xl px-4 py-4`}>
            {#if !gitState}
              <div class="type-body-4 text-fog/62">Git state will appear once the workspace finishes loading.</div>
            {:else if !gitState.isGitRepo}
              <div class="type-body-4 text-fog/62">This workspace is not a git repository yet, so there is no branch or working-tree diff to review.</div>
            {:else if gitState.clean}
              <div class="type-body-4 text-fog/62">No uncommitted changes right now. When the agent edits files, they will appear here immediately.</div>
            {:else}
              <div class="grid gap-3 lg:grid-cols-[minmax(0,260px)_minmax(0,1fr)]">
                <div class="ui-panel-subtle overflow-hidden">
                  <div class="border-b border-white/6 px-3 py-2 type-label text-fog/34">
                    Changed files
                  </div>
                  <div class="max-h-[320px] overflow-y-auto p-2">
                    <div class="space-y-1">
                      {#each gitState.changedFiles as file}
                        <button
                          type="button"
                          class={`flex w-full items-start justify-between gap-3 rounded-xl px-3 py-3 text-left transition ${
                            file.path === selectedGitPath
                              ? "bg-white/[0.06] text-soft-ivory"
                              : "hover:bg-white/[0.03] text-fog/72"
                          }`}
                          onclick={() => onSelectGitPath(file.path)}
                        >
                          <div class="min-w-0">
                            <div class="truncate type-body-4 text-soft-ivory">{file.path}</div>
                            <div class="mt-1 type-body-5 text-fog/46">{file.summary}</div>
                          </div>
                          <div class="shrink-0 text-right text-[11px] text-fog/46">
                            <div class="text-misty-green">+{file.additions}</div>
                            <div class="mt-1 text-red-200/75">-{file.deletions}</div>
                          </div>
                        </button>
                      {/each}
                    </div>
                  </div>
                </div>

                <div class="ui-panel-subtle min-h-[220px] overflow-hidden">
                  <div class="border-b border-white/6 px-4 py-3">
                    <div class="type-label text-fog/34">Diff</div>
                    <div class="mt-2 type-body-4 text-soft-ivory">
                      {selectedGitPath ?? "Select a changed file"}
                    </div>
                  </div>

                  {#if gitDiffLoading}
                    <div class="px-4 py-4 type-body-4 text-fog/62">Loading the current git diff…</div>
                  {:else if gitError}
                    <div class="px-4 py-4 type-body-4 text-warning-amber">{gitError}</div>
                  {:else if gitDiffText}
                    <pre class="ui-code-block max-h-[320px] overflow-auto px-4 py-4 text-[12px] leading-6 text-fog/82 whitespace-pre-wrap break-words">{gitDiffText}</pre>
                  {:else}
                    <div class="px-4 py-4 type-body-4 text-fog/62">Choose a file to inspect its current git diff.</div>
                  {/if}
                </div>
              </div>
            {/if}
          </div>
        </section>
      </section>

      <div class="space-y-4">
        <div class="type-label text-fog/34">Conversation</div>
        {#each conversationBlocks as block}
          {#if block.kind === "message"}
            <article class={messageStyle(block.message)}>
              <div class="flex items-start justify-between gap-3">
                <div class="min-w-0">
                  <div class="mb-2 type-label text-fog/34">
                    {roleLabel(block.message)}
                  </div>
                  {#if block.message.role !== "assistant"}
                    <div class="type-heading-4 text-soft-ivory">{block.message.title}</div>
                  {/if}
                  <div class={`${block.message.role === "assistant" ? "type-body-3 text-fog/88" : "mt-2 type-body-3 text-fog/82"}`}>
                    {block.message.body}
                  </div>
                </div>
              </div>
              <div class="mt-3 type-body-5 text-fog/36">{block.message.meta}</div>
            </article>
          {:else}
            <details class={`rounded-2xl border ${actionGroupTone(block.messages)}`} open={block.defaultOpen}>
              <summary class="list-none cursor-pointer px-4 py-4 [&::-webkit-details-marker]:hidden">
                <div class="flex items-start justify-between gap-3">
                  <div class="min-w-0">
                    <div class="mb-2 flex flex-wrap items-center gap-2">
                      <span class="type-label text-fog/34">Actions</span>
                      <span class="rounded-md bg-white/[0.04] px-2 py-0.5 text-[11px] text-fog/58">
                        {block.messages.length}
                      </span>
                    </div>
                    <div class="type-heading-4 text-soft-ivory">{actionGroupSummary(block.messages)}</div>
                    <div class="mt-2 type-body-4 text-fog/70">{actionGroupDetail(block.messages)}</div>
                  </div>
                  <div class="shrink-0 type-body-5 text-fog/34">
                    {block.messages[block.messages.length - 1]?.meta}
                  </div>
                </div>
              </summary>

              <div class="border-t border-white/6 px-4 py-3">
                <div class="space-y-3">
                  {#each block.messages as message}
                    <article class="rounded-xl border border-white/6 bg-white/[0.02] px-3 py-3">
                      <div class="flex items-start justify-between gap-3">
                        <div class="min-w-0">
                          <div class="mb-1 flex flex-wrap items-center gap-2">
                            <span class="type-heading-4 text-soft-ivory">{message.title}</span>
                            <span class="rounded-md bg-white/[0.04] px-2 py-0.5 text-[11px] text-fog/54">
                              {roleLabel(message)}
                            </span>
                          </div>
                          <div class="type-body-4 whitespace-pre-wrap break-words text-fog/72">{message.body}</div>
                        </div>
                        <div class="shrink-0 type-body-5 text-fog/34">{message.meta}</div>
                      </div>
                    </article>
                  {/each}
                </div>
              </div>
            </details>
          {/if}
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
            label=""
            ariaLabel={submitButtonLabel}
            title={submitButtonLabel}
            variant="ghost"
            disabled={runtimeBusy || !composerText.trim()}
            class="w-10 min-w-10 rounded-2xl bg-transparent p-0 hover:bg-white/[0.03]"
            height="h-10"
            onclick={onSubmitPrompt}
          >
            {#snippet children()}
              {#if runtimeBusy}
                <span class="text-[11px] font-medium tracking-[0.04em]">...</span>
              {:else}
                <img src={submitIconUrl} alt="" class="h-9 w-9" />
              {/if}
            {/snippet}
          </Button>
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
