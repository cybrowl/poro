<!-- Button.svelte -->
<script>
	let {
		label = 'Button',
		variant = 'dark',
		width = 'w-auto',
		height = 'h-14',
		onclick = () => {},
		disabled = false,
		class: customClass = '',
		children
	} = $props();

	const variants = {
		dark: 'bg-deep-charcoal text-white hover:bg-dark-slate/90',
		gold: 'bg-merigold text-black hover:bg-merigold/90'
	};

	let buttonClasses = $derived(
		`inline-flex items-center justify-center px-4 py-2 rounded-md font-medium focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-${variant === 'dark' ? 'graphite' : 'merigold'} ${variants[variant] || variants.dark} ${width} ${height} ${disabled ? 'opacity-50 cursor-not-allowed' : ''} ${customClass}`
	);
</script>

<button class={buttonClasses} {onclick} {disabled} type="button">
	{#if children}
		{@render children()}
	{:else}
		{label}
	{/if}
</button>
