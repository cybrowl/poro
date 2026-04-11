<script lang="ts">
  import Button from "$components/basic_elems/Button.svelte";
  import historyIconUrl from "../../assets/history.svg";
  import newChatIconUrl from "../../assets/new_chat.svg";
  import settingsIconUrl from "../../assets/settings.svg";
  import submitIconUrl from "../../assets/submit.svg";
  import logoUrl from "../../assets/logo.svg";
  import grokIconUrl from "../../assets/models/grok.svg";
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
    onOpenSettings: () => void;
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
    onOpenSettings,
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

  function assistantName() {
    const model = session.model.toLowerCase();
    if (model.includes("grok")) return "Grok 4";
    if (model.includes("claude")) return "Claude";
    if (model.includes("gemma")) return "Poro Local";
    return session.provider;
  }

  function assistantIcon() {
    const model = session.model.toLowerCase();
    if (model.includes("grok")) return grokIconUrl;
    return logoUrl;
  }

  function splitParagraphs(value: string) {
    return value
      .split(/\n\s*\n/)
      .map((part) => part.trim())
      .filter(Boolean);
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
  let showWorkingTree = $derived(!!gitState?.isGitRepo && !gitState.clean);
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

<section class="flex min-h-0 flex-1 bg-obsidian">
  <div class="flex min-h-0 flex-1 flex-col lg:flex-row">
    <div class="min-h-0 flex-1 overflow-y-auto lg:w-[44%] lg:flex-none">
      <div class="mx-auto flex w-full max-w-[560px] flex-col gap-8 px-8 py-10">
        {#if currentActivity && runtimeBusy}
          <div class="flex items-center gap-3 text-fog/48">
            <span class="h-2 w-2 animate-pulse rounded-full bg-accent-gold"></span>
            <span class="type-body-5">{currentActivity.label} • {currentActivity.summary}</span>
          </div>
        {/if}

        <div class="space-y-8">
          {#each conversationBlocks as block}
            {#if block.kind === "message"}
              {#if block.message.role === "user"}
                <article class="space-y-3">
                  <div class="flex items-center gap-2">
                    <span class="flex h-7 w-7 items-center justify-center rounded-full border border-fog/26 text-[13px] text-fog/72">◌</span>
                    <span class="type-heading-4 text-fog/82">You</span>
                  </div>
                  <div class="pl-9 pr-2">
                    {#each splitParagraphs(block.message.body) as paragraph}
                      <p class="type-body-2 leading-[1.45] text-fog/86">{paragraph}</p>
                    {/each}
                  </div>
                </article>
              {:else if block.message.role === "assistant"}
                <article class="space-y-3">
                  <div class="flex items-center gap-2">
                    <img src={assistantIcon()} alt="" class="h-7 w-7" />
                    <span class="type-heading-4 text-accent-gold">{assistantName()}</span>
                  </div>
                  <div class="space-y-5 pl-9 pr-4">
                    {#each splitParagraphs(block.message.body) as paragraph}
                      <p class="type-body-2 leading-[1.5] text-fog/82">{paragraph}</p>
                    {/each}
                  </div>
                </article>
              {:else}
                <article class="pl-9">
                  <div class="type-body-4 text-fog/64">{block.message.body}</div>
                </article>
              {/if}
            {:else}
              <details class="ml-9 overflow-hidden rounded-2xl border border-white/6 bg-white/[0.02]" open={block.defaultOpen}>
                <summary class="list-none cursor-pointer px-4 py-3 [&::-webkit-details-marker]:hidden">
                  <div class="flex items-center justify-between gap-3">
                    <div>
                      <div class="type-heading-4 text-soft-ivory">{actionGroupSummary(block.messages)}</div>
                      <div class="mt-1 type-body-4 text-fog/60">{actionGroupDetail(block.messages)}</div>
                    </div>
                    <div class="type-body-5 text-fog/34">{block.messages[block.messages.length - 1]?.meta}</div>
                  </div>
                </summary>
                <div class="border-t border-white/6 px-4 py-3">
                  <div class="space-y-3">
                    {#each block.messages as message}
                      <div class="rounded-xl bg-white/[0.025] px-3 py-3">
                        <div class="flex items-center justify-between gap-3">
                          <div class="type-heading-4 text-soft-ivory">{message.title}</div>
                          <div class="type-body-5 text-fog/34">{message.meta}</div>
                        </div>
                        <div class="mt-2 type-body-4 whitespace-pre-wrap break-words text-fog/66">{message.body}</div>
                      </div>
                    {/each}
                  </div>
                </div>
              </details>
            {/if}
          {/each}
        </div>
      </div>
    </div>

    <div class="hidden w-px bg-white/6 lg:block"></div>

    <div class="flex min-h-0 flex-1 flex-col">
      <div class="flex items-center justify-end gap-2 px-6 py-5">
        <button
          type="button"
          class="flex h-8 w-8 items-center justify-center rounded-lg text-fog/44 transition hover:bg-white/[0.03] hover:text-soft-ivory"
          aria-label="Clear draft"
          title="Clear draft"
          onclick={() => onComposerInput("")}
        >
          <img src={newChatIconUrl} alt="" class="h-4 w-4 opacity-80" />
        </button>
        <button
          type="button"
          class="flex h-8 w-8 items-center justify-center rounded-lg text-fog/44 transition hover:bg-white/[0.03] hover:text-soft-ivory"
          aria-label="Refresh session"
          title="Refresh session"
          onclick={onRefreshRuntime}
        >
          <img src={historyIconUrl} alt="" class="h-4 w-4 opacity-80" />
        </button>
        <button
          type="button"
          class="flex h-8 w-8 items-center justify-center rounded-lg text-fog/44 transition hover:bg-white/[0.03] hover:text-soft-ivory"
          aria-label="Open settings"
          title="Open settings"
          onclick={onOpenSettings}
        >
          <img src={settingsIconUrl} alt="" class="h-4 w-4 opacity-80" />
        </button>
      </div>

      <div class="flex min-h-0 flex-1 flex-col px-6 pb-6">
        <div class="relative flex min-h-0 flex-1 flex-col overflow-hidden rounded-[28px] bg-transparent">
          <Button
            label=""
            ariaLabel={submitButtonLabel}
            title={submitButtonLabel}
            variant="gold"
            disabled={runtimeBusy || !composerText.trim()}
            class="absolute right-0 top-0 z-10 h-14 w-14 min-w-14 rounded-[18px] border-0 shadow-[0_12px_40px_rgba(213,161,42,0.18)]"
            height="h-14"
            onclick={onSubmitPrompt}
          >
            {#snippet children()}
              {#if runtimeBusy}
                <span class="text-[11px] font-medium tracking-[0.04em] text-[#1f1807]">...</span>
              {:else}
                <img src={submitIconUrl} alt="" class="h-7 w-7" />
              {/if}
            {/snippet}
          </Button>

          <textarea
            class="type-body-2 min-h-0 flex-1 resize-none bg-transparent px-4 py-4 pr-20 text-fog/82 outline-none placeholder:text-fog/20"
            value={composerText}
            placeholder="|"
            oninput={(event) => onComposerInput((event.currentTarget as HTMLTextAreaElement).value)}
          ></textarea>

          <div class="mt-auto border-t border-white/6 px-4 py-3">
            <div class="flex flex-wrap items-center justify-between gap-3">
              <div class="type-body-5 text-fog/38">{runtimeStatusLine}</div>
              <div class="flex items-center gap-2">
                {#if runtimeActive}
                  <Button
                    label="Stop"
                    variant="ghost"
                    height="h-8"
                    class="text-fog/44"
                    onclick={onStopRuntime}
                  />
                {/if}
                <button
                  type="button"
                  class={`rounded-md border px-2.5 py-1 text-[11px] ${sessionStateTone()}`}
                >
                  {sessionStateLabel()}
                </button>
              </div>
            </div>
          </div>
        </div>

        {#if showWorkingTree}
          <details class={`mt-4 overflow-hidden rounded-2xl ${gitSummaryTone}`}>
            <summary class="list-none cursor-pointer px-4 py-3 [&::-webkit-details-marker]:hidden">
              <div class="flex items-center justify-between gap-3">
                <div>
                  <div class="type-heading-4 text-soft-ivory">Working tree</div>
                  <div class="mt-1 type-body-5 text-fog/50">{gitState?.summary}</div>
                </div>
                <div class="type-body-5 text-fog/34">{gitState?.changedFiles.length ?? 0} files</div>
              </div>
            </summary>

            <div class="grid gap-px border-t border-white/6 bg-white/6 lg:grid-cols-[minmax(0,220px)_minmax(0,1fr)]">
              <div class="bg-obsidian/70 p-2">
                <div class="space-y-1">
                  {#each gitState?.changedFiles ?? [] as file}
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
                        <div class="mt-1 type-body-5 text-fog/44">{file.summary}</div>
                      </div>
                    </button>
                  {/each}
                </div>
              </div>

              <div class="bg-obsidian/70">
                {#if gitDiffLoading}
                  <div class="px-4 py-4 type-body-4 text-fog/62">Loading the current git diff…</div>
                {:else if gitError}
                  <div class="px-4 py-4 type-body-4 text-warning-amber">{gitError}</div>
                {:else if gitDiffText}
                  <pre class="ui-code-block max-h-[260px] overflow-auto rounded-none border-0 bg-transparent px-4 py-4 text-[12px] leading-6 text-fog/82 whitespace-pre-wrap break-words">{gitDiffText}</pre>
                {:else}
                  <div class="px-4 py-4 type-body-4 text-fog/62">Choose a file to inspect its current git diff.</div>
                {/if}
              </div>
            </div>
          </details>
        {/if}
      </div>
    </div>
  </div>
</section>
