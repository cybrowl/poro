<script lang="ts">
  import Button from "$components/basic_elems/Button.svelte";
  import type { BackendHealth } from "$lib/clawRuntime";
  import {
    providerDefaultModel,
    providerModelOptions,
    type PermissionMode,
    type ProviderRecord,
  } from "$lib/mockDesktopData";
  import { fade, fly } from "svelte/transition";

  interface Props {
    open: boolean;
    backendPath: string;
    xAiApiKey: string;
    openAiApiKey: string;
    backendHealth: BackendHealth | null;
    healthCheckPending: boolean;
    isDesktop: boolean;
    recentWorkspaceCount: number;
    providers: ProviderRecord[];
    selectedProviderId: string;
    selectedModel: string;
    selectedPermission: PermissionMode;
    onSelectProvider: (id: string) => void;
    onSelectModel: (model: string) => void;
    onSelectPermission: (mode: PermissionMode) => void;
    onBackendPathChange: (path: string) => void;
    onXAiApiKeyChange: (value: string) => void;
    onOpenAiApiKeyChange: (value: string) => void;
    onRunHealthCheck: () => void;
    modelOptions: string[];
    permissionModes: PermissionMode[];
    onClose: () => void;
  }

  let {
    open,
    backendPath,
    xAiApiKey,
    openAiApiKey,
    backendHealth,
    healthCheckPending,
    isDesktop,
    recentWorkspaceCount,
    providers,
    selectedProviderId,
    selectedModel,
    selectedPermission,
    onSelectProvider,
    onSelectModel,
    onSelectPermission,
    onBackendPathChange,
    onXAiApiKeyChange,
    onOpenAiApiKeyChange,
    onRunHealthCheck,
    modelOptions,
    permissionModes,
    onClose,
  }: Props = $props();

  let draftBackendPath = $state("");
  let draftXAiApiKey = $state("");
  let draftOpenAiApiKey = $state("");
  let draftSelectedProviderId = $state("local");
  let draftSelectedModel = $state("");
  let draftSelectedPermission = $state<PermissionMode>("workspace-write");

  let draftModelOptions = $derived(
    providerModelOptions[draftSelectedProviderId] ?? modelOptions
  );
  let hasUnsavedChanges = $derived(
    draftBackendPath !== backendPath ||
      draftXAiApiKey !== xAiApiKey ||
      draftOpenAiApiKey !== openAiApiKey ||
      draftSelectedProviderId !== selectedProviderId ||
      draftSelectedModel !== selectedModel ||
      draftSelectedPermission !== selectedPermission
  );

  $effect(() => {
    if (!open) return;

    draftBackendPath = backendPath;
    draftXAiApiKey = xAiApiKey;
    draftOpenAiApiKey = openAiApiKey;
    draftSelectedProviderId = selectedProviderId;
    draftSelectedModel = selectedModel;
    draftSelectedPermission = selectedPermission;
  });

  function updateDraftProvider(id: string) {
    draftSelectedProviderId = id;
    const nextOptions = providerModelOptions[id] ?? modelOptions;
    if (!nextOptions.includes(draftSelectedModel)) {
      draftSelectedModel = providerDefaultModel[id] ?? nextOptions[0] ?? draftSelectedModel;
    }
  }

  async function applyDraft(closeAfter = false) {
    onBackendPathChange(draftBackendPath);
    onXAiApiKeyChange(draftXAiApiKey);
    onOpenAiApiKeyChange(draftOpenAiApiKey);
    await onSelectProvider(draftSelectedProviderId);
    await onSelectModel(draftSelectedModel);
    await onSelectPermission(draftSelectedPermission);

    if (closeAfter) {
      onClose();
    }
  }

  async function runDraftHealthCheck() {
    await applyDraft(false);
    onRunHealthCheck();
  }
</script>

