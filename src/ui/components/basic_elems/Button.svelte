<!-- Button.svelte -->
<script lang="ts">
  interface Props {
    label?: string;
    variant?: "dark" | "gold" | "ghost" | "outline";
    width?: string;
    height?: string;
    onclick?: (event: MouseEvent) => void;
    disabled?: boolean;
    class?: string;
    children?: import("svelte").Snippet;
  }

  let {
    label = "Button",
    variant = "dark",
    width = "w-auto",
    height = "h-14",
    onclick = () => {},
    disabled = false,
    class: customClass = "",
    children = undefined,
  }: Props = $props();

  const variants = {
    dark: "border border-white/8 bg-graphite/80 text-soft-ivory hover:border-white/12 hover:bg-midnight-slate/65",
    gold: "border border-signal-blue/28 bg-signal-blue/90 text-[#061321] shadow-[0_0_0_1px_rgba(78,161,255,0.12)] hover:bg-signal-blue/84",
    ghost:
      "border border-transparent bg-white/4 text-fog hover:bg-white/8 hover:text-soft-ivory",
    outline:
      "border border-white/12 bg-transparent text-fog hover:border-white/20 hover:bg-white/4 hover:text-soft-ivory",
  };

  let buttonClasses = $derived(
    `inline-flex items-center justify-center rounded-md px-3.5 py-2 text-[0.8rem] font-medium tracking-[0.01em] transition-colors duration-150 focus:outline-none ${variants[variant] || variants.dark} ${width} ${height} ${disabled ? "cursor-not-allowed opacity-50" : "cursor-pointer"} ${customClass}`
  );
</script>

<button class={buttonClasses} {onclick} {disabled} type="button">
  {#if children}
    {@render children()}
  {:else}
    {label}
  {/if}
</button>
