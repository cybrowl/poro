<!-- ButtonSignOn.svelte -->
<script>
	import Icon from './Icon.svelte';

	let {
		label = 'Continue with Google',
		logo_name = 'google_logo',
		logo_view_size = { width: 32, height: 32 },
		logo_size = '2rem',
		variant = 'none'
	} = $props();

	let buttonClasses = $derived(
		`sign_in_button ${variant === 'highlighted' ? 'bg-graphite' : ''}`
	);
</script>

<button class={buttonClasses}>
	<Icon
		name={logo_name}
		clickable={false}
		class="cursor-pointer"
		size={logo_size}
		scale={1}
		viewSize={logo_view_size}
	/>
	<p>{label}</p>
</button>

<style lang="postcss">
	.sign_in_button {
		@apply font-sans flex items-center w-80 gap-6 p-4 text-xl text-white border-solid rounded-xl border border-white/25;
	}
</style>
