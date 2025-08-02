<script>
	import Icon from '../basic_elements/Icon.svelte';

	let { selected_icon = 'gpt' } = $props();
	let selectedIcon = $state(selected_icon); // Make selected_icon reactive

	const icons = [
		{
			selectedName: 'gpt_gold',
			unselectedName: 'gpt_white',
			size: '1.5rem',
			viewSize: { width: 24, height: 24 },
			class: 'fill_none'
		},
		{
			selectedName: 'line', // No gold version since it's not clickable
			unselectedName: 'line',
			size: '1.5rem',
			viewSize: { width: 24, height: 24 },
			clickable: false,
			class: 'fill_white'
		},
		{
			selectedName: 'history_gold',
			unselectedName: 'history_white',
			size: '1.5rem',
			viewSize: { width: 24, height: 24 }
		},
		{
			selectedName: 'wallet_gold',
			unselectedName: 'wallet_white',
			size: '1.5rem',
			viewSize: { width: 24, height: 24 }
		},
		{
			selectedName: 'settings_gold',
			unselectedName: 'settings_white',
			size: '1.5rem',
			viewSize: { width: 24, height: 24 }
		},
		{
			selectedName: 'new_session_white', // No gold version needed
			unselectedName: 'new_session_white',
			size: '2rem',
			viewSize: { width: 32, height: 32 },
			class: 'fill_none ml-6'
		}
	];

	function select_icon(icon_name) {
		selectedIcon = icon_name; // Update the reactive state
	}

	function getIconName(icon) {
		return selectedIcon === icon.selectedName ? icon.selectedName : icon.unselectedName;
	}
</script>

<div class="main">
	{#each icons as icon}
		<Icon
			name={getIconName(icon)}
			class={icon.class || ''}
			viewSize={icon.viewSize}
			size={icon.size}
			clickable={icon.clickable !== false}
			on:icon-click={() => icon.clickable !== false && select_icon(icon.selectedName)}
		/>
	{/each}
</div>

<style lang="postcss">
	.main {
		@apply flex flex-row gap-6 items-center;
	}
</style>
