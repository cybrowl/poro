<script>
	let {checked = true} = $props();

	function handleToggle(newState) {
		checked = newState;
	}
</script>

<div class="toggle-switch">
	<button
		class="toggle-button {checked ? 'active' : ''}"
		onclick={() => handleToggle(true)}
	>
		Deposit
	</button>
	<button
		class="toggle-button {!checked ? 'active' : ''}"
		onclick={() => handleToggle(false)}
	>
		Cash Out
	</button>
</div>

<style lang="postcss">
	.toggle-switch {
		@apply inline-flex;
	}

	.toggle-button {
		@apply font-sans font-bold text-2xl px-10 py-3 rounded-full transition-all duration-300 ease-in-out focus:outline-none;
		@apply bg-transparent text-white cursor-pointer;
		@apply relative z-10;
	}

	.toggle-button.active {
		@apply bg-graphite text-white shadow-lg transform translate-x-0 z-20;
	}

	.toggle-button:first-child {
		@apply rounded-l-full;
		@apply -mr-[10%];
	}

	.toggle-button:last-child {
		@apply rounded-r-full;
	}

	.toggle-button:not(.active) {
		@apply bg-white text-carbon-black z-10;
	}

	.toggle-button:hover:not(.active) {
		@apply bg-graphite bg-opacity-50 text-white;
	}
</style>
