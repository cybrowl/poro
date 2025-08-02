<script>
	export let coinType = 'ckUSDC';
	export let amount = 0;

	function sanitize(event) {
		const el = event.target;

		let cleaned = el.value.replace(/[^\d.]/g, '').replace(/(\..*?)\./g, '$1');

		if (el.value !== cleaned) el.value = cleaned;

		amount = cleaned === '' ? 0 : Number(cleaned);
	}
</script>

<div class="balance-info">
	<input
		class="balance"
		type="text"
		bind:value={amount}
		on:input={sanitize}
		inputmode="decimal"
		autocomplete="off"
		pattern="[0-9]*[.]?[0-9]*"
		style="width: {String(amount).length || 1}ch"
	/>
	<span class="token">{coinType}</span>
</div>

<style lang="postcss">
	.balance-info {
		@apply flex items-baseline gap-6 text-8xl;
	}

	.balance {
		@apply text-white bg-transparent border-none outline-none;
		-moz-appearance: textfield;
	}

	.token {
		@apply text-midnight-slate;
	}
</style>
