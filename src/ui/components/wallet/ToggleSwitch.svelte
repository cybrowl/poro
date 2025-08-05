<!-- ToggleSwitch.svelte -->
<script>
	let { mode = 'deposit' } = $props();

	function handleToggle(newMode) {
		mode = newMode;
	}

	const commonClasses =
		'font-sans font-bold text-2xl px-10 py-3 rounded-full transition-all duration-300 ease-in-out focus:outline-none bg-transparent text-white cursor-pointer relative z-10';

	const depositClasses = $derived(
		`${commonClasses} rounded-l-full -mr-[10%] ${mode === 'deposit' ? 'bg-graphite text-white shadow-lg transform translate-x-0 z-10' : 'bg-white text-carbon-black z-20 hover:bg-graphite/50 hover:text-white'}`
	);

	const cashOutClasses = $derived(
		`${commonClasses} rounded-r-full ${mode === 'cashout' ? 'bg-graphite text-white shadow-lg transform translate-x-0 z-10' : 'bg-white text-carbon-black z-20 hover:bg-graphite/50 hover:text-white'}`
	);
</script>

<div class="inline-flex">
	<button class={depositClasses} onclick={() => handleToggle('deposit')}> Deposit </button>
	<button class={cashOutClasses} onclick={() => handleToggle('cashout')}> Cash Out </button>
</div>
