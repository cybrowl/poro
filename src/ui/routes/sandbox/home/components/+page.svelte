<script>
  import TopToolbar from "$components/home/TopToolbar.svelte";
</script>

<main
  class="bg-deep-charcoal flex items-center justify-center bg-gradient-to-r from-deep-charcoal to-dark-slate overflow-y-auto"
>
  <div class="flex flex-col items-center gap-8 text-white max-w-6xl p-8 w-full">
    <h1 class="text-4xl font-bold mb-6 text-center">
      Home Components Template
    </h1>
    <div class="grid grid-cols-1 gap-8 w-full">
      <!-- TopToolbar section -->
      <section
        class="flex flex-col gap-4 bg-dark-slate/50 p-6 rounded-lg border border-silver-mist/30 text-left"
      >
        <h2 class="text-2xl font-semibold">TopToolbar</h2>
        <p class="text-sm text-silver-mist mb-2">
          A toolbar component for the top of the home page, including navigation
          icons and actions.
        </p>
        <TopToolbar />
      </section>
    </div>
  </div>
</main>
