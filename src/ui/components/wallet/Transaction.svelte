<!-- Transaction.svelte -->
<script>
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

<div class={`flex items-center justify-between w-full h-12 ${customClass}`}>
	<div class="left-col">
		<p class="font-sans font-bold text-xl text-white">{status}</p>
		{#if description}
			<p class="font-sans text-base mt-1 text-silver-mist">{description}</p>
		{/if}

		{#if date}
			<p class="font-sans text-base mt-1 text-silver-mist">{date}</p>
		{/if}
	</div>

	<div class="right-col text-right">
		{#if fiatAmount}
			<p class="font-sans text-base mt-1 font-bold text-misty-green">{fiatAmount}</p>
		{/if}

		{#if cryptoAmount}
			<p class="font-sans text-base mt-1 text-white">{cryptoAmount}</p>
		{/if}

		{#if cashOutAmount}
			<p class="font-sans text-base mt-1 text-yellow-400">{cashOutAmount}</p>
		{/if}
	</div>
</div>
