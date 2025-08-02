<script>
	import Icon from '../basic_elements/Icon.svelte';

	let {title = 'Transaction History'} = $props();
</script>

<div class="transaction-history-container">
	<div class="transaction-header">
		<span class="header-title">{title}</span>
		<Icon
			name="arrow_right"
			class="cursor_pointer fill_white"
			size="1rem"
			viewSize={{width: 7, height: 12}}
		/>
	</div>
</div>

<style lang="postcss">
	.transaction-history-container {
		@apply flex items-center h-12 text-white cursor-pointer;
	}

	.transaction-header {
		@apply flex items-center justify-center w-full hover:text-merigold transition-colors duration-200;
	}

	.header-title {
		@apply text-base font-medium pr-3;
	}
</style>
