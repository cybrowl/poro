<script>
  import { onMount } from "svelte";
  import { initAuth } from "$lib/auth";

  let { children } = $props();

  import "../app.css";

  onMount(async () => {
    await initAuth();
  });
</script>

{@render children()}
