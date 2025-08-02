<script>
	import Icon from '../basic_elements/Icon.svelte';

	let {
		value = '',
		placeholder = '',
		icon = 'bank',
		width = 'w-full',
		disabled = false,
		class: customClass = '',
		oninput = () => {}
	} = $props();

	function handleInput(event) {
		value = event.target.value;
		oninput(value);
	}
</script>

<div class={`${width} ${customClass}`}>
	<p class="label">Address To Transfer</p>
	<div class={`input-with-icon`}>
		<Icon
			name={icon}
			clickable={false}
			class="stroke_gray"
			size="2rem"
			viewSize={{width: 32, height: 32}}
		/>

		<input
			class="text-input"
			type="text"
			{placeholder}
			bind:value
			{disabled}
			oninput={handleInput}
		/>
	</div>
</div>

<style lang="postcss">
	.input-with-icon {
		@apply flex items-center rounded-lg border border-silver-mist px-6 py-4;
	}

	.label {
		@apply text-white text-base mb-2;
	}

	.text-input {
		@apply flex-1 bg-transparent text-white placeholder-gray-500 outline-none border-none font-sans text-base pl-4;
	}
</style>
