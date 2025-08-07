<!-- MainToolbar.svelte -->
<script>
  import Icon from "../basic_elems/Icon.svelte";

  let { selected_icon = "gpt" } = $props();
  let selectedIcon = $state(selected_icon);

  const icons = [
    {
      selectedName: "gpt_gold",
      unselectedName: "gpt_white",
      size: "1.5rem",
      viewSize: { width: 24, height: 24 },
      class: "fill_none",
    },
    {
      selectedName: "line",
      unselectedName: "line",
      size: "1.5rem",
      viewSize: { width: 24, height: 24 },
      clickable: false,
      class: "fill_white",
    },
    {
      selectedName: "history_gold",
      unselectedName: "history_white",
      size: "1.5rem",
      viewSize: { width: 24, height: 24 },
    },
    {
      selectedName: "wallet_gold",
      unselectedName: "wallet_white",
      size: "1.5rem",
      viewSize: { width: 24, height: 24 },
    },
    {
      selectedName: "settings_gold",
      unselectedName: "settings_white",
      size: "1.5rem",
      viewSize: { width: 24, height: 24 },
    },
    {
      selectedName: "new_session_white",
      unselectedName: "new_session_white",
      size: "2rem",
      viewSize: { width: 32, height: 32 },
      class: "fill_none ml-6",
    },
  ];

  function select_icon(icon_name) {
    selectedIcon = icon_name;
  }

  function getIconName(icon) {
    return selectedIcon === icon.selectedName
      ? icon.selectedName
      : icon.unselectedName;
  }
</script>

<div class="main">
  {#each icons as icon}
    <Icon
      name={getIconName(icon)}
      class={icon.class || ""}
      viewSize={icon.viewSize}
      size={icon.size}
      clickable={icon.clickable !== false}
      onIconClick={() =>
        icon.clickable !== false && select_icon(icon.selectedName)}
    />
  {/each}
</div>

<style lang="postcss">
  .main {
    @apply flex flex-row gap-6 items-center;
  }
</style>
