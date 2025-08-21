<script>
  import { page } from "$app/stores"; // For URL params
  import Home from "./home/+page.svelte"; // Import your views
  import Wallet from "./wallet/components/+page.svelte"; // Adjust path as needed

  // Define available views
  const views = {
    home: { component: Home, name: "Home Page" },
    wallet: { component: Wallet, name: "Wallet Components" },
    // Add more as needed, e.g., buttons: { component: Buttons, name: 'Buttons' }
  };

  // Reactive state for selected view and props (use URL params for persistence)
  let selectedView = $page.url.searchParams.get("view") || "home";
  let props = {}; // Dynamic props object, e.g., { isLoading: false, userData: { name: 'Test' } }

  // Example variants/states for quick switching (like Storybook stories)
  const variants = {
    default: { isLoading: false, error: null },
    loading: { isLoading: true },
    error: { isLoading: false, error: "Failed to load" },
    // Customize per view if needed
  };
  let selectedVariant = "default";

  // Update props when variant changes
  $: props = variants[selectedVariant] || {};

  // Function to update URL params (for shareable states)
  function updateParams(key, value) {
    const params = new URLSearchParams($page.url.searchParams);
    params.set(key, value);
    history.replaceState({}, "", `${$page.url.pathname}?${params}`);
  }
</script>

<div
  class="grid grid-cols-[240px_1fr_240px] gap-6 p-6 bg-[#282a36] text-[#f8f8f2] min-h-screen"
>
  <!-- Navigation Sidebar -->
  <aside class="bg-[#44475a] shadow-md rounded-lg p-6 flex flex-col">
    <h2 class="text-lg font-semibold text-[#f8f8f2] mb-4">Views</h2>
    <ul class="space-y-1">
      {#each Object.keys(views) as key}
        <li>
          <button
            class="w-full text-left px-4 py-2 rounded-md transition-colors {selectedView ===
            key
              ? 'bg-[#bd93f9]/20 text-[#bd93f9] font-medium'
              : 'text-[#f8f8f2] hover:bg-[#6272a4]'}"
            on:click={() => {
              selectedView = key;
              updateParams("view", key);
            }}
          >
            {views[key].name}
          </button>
        </li>
      {/each}
    </ul>
  </aside>

  <!-- Main Preview Area -->
  <main class="bg-[#44475a] shadow-md rounded-lg p-6 overflow-auto">
    {#if views[selectedView]}
      <svelte:component this={views[selectedView].component} {...props} />
    {/if}
  </main>

  <!-- Controls Panel -->
  <aside class="bg-[#44475a] shadow-md rounded-lg p-6 flex flex-col">
    <h2 class="text-lg font-semibold text-[#f8f8f2] mb-4">Controls</h2>
    <div class="space-y-4">
      <!-- Variant Selector -->
      <label class="block">
        <span class="text-sm font-medium text-[#f8f8f2]">State/Variant:</span>
        <select
          bind:value={selectedVariant}
          on:change={() => updateParams("variant", selectedVariant)}
          class="mt-1 block w-full px-3 py-2 bg-[#282a36] border border-[#6272a4] rounded-md shadow-sm focus:outline-none focus:ring-[#bd93f9] focus:border-[#bd93f9] sm:text-sm text-[#f8f8f2]"
        >
          {#each Object.keys(variants) as v}
            <option value={v}>{v}</option>
          {/each}
        </select>
      </label>

      <!-- Dynamic Prop Controls -->
      <label class="flex items-center space-x-3">
        <span class="text-sm font-medium text-[#f8f8f2]">isLoading:</span>
        <input
          type="checkbox"
          bind:checked={props.isLoading}
          on:change={() => updateParams("isLoading", props.isLoading)}
          class="h-4 w-4 text-[#bd93f9] border-[#6272a4] rounded focus:ring-[#bd93f9] bg-[#282a36]"
        />
      </label>

      <label class="block">
        <span class="text-sm font-medium text-[#f8f8f2]"
          >Custom Prop (e.g., userName):</span
        >
        <input
          type="text"
          bind:value={props.userName}
          on:input={() => updateParams("userName", props.userName)}
          class="mt-1 block w-full px-3 py-2 bg-[#282a36] border border-[#6272a4] rounded-md shadow-sm focus:outline-none focus:ring-[#bd93f9] focus:border-[#bd93f9] sm:text-sm text-[#f8f8f2]"
        />
      </label>
      <!-- For complex props, use JSON textarea and parse it -->
    </div>
  </aside>
</div>
