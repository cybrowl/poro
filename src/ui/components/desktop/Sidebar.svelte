<script lang="ts">
  import logoUrl from "../../assets/logo.svg";
  import historyUrl from "../../assets/history.svg";
  import PoroIcon from "../basic_elems/PoroIcon.svelte";
  import PoroIconButton from "../basic_elems/PoroIconButton.svelte";

  interface Props {
    onPickWorkspace: () => void;
    onOpenWorkspacePicker: () => void;
    onOpenSessionSwitcher: () => void;
    onOpenSettings: () => void;
  }

  let {
    onPickWorkspace,
    onOpenWorkspacePicker,
    onOpenSessionSwitcher,
    onOpenSettings,
  }: Props = $props();
</script>

<aside
  class="flex min-h-0 w-full shrink-0 flex-col border-r border-white/6 bg-deep-charcoal lg:h-screen lg:w-[248px]"
>
  <div class="flex items-center justify-between px-4 py-3">
    <PoroIcon
      src={logoUrl}
      alt="Poro"
      decorative={false}
      size="2.25rem"
      class="w-auto"
      fit="contain"
    />
    <PoroIconButton
      iconSrc={historyUrl}
      variant="bare"
      iconSize="1.15rem"
      buttonSize="2rem"
      ariaLabel="Open session switcher"
      title="Open session switcher"
      onclick={onOpenSessionSwitcher}
      iconClass="opacity-72"
    />
  </div>

  <div class="px-3 pb-2">
    <div class="space-y-1">
      <button
        type="button"
        class="flex w-full items-center rounded-md px-3 py-2 text-left type-body-4 text-fog/72 transition hover:bg-white/[0.035] hover:text-soft-ivory"
        onclick={onPickWorkspace}
      >
        Open workspace
      </button>
      <button
        type="button"
        class="flex w-full items-center rounded-md px-3 py-2 text-left type-body-4 text-fog/72 transition hover:bg-white/[0.035] hover:text-soft-ivory"
        onclick={onOpenWorkspacePicker}
      >
        Search
      </button>
      <button
        type="button"
        class="flex w-full items-center rounded-md px-3 py-2 text-left type-body-4 text-fog/72 transition hover:bg-white/[0.035] hover:text-soft-ivory"
        onclick={onOpenSettings}
      >
        Settings
      </button>
    </div>
  </div>

  <div class="border-t border-white/6"></div>
  <div class="flex-1"></div>
</aside>
