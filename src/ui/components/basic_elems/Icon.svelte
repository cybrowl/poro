<!-- Icon.svelte -->
<script>
	import * as icons from './icons';

	let {
		name,
		clickable = true,
		viewSize = { width: 24, height: 24 },
		size = '2rem',
		scale = 1,
		class: className = '',
		onIconClick = () => {}
	} = $props();

	let viewbox = $derived({
		width: viewSize.width / scale,
		height: viewSize.height / scale
	});

	function handleClick(event) {
		event.stopPropagation();
		onIconClick(name);
	}
</script>

{#snippet IconSvg()}
	<svg
		class="{className} focus:outline-none focus:shadow-none"
		width={size}
		height={size}
		viewBox={`0 0 ${viewbox.width} ${viewbox.height}`}
	>
		{@html icons[name]}
	</svg>
{/snippet}

{#if clickable}
	<button onclick={handleClick}>
		{@render IconSvg()}
	</button>
{:else}
	<span>
		{@render IconSvg()}
	</span>
{/if}
