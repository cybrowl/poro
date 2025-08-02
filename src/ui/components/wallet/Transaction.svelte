<script>
	/**
	 * TransactionRow – single entry for the wallet transaction list.
	 * Matches the visual shown in the mock‑up.
	 *
	 * Props
	 *  • status         – e.g. "Deposited" / "Withdrawn"
	 *  • description    – secondary line (e.g. "Converted to Dollar (ckUSDC)")
	 *  • date           – human‑readable date string
	 *  • fiatAmount     – formatted fiat value (green)
	 *  • cryptoAmount   – formatted crypto value (white)
	 *  • class          – extra classes for root element
	 */
	let {
		status = 'Deposited',
		description = '',
		date = '',
		fiatAmount = '',
		cryptoAmount = '',
		cashOutAmount = '',
		class: customClass = ''
	} = $props();
</script>

<!-- container -->
<div class={`transaction-row ${customClass}`}>
	<!-- left column -->
	<div class="left-col">
		<p class="status">{status}</p>
		{#if description}
			<p class="description">{description}</p>
		{/if}

		{#if date}
			<p class="date">{date}</p>
		{/if}
	</div>

	<!-- right column -->
	<div class="right-col text-right">
		{#if fiatAmount}
			<p class="fiat">{fiatAmount}</p>
		{/if}

		{#if cryptoAmount}
			<p class="crypto">{cryptoAmount}</p>
		{/if}

		{#if cashOutAmount}
			<p class="cash-out">{cashOutAmount}</p>
		{/if}
	</div>
</div>

<style lang="postcss">
	.transaction-row {
		@apply flex items-center justify-between w-full h-12;
	}

	.status {
		@apply font-sans font-bold text-xl text-white;
	}

	p {
		@apply font-sans text-base mt-1;
	}

	.fiat {
		@apply font-bold text-misty-green;
	}

	.description,
	.date {
		@apply text-silver-mist;
	}

	.crypto {
		@apply text-white;
	}

	.cash-out {
		@apply text-yellow-400;
	}
</style>
