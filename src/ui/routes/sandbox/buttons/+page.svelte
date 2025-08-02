<script>
	import Button from '$components/basic_elems/Button.svelte';
	import ButtonSignOn from '$components/basic_elems/ButtonSignOn.svelte';
</script>

<main
	class="bg-deep-charcoal flex items-center justify-center h-screen bg-gradient-to-r from-deep-charcoal to-dark-slate"
>
	<div class="flex flex-col items-center gap-8 text-center text-white max-w-md p-4">
		<h1 class="text-3xl font-bold mb-4">Component Visual Guide</h1>

		<section class="w-full flex flex-col gap-4">
			<h2 class="text-2xl font-semibold">Buttons</h2>
			<p class="text-sm text-silver-mist mb-2">
				Standard buttons with dark and gold variants.
			</p>
			<Button label="Dark Button" variant="dark" />
			<Button label="Gold Button" variant="gold" />
			<Button label="Custom Width" variant="dark" width="w-full" />
		</section>

		<section class="w-full flex flex-col gap-4">
			<h2 class="text-2xl font-semibold">Sign-On Buttons</h2>
			<p class="text-sm text-silver-mist mb-2">
				Sign-on buttons with standard and highlighted variants.
			</p>
			<ButtonSignOn label="Continue with Google" variant="none" />
			<ButtonSignOn label="Highlighted Sign-On" variant="highlighted" />
		</section>
	</div>
</main>
