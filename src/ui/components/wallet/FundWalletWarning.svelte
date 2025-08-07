<!-- FundWalletWarning.svelte -->
<script>
  import Icon from "../basic_elems/Icon.svelte";
  import Button from "../basic_elems/Button.svelte";

  export let onOkClick = (event) => {};
  export let onFundWalletClick = (event) => {};

  function handleOkClick(event) {
    onOkClick(event);
  }

  function handleFundWalletClick(event) {
    onFundWalletClick(event);
  }
</script>

{#snippet WarningMessage()}
  <p class="text-white text-xl mb-14">
    Please ensure that your wallet has sufficient funds!
  </p>
{/snippet}

{#snippet ActionButtons()}
  <div class="flex gap-4 mt-6 absolute bottom-0 right-0 m-6">
    <Button
      label="Ok"
      variant="dark"
      width="w-16"
      height="h-14"
      on:click={handleOkClick}
    />
    <Button
      label="Fund wallet"
      variant="gold"
      width="w-32"
      height="h-14"
      on:click={handleFundWalletClick}
    />
  </div>
{/snippet}

<div
  class="bg-background-primary font-sans flex flex-col items-center justify-center w-[672px] h-[420px] p-6 text-center rounded-xl border border-white/25 relative"
>
  <div class="absolute top-0 left-0 m-6">
    <Icon
      name="poro_brand_white"
      class="cursor-default"
      size="2.5rem"
      scale="1"
      viewSize={{ width: 32, height: 32 }}
    />
  </div>

  <Icon
    name="piggy_bank"
    class="cursor-default mb-11"
    size="7rem"
    scale="1"
    viewSize={{ width: 112, height: 112 }}
  />
  {@render WarningMessage()}
  {@render ActionButtons()}
</div>
