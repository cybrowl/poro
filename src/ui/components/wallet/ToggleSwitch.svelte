<!-- ToggleSwitch.svelte -->
<script>
	let { mode = 'deposit' } = $props();

	function handleToggle(newMode) {
		mode = newMode;
	}

	const commonClasses =
		'font-sans text-xl px-12 py-3 rounded-full transition-all duration-300 ease-in-out focus:outline-none bg-transparent cursor-pointer relative z-10';

	const depositClasses = $derived(
		`${commonClasses} rounded-l-full -mr-[4%] ${mode === 'cashout' ? 'bg-graphite text-white shadow-lg transform translate-x-0 z-20' : 'bg-white text-carbon-black z-10 hover:bg-graphite/50 hover:text-white'}`
	);

	const cashOutClasses = $derived(
		`${commonClasses} rounded-r-full ${mode === 'deposit' ? 'bg-graphite text-white shadow-lg transform translate-x-0 z-20' : 'bg-white text-carbon-black z-10 hover:bg-graphite/50 hover:text-white'}`
	);
</script>

<div class="inline-flex">
	<button class={depositClasses} onclick={() => handleToggle('deposit')}> Deposit </button>
	<button class={cashOutClasses} onclick={() => handleToggle('cashout')}> Cash Out </button>
</div>
