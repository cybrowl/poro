<script lang="ts">
  import type { ActivityItem } from "$lib/mockDesktopData";

  interface Props {
    items: ActivityItem[];
  }

  let { items }: Props = $props();

  function dotClass(status: ActivityItem["status"]) {
    if (status === "active") return "bg-misty-green shadow-[0_0_18px_rgba(134,249,119,0.35)]";
    if (status === "complete") return "bg-marigold";
    return "bg-fog/35";
  }
</script>

<section
  class="rounded-[30px] border border-white/8 bg-carbon-black/80 p-4 shadow-2xl backdrop-blur-md sm:p-5"
>
  <div class="flex items-center justify-between gap-3">
    <div>
      <div class="text-[0.68rem] uppercase tracking-[0.35em] text-fog/55">
        Tool Activity
      </div>
      <div class="mt-2 text-xl">Timeline</div>
    </div>
    <div class="text-xs uppercase tracking-[0.22em] text-fog/45">
      local runtime
    </div>
  </div>

  <div class="mt-5 space-y-4">
    {#if items.length}
      {#each items as item}
        <div class="rounded-[22px] border border-white/6 bg-white/3 p-4">
          <div class="flex items-start gap-4">
            <div class={`mt-1 h-3 w-3 rounded-full ${dotClass(item.status)}`}></div>
            <div class="min-w-0 flex-1">
              <div class="flex flex-wrap items-center justify-between gap-3">
                <div class="text-sm uppercase tracking-[0.22em] text-soft-ivory">
                  {item.label}
                </div>
                <div class="text-xs uppercase tracking-[0.22em] text-fog/45">
                  {item.timestamp}
                </div>
              </div>
              <p class="mt-3 text-sm leading-6 text-fog/72">{item.summary}</p>
            </div>
          </div>
        </div>
      {/each}
    {:else}
      <div class="rounded-[22px] border border-dashed border-white/10 bg-white/2 p-4 text-sm leading-7 text-fog/65">
        Runtime activity will appear here once the local `claw` session starts streaming output.
      </div>
    {/if}
  </div>
</section>
