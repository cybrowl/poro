<script lang="ts">
  import Button from "$components/basic_elems/Button.svelte";

  // Demo click handler (so you can see it working)
  const handleClick = (label: string) => {
    console.log(`Clicked: ${label}`);
  };
</script>

<main class="bg-deep-charcoal min-h-screen py-12 px-6">
  <div class="max-w-6xl mx-auto">
    <!-- Header -->
    <div class="text-center mb-12">
      <h1 class="text-5xl font-bold text-white mb-3">Button Components</h1>
      <p class="text-silver-mist text-lg">
        Visual guide for all button variants and states
      </p>
    </div>

    <div class="max-w-md mx-auto">
      <div class="wallet-card p-8">
        <h2 class="text-2xl font-semibold mb-8 text-center">Button Variants</h2>

        <div class="flex flex-col gap-8">
          <!-- Dark -->
          <div>
            <p class="text-sm text-silver-mist mb-3">Dark Variant</p>
            <Button
              label="Dark Button"
              variant="dark"
              onclick={() => handleClick("Dark Button")}
            />
          </div>

          <!-- Gold -->
          <div>
            <p class="text-sm text-silver-mist mb-3">Gold Variant</p>
            <Button
              label="Gold Button"
              variant="gold"
              onclick={() => handleClick("Gold Button")}
            />
          </div>

          <!-- Full Width -->
          <div>
            <p class="text-sm text-silver-mist mb-3">Full Width</p>
            <Button
              label="Custom Width Button"
              variant="dark"
              width="w-full"
              onclick={() => handleClick("Full Width")}
            />
          </div>
        </div>
      </div>
    </div>

    <!-- Footer note -->
    <p class="text-center text-silver-mist/70 mt-16 text-sm">
      Click any button to see console feedback. This is a visual demo page.
    </p>
  </div>
</main>

<style lang="postcss">
  @reference "../../../app.css";

  .wallet-card {
    @apply bg-dark-slate/50 rounded-2xl border border-silver-mist/30 transition-all hover:border-silver-mist/50;
  }
</style>
