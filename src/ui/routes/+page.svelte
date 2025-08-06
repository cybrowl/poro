<script>
	import { authStore, login, logout } from '$lib/auth';

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
