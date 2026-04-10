<script lang="ts">
  import Button from "$components/basic_elems/Button.svelte";
  import type { BackendHealth } from "$lib/clawRuntime";
  import type { PermissionMode, ProviderRecord } from "$lib/mockDesktopData";
  import { fade, fly } from "svelte/transition";

  interface Props {
    open: boolean;
    backendPath: string;
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
    onRunHealthCheck: () => void;
    modelOptions: string[];
    permissionModes: PermissionMode[];
    onClose: () => void;
  }

  let {
    open,
    backendPath,
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
    onRunHealthCheck,
    modelOptions,
    permissionModes,
    onClose,
  }: Props = $props();
</script>

{#if open}
  <button
    class="fixed inset-0 z-40 bg-black/55 backdrop-blur-sm"
    onclick={onClose}
    aria-label="Close settings sheet"
    type="button"
    transition:fade={{ duration: 140 }}
  ></button>

  <aside
    class="fixed right-4 top-4 z-50 h-[calc(100vh-2rem)] w-[min(560px,calc(100vw-2rem))] overflow-y-auto rounded-[22px] border border-white/10 bg-[#0f141c]/97 p-5 shadow-[0_32px_120px_rgba(0,0,0,0.58)]"
    transition:fly={{ duration: 180, x: 30 }}
  >
    <div class="flex items-start justify-between gap-4">
      <div>
        <div class="font-mono text-[0.66rem] uppercase tracking-[0.34em] text-fog/48">
          Runtime Settings
        </div>
        <h3 class="mt-3 text-[2rem] font-medium leading-none tracking-[-0.05em] text-soft-ivory">
          Harness and local model defaults
        </h3>
        <p class="mt-3 max-w-[44ch] text-sm leading-7 text-fog/68">
          This panel now reflects the sibling Harness integration. Local mode is
          the first-class path, with health checks for both the server binary and
          the local Ollama daemon.
        </p>
      </div>

      <Button label="Close" variant="outline" height="h-10" onclick={onClose} />
    </div>

    <div class="mt-6 space-y-4">
      <section class="rounded-[16px] border border-white/8 bg-carbon-black/70 p-4">
        <div class="font-mono text-[0.66rem] uppercase tracking-[0.32em] text-fog/46">
          Backend
        </div>
        <label class="mt-4 block">
          <span class="sr-only">Backend path</span>
          <input
            class="w-full rounded-[12px] border border-white/8 bg-dark-slate/90 px-4 py-3 font-mono text-sm text-fog/80 outline-none transition focus:border-signal-blue/35"
            type="text"
            value={backendPath}
            placeholder="harness-server"
            oninput={(event) =>
              onBackendPathChange((event.currentTarget as HTMLInputElement).value)}
          />
        </label>

        <div class="mt-4 flex flex-wrap gap-2">
          <Button
            label={healthCheckPending ? "Checking..." : "Run Health Check"}
            variant="gold"
            height="h-10"
            disabled={healthCheckPending}
            onclick={onRunHealthCheck}
          />
          <span
            class={`rounded-full px-3 py-2 font-mono text-[0.62rem] uppercase tracking-[0.18em] ${
              isDesktop ? "bg-misty-green/12 text-misty-green" : "bg-white/6 text-fog/56"
            }`}
          >
            {isDesktop ? "desktop active" : "browser preview"}
          </span>
          <span class="rounded-full bg-white/6 px-3 py-2 font-mono text-[0.62rem] uppercase tracking-[0.18em] text-fog/54">
            {recentWorkspaceCount} workspace(s)
          </span>
          {#if backendHealth}
            <span
              class={`rounded-full px-3 py-2 font-mono text-[0.62rem] uppercase tracking-[0.18em] ${
                backendHealth.runnable
                  ? "bg-signal-blue/12 text-signal-blue"
                  : "bg-white/6 text-fog/56"
              }`}
            >
              {backendHealth.status}
            </span>
            <span class="rounded-full bg-white/6 px-3 py-2 font-mono text-[0.62rem] uppercase tracking-[0.18em] text-fog/54">
              {backendHealth.sessionCount} session(s)
            </span>
          {/if}
        </div>

        <div class="mt-4 rounded-[12px] border border-white/8 bg-white/[0.035] px-4 py-3 text-sm leading-6 text-fog/68">
          {backendHealth?.message ??
            "Run the health check to verify the `harness-server` path and the local session store."}
        </div>

        {#if backendHealth?.resolvedPath}
          <div class="mt-4 rounded-[12px] border border-white/8 bg-dark-slate/90 px-4 py-3 text-sm text-fog/70">
            <div class="font-mono text-[0.62rem] uppercase tracking-[0.22em] text-fog/46">
              Resolved binary
            </div>
            <div class="mt-2 break-all font-mono text-[0.8rem]">{backendHealth.resolvedPath}</div>
            {#if backendHealth.version}
              <div class="mt-2 font-mono text-[0.68rem] uppercase tracking-[0.16em] text-fog/46">
                {backendHealth.version}
              </div>
            {/if}
          </div>
        {/if}

        {#if backendHealth?.localRuntime}
          <div class="mt-4 rounded-[12px] border border-white/8 bg-dark-slate/90 px-4 py-4 text-sm text-fog/68">
            <div class="flex flex-wrap gap-2">
              <span
                class={`rounded-full px-3 py-2 font-mono text-[0.62rem] uppercase tracking-[0.18em] ${
                  backendHealth.localRuntime.reachable
                    ? "bg-misty-green/12 text-misty-green"
                    : "bg-white/6 text-fog/56"
                }`}
              >
                {backendHealth.localRuntime.reachable ? "ollama online" : "ollama offline"}
              </span>
              <span
                class={`rounded-full px-3 py-2 font-mono text-[0.62rem] uppercase tracking-[0.18em] ${
                  backendHealth.localRuntime.hasSelectedModel
                    ? "bg-signal-blue/12 text-signal-blue"
                    : "bg-warning-amber/12 text-warning-amber"
                }`}
              >
                {backendHealth.localRuntime.hasSelectedModel ? "model ready" : "model missing"}
              </span>
              {#if backendHealth.localRuntime.version}
                <span class="rounded-full bg-white/6 px-3 py-2 font-mono text-[0.62rem] uppercase tracking-[0.18em] text-fog/54">
                  Ollama {backendHealth.localRuntime.version}
                </span>
              {/if}
            </div>

            <div class="mt-3 leading-7">{backendHealth.localRuntime.message}</div>

            {#if backendHealth.localRuntime.availableModels.length}
              <div class="mt-4 font-mono text-[0.62rem] uppercase tracking-[0.22em] text-fog/46">
                Installed models
              </div>
              <div class="mt-2 flex flex-wrap gap-2">
                {#each backendHealth.localRuntime.availableModels as model}
                  <span class="rounded-full border border-white/8 bg-white/4 px-3 py-2 font-mono text-[0.62rem] uppercase tracking-[0.18em] text-fog/56">
                    {model}
                  </span>
                {/each}
              </div>
            {/if}
          </div>
        {/if}

        {#if selectedProviderId === "local"}
          <div class="mt-4 rounded-[12px] border border-signal-blue/18 bg-signal-blue/8 px-4 py-4 text-sm leading-7 text-fog/70">
            Local mode expects Ollama on `http://127.0.0.1:11434` with a Gemma 4
            model available. `ollama pull gemma4:e2b` is still the easiest first
            boot path.
          </div>
        {:else if selectedProviderId === "grok"}
          <div class="mt-4 rounded-[12px] border border-signal-blue/18 bg-signal-blue/8 px-4 py-4 text-sm leading-7 text-fog/70">
            Grok mode uses the hosted xAI API through the sibling Harness. Launch
            Poro from a terminal that exports `XAI_API_KEY`, then pick the model
            you want to test.
          </div>
        {/if}
      </section>

      <section class="rounded-[16px] border border-white/8 bg-carbon-black/70 p-4">
        <div class="font-mono text-[0.66rem] uppercase tracking-[0.32em] text-fog/46">
          Provider
        </div>
        <div class="mt-4 space-y-2">
          {#each providers as provider}
            <button
              type="button"
              class={`w-full rounded-[12px] border px-4 py-4 text-left transition ${
                selectedProviderId === provider.id
                  ? "border-signal-blue/30 bg-signal-blue/10"
                  : "border-white/8 bg-white/[0.03] hover:border-white/14 hover:bg-white/[0.05]"
              }`}
              onclick={() => onSelectProvider(provider.id)}
            >
              <div class="flex items-center justify-between gap-4">
                <div>
                  <div class="text-sm font-medium uppercase tracking-[0.18em] text-soft-ivory">
                    {provider.label}
                  </div>
                  <div class="mt-2 font-mono text-[0.72rem] uppercase tracking-[0.16em] text-fog/46">
                    {provider.endpoint}
                  </div>
                </div>
                <span class="rounded-full bg-white/6 px-3 py-1 text-[0.58rem] uppercase tracking-[0.22em] text-fog/54">
                  {provider.status}
                </span>
              </div>
            </button>
          {/each}
        </div>
      </section>

      <section class="rounded-[16px] border border-white/8 bg-carbon-black/70 p-4">
        <div class="font-mono text-[0.66rem] uppercase tracking-[0.32em] text-fog/46">
          Default Model
        </div>
        <div class="mt-4 flex flex-wrap gap-2">
          {#each modelOptions as model}
            <button
              type="button"
              class={`rounded-xl border px-3 py-2 font-mono text-[0.68rem] uppercase tracking-[0.18em] transition ${
                selectedModel === model
                  ? "border-signal-blue/35 bg-signal-blue/10 text-signal-blue"
                  : "border-white/10 bg-dark-slate/90 text-fog/58 hover:border-white/16 hover:bg-white/[0.05]"
              }`}
              onclick={() => onSelectModel(model)}
            >
              {model}
            </button>
          {/each}
        </div>
      </section>

      <section class="rounded-[16px] border border-white/8 bg-carbon-black/70 p-4">
        <div class="font-mono text-[0.66rem] uppercase tracking-[0.32em] text-fog/46">
          Default Permission
        </div>
        <div class="mt-4 flex flex-wrap gap-2">
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
                  : "border-white/10 bg-dark-slate/90 text-fog/58 hover:border-white/16 hover:bg-white/[0.05]"
              }`}
              onclick={() => onSelectPermission(mode)}
            >
              {mode}
            </button>
          {/each}
        </div>
      </section>

      <section class="rounded-[16px] border border-white/8 bg-carbon-black/70 p-4">
        <div class="font-mono text-[0.66rem] uppercase tracking-[0.32em] text-fog/46">
          Review Defaults
        </div>
        <div class="mt-4 grid gap-3">
          <div class="rounded-[12px] border border-white/8 bg-dark-slate/90 px-4 py-4">
            <div class="text-sm font-medium uppercase tracking-[0.14em] text-soft-ivory">
              Always show diff before apply
            </div>
            <div class="mt-2 text-sm leading-6 text-fog/64">
              Kept visible by default so the desktop experience feels inspectable
              instead of magical.
            </div>
          </div>
          <div class="rounded-[12px] border border-white/8 bg-dark-slate/90 px-4 py-4">
            <div class="text-sm font-medium uppercase tracking-[0.14em] text-soft-ivory">
              Tool timeline density
            </div>
            <div class="mt-2 text-sm leading-6 text-fog/64">
              Balanced. Enough event detail to trust the harness without turning
              the app into a scrolling transcript dump.
            </div>
          </div>
        </div>
      </section>
    </div>
  </aside>
{/if}
