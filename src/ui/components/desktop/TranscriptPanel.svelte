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
    if (message.role === "tool") return "tool";
    if (message.role === "user") return "user";
    return "system";
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

  let visibleActivity = $derived(session.activity.slice(0, 10));
</script>

<section
  class="flex min-h-0 flex-1 flex-col rounded-[22px] border border-white/8 bg-dark-slate/94 p-3 shadow-[0_24px_80px_rgba(0,0,0,0.34)] backdrop-blur-md sm:p-4"
>
  <div class="rounded-[16px] border border-white/8 bg-carbon-black/72 px-4 py-4">
    <div class="flex flex-col gap-5 xl:flex-row xl:items-start xl:justify-between">
      <div class="min-w-0">
        <div class="flex flex-wrap items-center gap-2">
          <span class="rounded-full border border-signal-blue/22 bg-signal-blue/10 px-2.5 py-1 font-mono text-[0.62rem] uppercase tracking-[0.22em] text-signal-blue">
            Session Log
          </span>
          <span class="rounded-full border border-white/8 bg-white/4 px-2.5 py-1 font-mono text-[0.62rem] uppercase tracking-[0.2em] text-fog/52">
            {runtimeActive ? "runtime live" : "runtime idle"}
          </span>
        </div>

        <div class="mt-3 flex flex-wrap gap-3 font-mono text-[0.7rem] uppercase tracking-[0.18em] text-fog/44">
          <span class="break-all">{session.cwd}</span>
          <span>{session.tokenUsage}</span>
          <span>{session.cost}</span>
        </div>
      </div>

      <div class="grid gap-4 xl:min-w-[420px] xl:grid-cols-2">
        <div>
          <div class="mb-2 font-mono text-[0.62rem] uppercase tracking-[0.28em] text-fog/46">
            Model
          </div>
          <div class="flex flex-wrap gap-2">
            {#each modelOptions as model}
              <button
                type="button"
                class={`rounded-xl border px-3 py-2 font-mono text-[0.68rem] uppercase tracking-[0.18em] transition ${
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
          <div class="mb-2 font-mono text-[0.62rem] uppercase tracking-[0.28em] text-fog/46">
            Permission
          </div>
          <div class="flex flex-wrap gap-2">
            {#each permissionModes as mode}
              <button
                type="button"
                class={`rounded-xl border px-3 py-2 font-mono text-[0.68rem] uppercase tracking-[0.18em] transition ${
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

  <div class="mt-3 flex-1 overflow-y-auto pr-1">
    {#if visibleActivity.length}
      <div class="mb-3 rounded-[16px] border border-white/8 bg-carbon-black/72 p-3">
        <div class="flex items-center justify-between gap-3">
          <div class="font-mono text-[0.62rem] uppercase tracking-[0.28em] text-fog/46">
            Live Actions
          </div>
          <div class="font-mono text-[0.62rem] uppercase tracking-[0.18em] text-fog/38">
            {visibleActivity.length} visible
          </div>
        </div>

        <div class="mt-3 space-y-2">
          {#each visibleActivity as item}
            <article class={`rounded-[14px] border px-3 py-3 ${activityStyle(item)}`}>
              <div class="flex flex-col gap-2 sm:flex-row sm:items-start sm:justify-between">
                <div class="min-w-0">
                  <div class="flex items-center gap-2">
                    <span
                      class={`h-2 w-2 rounded-full ${
                        item.status === "complete"
                          ? "bg-misty-green"
                          : item.status === "queued"
                            ? "bg-amber-300"
                            : "bg-signal-blue"
                      }`}
                    ></span>
                    <div class="font-mono text-[0.68rem] uppercase tracking-[0.18em] text-fog/52">
                      {item.label}
                    </div>
                  </div>
                  <p class="mt-2 text-[0.92rem] leading-6 text-fog/82">{item.summary}</p>
                </div>

                <div class="shrink-0 font-mono text-[0.62rem] uppercase tracking-[0.18em] text-fog/38">
                  {item.timestamp}
                </div>
              </div>
            </article>
          {/each}
        </div>
      </div>
    {/if}

    <div class="space-y-3">
      {#each session.transcript as message}
        <article class={`rounded-[16px] border p-4 ${messageStyle(message)}`}>
          <div class="flex flex-col gap-3 md:flex-row md:items-start md:justify-between">
            <div class="min-w-0 md:max-w-[140px]">
              <div class="font-mono text-[0.62rem] uppercase tracking-[0.26em] text-fog/46">
                {roleLabel(message)}
              </div>
              <div class="mt-2 text-sm font-medium text-soft-ivory">{message.title}</div>
            </div>

            <div class="min-w-0 flex-1">
              <p class="text-[0.95rem] leading-7 text-fog/84">{message.body}</p>
            </div>

            <div class="shrink-0 font-mono text-[0.62rem] uppercase tracking-[0.18em] text-fog/42">
              {message.meta}
            </div>
          </div>
        </article>
      {/each}
    </div>
  </div>

  <div class="mt-3 rounded-[16px] border border-white/8 bg-carbon-black/80">
    <div class="border-b border-white/8 px-4 py-3">
      <div class="flex flex-col gap-3 lg:flex-row lg:items-center lg:justify-between">
        <div>
          <div class="font-mono text-[0.62rem] uppercase tracking-[0.28em] text-fog/46">
            Composer
          </div>
          <div class="mt-2 max-w-4xl text-sm leading-6 text-fog/70">
            Prompts go to the local Harness runtime. In local mode, Poro talks to
            Ollama on your machine and keeps the session history in the desktop
            app store instead of inside the repo.
          </div>
        </div>
        <div class="font-mono text-[0.68rem] uppercase tracking-[0.18em] text-fog/42">
          {selectedProviderLabel} • {selectedModel} • {selectedPermission}
        </div>
      </div>
    </div>

    <div class="p-4">
      <label class="block">
        <span class="sr-only">Session prompt</span>
        <textarea
          class="min-h-32 w-full resize-y rounded-[14px] border border-white/8 bg-dark-slate/92 p-4 font-['SF_Mono','JetBrains_Mono','IBM_Plex_Mono',Menlo,monospace] text-[0.9rem] leading-7 text-fog/84 outline-none transition focus:border-signal-blue/35 focus:bg-dark-slate sm:p-5"
          value={composerText}
          placeholder="Describe the task you want Poro to run in this workspace."
          oninput={(event) => onComposerInput((event.currentTarget as HTMLTextAreaElement).value)}
        ></textarea>
      </label>

      <div class="mt-3 rounded-[12px] border border-white/8 bg-white/[0.035] px-4 py-3 text-sm leading-6 text-fog/66">
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
