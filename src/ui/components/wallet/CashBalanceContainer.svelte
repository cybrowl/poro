<script>
	import Icon from '../basic_elements/Icon.svelte';

	let {
		amount = '0',
		currency = 'ckUSDC',
		promptLabel = 'Add Cash To',
		onAddCash = () => {}
	} = $props();

	let currencyLabel = $derived(`United States Dollar (${currency})`);
</script>

<div class="cash-balance-container">
	<div class="cash-icon">
		<Icon
			name="ckUSDC"
			class="cursor_default"
			size="3.125rem"
			viewSize={{width: 50, height: 50}}
		/>
	</div>

	<div class="cash-details">
		<p class="prompt-label">{promptLabel}</p>
		<p class="currency-label">{currencyLabel}</p>
	</div>

	<div class="cash-amount">
		<p class="amount-label">${amount}</p>
		<p class="status-label">Available</p>
	</div>
</div>

<style lang="postcss">
	.cash-balance-container {
		@apply flex items-center justify-between w-full h-12;
	}

	.cash-icon {
		@apply mr-4 flex-shrink-0;
	}

	.cash-details {
		@apply flex flex-col justify-center flex-1;
	}

	.prompt-label {
		@apply text-white text-base font-bold leading-none;
	}

	.currency-label {
		@apply text-silver-mist text-base font-normal leading-tight mt-1;
	}

	.cash-amount {
		@apply flex flex-col items-end justify-center;
	}

	.amount-label {
		@apply text-silver-mist text-xl font-bold leading-none;
	}

	.status-label {
		@apply text-silver-mist text-base font-normal leading-tight mt-1;
	}
</style>
