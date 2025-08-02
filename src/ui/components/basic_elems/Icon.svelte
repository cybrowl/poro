<script>
	import * as icons from './icons';
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	let {
		name,
		clickable = true,
		focusable = false,
		viewSize = { width: 24, height: 24 },
		size = '2rem',
		scale = 1,
		class: className = ''
	} = $props();

	let viewbox = $derived({
		width: viewSize.width / scale,
		height: viewSize.height / scale
	});

	function onClick(event) {
		if (clickable) {
			event.stopPropagation();
			dispatch('icon-click', { name });
		}
	}
</script>

{#snippet IconSvg()}
	<svg
		class={className}
		width={size}
		height={size}
		viewBox={`0 0 ${viewbox.width} ${viewbox.height}`}
	>
		{@html icons[name]}
	</svg>
{/snippet}

{#if clickable}
	<button onclick={onClick}>
		{@render IconSvg()}
	</button>
{:else}
	<span>
		{@render IconSvg()}
	</span>
{/if}

<style lang="postcss">
	.cursor_pointer {
		@apply cursor-pointer;
	}
	.cursor_default {
		@apply cursor-default;
	}
	.responsive_icon {
		@apply w-16 h-16 lg:w-11 lg:h-11 3xl:w-11 3xl:h-11;
	}
	.fill_none {
		@apply fill-none;
	}
	.stroke_white {
		@apply stroke-white;
	}
	.stroke_gray {
		@apply stroke-silver-mist;
	}
	.stroke_gold {
		@apply stroke-merigold;
	}
	.fill_white {
		@apply fill-white;
	}
	.fill_gold {
		@apply fill-merigold;
	}
	svg:focus {
		outline: none;
		box-shadow: none;
	}
</style>
