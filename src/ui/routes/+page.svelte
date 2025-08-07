<script>
  import { authStore, login, logout } from "$lib/auth";
  import { getActor } from "$lib/canister"; // Import from your canister lib

  // Reactive logging of auth store changes
  $: console.log("Auth store updated:", $authStore);

  // Destructure store values reactively
  $: ({ isAuthenticated, principal } = $authStore);

  let callerPrincipal = null;
  let error = null;

  // Function to test calling the Motoko getCaller method using the lib
  async function testGetCaller() {
    try {
      error = null;
      const actor = getActor("user"); // From canister.js; assumes it returns the actor with agent/canisterId handled
      console.log("actor", actor);
      callerPrincipal = await actor.getCaller();
      console.log("Caller Principal from canister:", callerPrincipal.toText());
    } catch (err) {
      error = err.message;
      console.error("Error calling getCaller:", err);
    }
  }
</script>

<main>
  {#if isAuthenticated}
    <p>Welcome, {principal}!</p>
    <button on:click={testGetCaller}>Test Caller Principal</button>
    {#if callerPrincipal}
      <p>Caller Principal from Canister: {callerPrincipal.toText()}</p>
      <p>
        Matches Frontend Principal: {callerPrincipal.toText() === principal
          ? "Yes"
          : "No"}
      </p>
    {/if}
    {#if error}
      <p style="color: red;">Error: {error}</p>
    {/if}
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
    background-color: #333;
  }

  p {
    font-size: 1.2em;
    margin-bottom: 1em;
    color: #fafafa;
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
    margin-bottom: 1em;
  }

  button:hover {
    background-color: #0056b3;
  }
</style>
