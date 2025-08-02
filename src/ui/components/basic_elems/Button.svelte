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
		children = undefined
	} = $props();

	const variants = {
		dark: 'bg-graphite text-white hover:bg-graphite/90',
		gold: 'bg-marigold text-black hover:bg-marigold/90'
	};

	let buttonClasses = $derived(
		`inline-flex items-center justify-center px-4 py-2 rounded-md font-medium focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-${variant === 'dark' ? 'graphite' : 'marigold'} ${variants[variant] || variants.dark} ${width} ${height} ${disabled ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer'} ${customClass}`
	);
</script>

<button class={buttonClasses} {onclick} {disabled} type="button">
	{#if children}
		{@render children()}
	{:else}
		{label}
	{/if}
</button>
