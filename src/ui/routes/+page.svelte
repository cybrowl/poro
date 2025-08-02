<script>
	let name = 'World';
</script>

<main
	class="flex items-center justify-center h-screen bg-gradient-to-r from-blue-500 to-purple-600"
>
	<div class="text-center">
		<h1 class="text-5xl font-bold text-white mb-4">Hello, {name}!</h1>
		<p class="text-xl text-gray-200">Welcome to Svelte 5 with Tailwind CSS 4.</p>
		<input
			type="text"
			bind:value={name}
			class="mt-4 px-4 py-2 rounded-lg bg-white text-black focus:outline-none focus:ring-2 focus:ring-purple-300"
			placeholder="Enter your name"
		/>
	</div>
</main>