{#if open}
  <button
    class="ui-overlay fixed inset-0 z-40"
    onclick={onClose}
    aria-label="Close settings sheet"
    type="button"
    transition:fade={{ duration: 140 }}
  ></button>

  <div
    class="fixed left-1/2 top-1/2 z-50 w-[min(900px,calc(100vw-2rem))] -translate-x-1/2 -translate-y-1/2"
    transition:fly={{ duration: 180, y: 18 }}
  >
    <section
      class="ui-sheet overflow-hidden px-5 pb-5 pt-4"
      style="background-color: #111113; border-color: rgb(255 255 255 / 0.08);"
    >
      <div class="flex items-start justify-between gap-4 border-b border-white/[0.05] pb-4">
        <div class="max-w-[34rem]">
          <div class="ui-section-label">Runtime Settings</div>
          <h3 class="mt-2 text-[1.1rem] font-medium leading-[1.2] tracking-[-0.03em] text-soft-ivory">
            Harness and local model defaults
          </h3>
          <p class="mt-2 text-[0.875rem] leading-6 text-fog/56">
            Configure the local runtime, choose a default provider, and keep the
            desktop experience grounded in inspectable behavior.
          </p>
        </div>

        <button
          type="button"
          class="rounded-xl px-3 py-2 text-[0.8125rem] text-fog/52 transition hover:bg-white/[0.03] hover:text-soft-ivory"
          onclick={onClose}
        >
          Close
        </button>
      </div>

      <div class="ui-scrollbar-hidden mt-4 max-h-[72vh] space-y-3 overflow-y-auto pr-1">
      <section class="ui-panel p-4">
        <div class="ui-section-label">Backend</div>
        <label class="mt-4 block">
          <span class="sr-only">Backend path</span>
            <input
              class="ui-input code-font px-4 py-3 text-sm"
              type="text"
              value={draftBackendPath}
              placeholder="harness-server"
              oninput={(event) =>
                (draftBackendPath = (event.currentTarget as HTMLInputElement).value)}
            />
          </label>

        <div class="mt-4 flex flex-wrap gap-2">
          <Button
            label={healthCheckPending ? "Checking..." : "Run Health Check"}
            variant="dark"
            height="h-10"
            disabled={healthCheckPending}
            onclick={runDraftHealthCheck}
          />
          <span
            class={`ui-chip ${
              isDesktop ? "ui-chip-success" : "ui-chip-neutral"
            }`}
          >
            {isDesktop ? "desktop active" : "browser preview"}
          </span>
          <span class="ui-chip ui-chip-neutral">
            {recentWorkspaceCount} workspace(s)
          </span>
          {#if backendHealth}
            <span
              class={`ui-chip ${
                backendHealth.runnable ? "ui-chip-accent" : "ui-chip-neutral"
              }`}
            >
              {backendHealth.status}
            </span>
            <span class="ui-chip ui-chip-neutral">
              {backendHealth.sessionCount} session(s)
            </span>
          {/if}
        </div>

        <div class="ui-panel-soft mt-4 px-4 py-3 text-sm leading-6 text-fog/68">
          {backendHealth?.message ??
            "Run the health check to verify the `harness-server` path and the local session store."}
        </div>

        {#if backendHealth?.resolvedPath}
          <div class="ui-panel-subtle mt-4 px-4 py-3 text-sm text-fog/70">
            <div class="ui-section-label">Resolved binary</div>
            <div class="code-font mt-2 break-all text-[0.8rem]">{backendHealth.resolvedPath}</div>
            {#if backendHealth.version}
              <div class="code-font mt-2 text-[0.68rem] uppercase tracking-[0.16em] text-fog/46">
                {backendHealth.version}
              </div>
            {/if}
          </div>
        {/if}

        {#if backendHealth?.localRuntime}
          <div class="ui-panel-subtle mt-4 px-4 py-4 text-sm text-fog/68">
            <div class="flex flex-wrap gap-2">
              <span
                class={`ui-chip ${
                  backendHealth.localRuntime.reachable ? "ui-chip-success" : "ui-chip-neutral"
                }`}
              >
                {backendHealth.localRuntime.reachable ? "ollama online" : "ollama offline"}
              </span>
              <span
                class={`ui-chip ${
                  backendHealth.localRuntime.hasSelectedModel
                    ? "ui-chip-accent"
                    : "ui-chip-warning"
                }`}
              >
                {backendHealth.localRuntime.hasSelectedModel ? "model ready" : "model missing"}
              </span>
              {#if backendHealth.localRuntime.version}
                <span class="ui-chip ui-chip-neutral">
                  Ollama {backendHealth.localRuntime.version}
                </span>
              {/if}
            </div>

            <div class="mt-3 leading-7">{backendHealth.localRuntime.message}</div>

            {#if backendHealth.localRuntime.availableModels.length}
              <div class="ui-section-label mt-4">Installed models</div>
              <div class="mt-2 flex flex-wrap gap-2">
                {#each backendHealth.localRuntime.availableModels as model}
                  <span class="ui-chip code-font border border-white/8 bg-white/4 text-fog/56">
                    {model}
                  </span>
                {/each}
              </div>
            {/if}
          </div>
        {/if}

        {#if draftSelectedProviderId === "local"}
          <div class="ui-panel-accent mt-4 px-4 py-4 text-sm leading-7 text-fog/70">
            Local mode expects Ollama on `http://127.0.0.1:11434` with a Gemma 4
            model available. `ollama pull gemma4:e2b` is still the easiest first
            boot path.
          </div>
        {:else if draftSelectedProviderId === "grok"}
          <div class="ui-panel-accent mt-4 px-4 py-4 text-sm leading-7 text-fog/70">
            Grok mode uses the hosted xAI API through the sibling Harness. Save a
            valid `XAI_API_KEY` here, then run a health check before launching.
          </div>
        {/if}
      </section>

      <section class="ui-panel p-4">
        <div class="ui-section-label">API Keys</div>
        <div class="mt-4 grid gap-3">
          <label class="block">
            <div class="mb-2 text-[0.875rem] text-fog/74">xAI / Grok</div>
            <input
              class="ui-input code-font px-4 py-3 text-sm"
              type="password"
              value={draftXAiApiKey}
              placeholder="xai-..."
              autocomplete="off"
              spellcheck="false"
              oninput={(event) =>
                (draftXAiApiKey = (event.currentTarget as HTMLInputElement).value)}
            />
          </label>

          <label class="block">
            <div class="mb-2 text-[0.875rem] text-fog/74">OpenAI / ChatGPT</div>
            <input
              class="ui-input code-font px-4 py-3 text-sm"
              type="password"
              value={draftOpenAiApiKey}
              placeholder="sk-..."
              autocomplete="off"
              spellcheck="false"
              oninput={(event) =>
                (draftOpenAiApiKey = (event.currentTarget as HTMLInputElement).value)}
            />
          </label>
        </div>

        <div class="ui-panel-soft mt-4 px-4 py-3 text-sm leading-6 text-fog/64">
          Saved locally in Poro desktop settings and loaded into the app process for provider use.
        </div>
      </section>

      <section class="ui-panel p-4">
        <div class="ui-section-label">Provider</div>
        <div class="mt-4 space-y-2">
          {#each providers as provider}
            <button
              type="button"
              class={`ui-panel-soft w-full px-4 py-4 text-left transition ${
                draftSelectedProviderId === provider.id
                  ? "border-accent-gold/18 bg-accent-gold/[0.05]"
                  : "hover:border-white/14 hover:bg-white/[0.05]"
              }`}
              onclick={() => updateDraftProvider(provider.id)}
            >
              <div class="flex items-center justify-between gap-4">
                <div>
                  <div class="type-heading-4 uppercase tracking-[0.18em] text-soft-ivory">
                    {provider.label}
                  </div>
                  <div class="code-font mt-2 text-[0.72rem] uppercase tracking-[0.16em] text-fog/46">
                    {provider.endpoint}
                  </div>
                </div>
                <span class="ui-chip ui-chip-neutral">
                  {provider.status}
                </span>
              </div>
            </button>
          {/each}
        </div>
      </section>

      <section class="ui-panel p-4">
        <div class="ui-section-label">Default Model</div>
        <div class="mt-4 flex flex-wrap gap-2">
          {#each draftModelOptions as model}
            <button
              type="button"
              class={`code-font rounded-xl border px-3 py-2 text-[0.68rem] uppercase tracking-[0.18em] transition ${
                draftSelectedModel === model
                  ? "border-accent-gold/35 bg-accent-gold/10 text-accent-gold"
                  : "border-white/10 bg-dark-slate/90 text-fog/58 hover:border-white/16 hover:bg-white/[0.05]"
              }`}
              onclick={() => (draftSelectedModel = model)}
            >
              {model}
            </button>
          {/each}
        </div>
      </section>

      <section class="ui-panel p-4">
        <div class="ui-section-label">Default Permission</div>
        <div class="mt-4 flex flex-wrap gap-2">
          {#each permissionModes as mode}
            <button
              type="button"
              class={`code-font rounded-xl border px-3 py-2 text-[0.68rem] uppercase tracking-[0.18em] transition ${
                draftSelectedPermission === mode
                  ? mode === "danger-full-access"
                    ? "border-red-400/30 bg-red-400/10 text-red-200"
                    : mode === "workspace-write"
                      ? "border-misty-green/28 bg-misty-green/10 text-misty-green"
                      : "border-white/16 bg-white/8 text-soft-ivory"
                  : "border-white/10 bg-dark-slate/90 text-fog/58 hover:border-white/16 hover:bg-white/[0.05]"
              }`}
              onclick={() => (draftSelectedPermission = mode)}
            >
              {mode}
            </button>
          {/each}
        </div>
      </section>

      <section class="ui-panel p-4">
        <div class="ui-section-label">Review Defaults</div>
        <div class="mt-4 grid gap-3">
          <div class="ui-panel-subtle px-4 py-4">
            <div class="type-heading-4 uppercase tracking-[0.14em] text-soft-ivory">
              Always show diff before apply
            </div>
            <div class="mt-2 type-body-4 text-fog/64">
              Kept visible by default so the desktop experience feels inspectable
              instead of magical.
            </div>
          </div>
          <div class="ui-panel-subtle px-4 py-4">
            <div class="type-heading-4 uppercase tracking-[0.14em] text-soft-ivory">
              Tool timeline density
            </div>
            <div class="mt-2 type-body-4 text-fog/64">
              Balanced. Enough event detail to trust the harness without turning
              the app into a scrolling transcript dump.
            </div>
          </div>
        </div>
      </section>

      <div class="flex items-center justify-end gap-2 border-t border-white/[0.05] pt-4">
        <button
          type="button"
          class="rounded-xl px-3 py-2 text-[0.8125rem] text-fog/52 transition hover:bg-white/[0.03] hover:text-soft-ivory"
          onclick={onClose}
        >
          Cancel
        </button>
        <Button
          label="Save"
          variant="gold"
          height="h-10"
          disabled={!hasUnsavedChanges}
          onclick={() => applyDraft(true)}
        />
      </div>
      </div>
    </section>
  </div>
{/if}
