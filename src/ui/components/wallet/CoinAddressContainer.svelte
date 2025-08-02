<script>
	import Icon from '../basic_elements/Icon.svelte';
	import Button from '../basic_elements/Button.svelte';

	let {
		coinType = 'ICP',
		address = '87a4427b0ae47c3a92f0f2132a98f9a5d69d5ecefa97ad9ac3919766bbae85ac',
		onCopy = () => {}
	} = $props();

	const coinIconMap = {
		ICP: 'internet_computer_protocol',
		BTC: 'bitcoin',
		ETH: 'ethereum'
	};

	let copied = $state(false);

	async function handleCopy() {
		try {
			await navigator.clipboard.writeText(address);
			copied = true;
			setTimeout(() => (copied = false), 2000);
			onCopy();
		} catch (error) {
			console.error('Copy failed:', error);
		}
	}
</script>

{#snippet AddressInfo()}
	<div class="address-info">
		<p class="label">{coinType} Address</p>
		<p class="address-text" title={address}>{address}</p>
	</div>
{/snippet}

<div class="coin-address-container">
	<div class="coin-icon">
		<Icon
			name={coinIconMap[coinType] || 'internet_computer_protocol'}
			class="cursor_default"
			size="3.125rem"
			viewSize={{width: 50, height: 50}}
		/>
	</div>

	{@render AddressInfo()}

	<Button
		class="copy-button"
		label="Ok"
		variant="dark"
		--width="w-20"
		height="h-14"
		onclick={handleCopy}
	>
		{#if copied}
			Copied!
		{:else}
			Copy
		{/if}
	</Button>
</div>

<style lang="postcss">
	.coin-address-container {
		@apply flex items-center w-full h-14 gap-4;
	}

	.coin-icon {
		@apply flex-shrink-0; /* Prevent icon from shrinking */
	}

	.address-info {
		@apply flex flex-col justify-center min-w-0 flex-grow; /* min-w-0 allows truncation */
	}

	.label {
		@apply text-lg text-white;
	}

	.address-text {
		@apply font-mono text-sm text-silver-mist truncate;
		@apply hover:overflow-visible hover:whitespace-normal;
	}
</style>
