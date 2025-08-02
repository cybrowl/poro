<script>
	let {message = 'ICP will be automatically exchanged for ckUSDC.'} = $props();
</script>

<div class="banner-wrapper">
	<div class="banner">
		<div class="banner-edge"></div>
		<span class="banner-text">{message}</span>
	</div>
</div>

<style lang="postcss">
	.banner-wrapper {
		@apply inline-block;
	}

	.banner {
		@apply inline-flex items-center rounded-lg overflow-hidden bg-carbon-black bg-opacity-50 border border-white border-opacity-10;
		@apply h-20;
	}

	.banner-edge {
		@apply w-4 h-full bg-merigold rounded-l-lg;
	}

	.banner-text {
		@apply text-white text-base font-sans mx-6;
	}
</style>
