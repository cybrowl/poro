<!-- Banner.svelte -->
<script>
  let { message = "ICP will be automatically exchanged for ckUSDC." } =
    $props();
</script>

<div class="inline-block">
  <div
    class="inline-flex items-center rounded-lg overflow-hidden bg-carbon-black/50 border border-white/10 h-20"
  >
    <div class="w-4 h-full bg-marigold rounded-l-lg"></div>
    <span class="text-white text-base font-sans mx-6">{message}</span>
  </div>
</div>
