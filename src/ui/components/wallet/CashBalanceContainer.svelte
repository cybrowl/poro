<!-- CashBalanceContainer.svelte -->
<script>
	import Icon from '../basic_elems/Icon.svelte';

	let {
		amount = '0',
		currency = 'ckUSDC',
		promptLabel = 'Add Cash To',
		onAddCash = () => {}
	} = $props();

	let currencyLabel = $derived(`United States Dollar (${currency})`);
</script>

<div class="flex items-center justify-between w-full h-12">
	<div class="mr-4 flex-shrink-0">
		<Icon
			name="ckUSDC"
			class="cursor-default"
			size="3.125rem"
			viewSize={{ width: 50, height: 50 }}
		/>
	</div>

	<div class="flex flex-col justify-center flex-1">
		<p class="text-white text-base font-bold leading-none">{promptLabel}</p>
		<p class="text-silver-mist text-base font-normal leading-tight mt-1">{currencyLabel}</p>
	</div>

	<div class="flex flex-col items-end justify-center">
		<p class="text-silver-mist text-xl font-bold leading-none">${amount}</p>
		<p class="text-silver-mist text-base font-normal leading-tight mt-1">Available</p>
	</div>
</div>
