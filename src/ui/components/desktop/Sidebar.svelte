<script lang="ts">
  import logoUrl from "../../assets/logo.svg";
  import folderUrl from "../../assets/folder.svg";
  import PoroIcon from "../basic_elems/PoroIcon.svelte";
  import PoroIconButton from "../basic_elems/PoroIconButton.svelte";

  interface Props {
    onOpenWorkspaceSwitcher: () => void;
  }

  let { onOpenWorkspaceSwitcher }: Props = $props();
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
      iconSrc={folderUrl}
      variant="bare"
      iconSize="1.15rem"
      buttonSize="2rem"
      ariaLabel="Open workspace and session switcher"
      title="Open workspace and session switcher"
      onclick={onOpenWorkspaceSwitcher}
      iconClass="opacity-72"
    />
  </div>
  <div class="flex-1"></div>
</aside>
