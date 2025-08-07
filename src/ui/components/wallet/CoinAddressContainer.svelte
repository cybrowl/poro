<!-- CoinAddressContainer.svelte -->
<script>
  import Icon from "../basic_elems/Icon.svelte";
  import Button from "../basic_elems/Button.svelte";

  let {
    coinType = "ICP",
    address = "87a4427b0ae47c3a92f0f2132a98f9a5d69d5ecefa97ad9ac3919766bbae85ac",
    onCopy = () => {},
  } = $props();

  const coinIconMap = {
    ICP: "internet_computer_protocol",
    BTC: "bitcoin",
    ETH: "ethereum",
  };

  let copied = $state(false);

  async function handleCopy() {
    try {
      await navigator.clipboard.writeText(address);
      copied = true;
      setTimeout(() => (copied = false), 2000);
      onCopy();
    } catch (error) {
      console.error("Copy failed:", error);
    }
  }
</script>

{#snippet AddressInfo()}
  <div class="flex flex-col justify-center min-w-0 flex-grow">
    <p class="text-lg text-white">{coinType} Address</p>
    <p
      class="font-mono text-sm text-silver-mist truncate hover:overflow-visible hover:whitespace-normal"
      title={address}
    >
      {address}
    </p>
  </div>
{/snippet}

<div class="flex items-center w-full h-14 gap-4">
  <div class="flex-shrink-0">
    <Icon
      name={coinIconMap[coinType] || "internet_computer_protocol"}
      class="cursor-default"
      size="3.125rem"
      viewSize={{ width: 50, height: 50 }}
    />
  </div>

  {@render AddressInfo()}

  <Button
    class="copy-button"
    label="Ok"
    variant="dark"
    width="w-20"
    height="h-14"
    onclick={handleCopy}
  >
    {#if copied}
      Copied!
    {:else}
      Copy
    {/if}
  </Button>
</div>
