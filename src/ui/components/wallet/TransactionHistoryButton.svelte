<!-- TransactionHistoryButton.svelte -->
<script>
	import Icon from '../basic_elems/Icon.svelte';

	let { title = 'Transaction History' } = $props();
</script>

<div class="flex items-center h-12 text-white cursor-pointer">
	<div
		class="flex items-center justify-center w-full hover:text-marigold transition-colors duration-200"
	>
		<span class="text-base font-medium pr-3">{title}</span>
		<Icon
			name="arrow_right"
			class="cursor-pointer fill-white"
			size="1rem"
			viewSize={{ width: 7, height: 12 }}
		/>
	</div>
</div>
