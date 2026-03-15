<script lang="ts">
  export let onImprove: () => void;
</script>

<div
  class="fixed bg-[#21222c] border border-[#bd93f9] rounded-xl shadow-2xl p-2 flex gap-2 z-50"
>
  <button
    on:click={onImprove}
    class="px-5 py-2 hover:bg-[#bd93f9] hover:text-black rounded-lg text-sm font-medium"
  >
    Improve prompt with AI
  </button>
</div>
