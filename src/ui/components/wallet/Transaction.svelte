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

<div class={`flex items-start justify-between w-full min-h-12 ${customClass}`}>
	<div class="flex flex-col gap-1 text-left">
		<p class="font-sans font-bold text-xl text-white">{status}</p>
		{#if description}
			<p class="font-sans text-base text-silver-mist">{description}</p>
		{/if}
		{#if date}
			<p class="font-sans text-base text-silver-mist">{date}</p>
		{/if}
	</div>
	<div class="flex flex-col gap-1 text-right">
		{#if fiatAmount}
			<p class="font-sans text-base font-bold text-misty-green">{fiatAmount}</p>
		{/if}
		{#if cryptoAmount}
			<p class="font-sans text-base text-white">{cryptoAmount}</p>
		{/if}
		{#if cashOutAmount}
			<p class="font-sans text-base text-yellow-400">{cashOutAmount}</p>
		{/if}
	</div>
</div>
