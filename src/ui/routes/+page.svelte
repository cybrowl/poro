<script>
	import { authStore, login, logout } from '$lib/auth';

	// Reactive logging of auth store changes
	$: console.log('Auth store updated:', $authStore);

	// Destructure store values reactively
	$: ({ isAuthenticated, principal } = $authStore);
</script>

<main>
	{#if isAuthenticated}
		<p>Welcome, {principal}!</p>
		<button on:click={logout}>Logout</button>
	{:else}
		<button on:click={login}>Login with Internet Identity</button>
	{/if}
</main>

<style>
	/* Basic styling for improved UX */
	main {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 100vh;
		font-family: sans-serif;
	}

	p {
		font-size: 1.2em;
		margin-bottom: 1em;
	}

	button {
		padding: 0.5em 1em;
		font-size: 1em;
		background-color: #007bff;
		color: white;
		border: none;
		border-radius: 4px;
		cursor: pointer;
		transition: background-color 0.3s;
	}

	button:hover {
		background-color: #0056b3;
	}
</style>
