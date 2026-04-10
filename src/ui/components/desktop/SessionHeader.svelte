<script lang="ts">
  import Button from "$components/basic_elems/Button.svelte";
  import type { SessionRecord, WorkspaceRecord } from "$lib/mockDesktopData";

  interface Props {
    workspace: WorkspaceRecord;
    session: SessionRecord;
    activeProviderLabel: string;
    onOpenWorkspacePicker: () => void;
    onOpenSettings: () => void;
  }

  let {
    workspace,
    session,
    activeProviderLabel,
    onOpenWorkspacePicker,
    onOpenSettings,
  }: Props = $props();
</script>

<section
  class="rounded-[16px] border border-white/8 bg-[#0d1117] px-4 py-3 shadow-[0_18px_60px_rgba(0,0,0,0.3)] sm:px-5"
>
  <div class="flex flex-col gap-4 xl:flex-row xl:items-start xl:justify-between">
    <div class="min-w-0">
      <div class="flex flex-wrap items-center gap-2">
        <span class="rounded-md border border-signal-blue/24 bg-signal-blue/10 px-2.5 py-1 font-mono text-[0.62rem] uppercase tracking-[0.22em] text-signal-blue">
          Session
        </span>
        <span class="rounded-md border border-white/8 bg-white/4 px-2.5 py-1 font-mono text-[0.62rem] uppercase tracking-[0.22em] text-fog/55">
          {workspace.name}
        </span>
        <span class="rounded-md border border-white/8 bg-white/4 px-2.5 py-1 font-mono text-[0.62rem] uppercase tracking-[0.22em] text-fog/55">
          {session.branch}
        </span>
      </div>

      <div class="mt-3 flex flex-wrap items-center gap-3">
        <h2 class="min-w-0 text-[1.28rem] font-medium leading-none tracking-[-0.035em] text-soft-ivory sm:text-[1.45rem]">
          {session.title}
        </h2>
        <span
          class={`rounded-md px-3 py-1.5 font-mono text-[0.62rem] uppercase tracking-[0.22em] ${
            session.status === "Live"
              ? "bg-misty-green/12 text-misty-green"
              : session.status === "Paused"
                ? "bg-white/6 text-fog/65"
                : "bg-signal-blue/12 text-signal-blue"
          }`}
        >
          {session.status}
        </span>
      </div>

      <p class="mt-2 max-w-3xl text-sm leading-6 text-fog/68">{session.goal}</p>
    </div>

    <div class="flex min-w-0 flex-col gap-3 xl:items-end">
      <div class="flex flex-wrap gap-2">
        <span class="rounded-md border border-white/8 bg-[#0a0d12] px-3 py-1.5 font-mono text-[0.62rem] uppercase tracking-[0.18em] text-fog/55">
          {activeProviderLabel}
        </span>
        <span class="rounded-md border border-white/8 bg-[#0a0d12] px-3 py-1.5 font-mono text-[0.62rem] uppercase tracking-[0.18em] text-fog/55">
          {session.model}
        </span>
        <span class="rounded-md border border-white/8 bg-[#0a0d12] px-3 py-1.5 font-mono text-[0.62rem] uppercase tracking-[0.18em] text-fog/55">
          {session.permission}
        </span>
      </div>

      <div class="flex flex-wrap gap-2">
        <Button
          label="Switch Workspace"
          variant="ghost"
          height="h-10"
          onclick={onOpenWorkspacePicker}
        />
        <Button
          label="Session Settings"
          variant="outline"
          height="h-10"
          onclick={onOpenSettings}
        />
      </div>
    </div>
  </div>
</section>
