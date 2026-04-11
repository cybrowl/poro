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
    dark: "border border-white/8 bg-white/[0.035] text-fog/84 hover:border-white/12 hover:bg-white/[0.055] hover:text-soft-ivory",
    gold: "border border-accent-gold/26 bg-accent-gold text-[#1f1807] hover:bg-warning-amber",
    ghost:
      "border border-transparent bg-transparent text-fog/62 hover:bg-white/[0.04] hover:text-soft-ivory",
    outline:
      "border border-white/10 bg-transparent text-fog/70 hover:border-white/16 hover:bg-white/[0.03] hover:text-soft-ivory",
  };

  let buttonClasses = $derived(
    `inline-flex items-center justify-center rounded-lg px-3 py-2 text-[14px] font-normal tracking-[0.005em] transition-colors duration-150 focus:outline-none ${variants[variant] || variants.dark} ${width} ${height} ${disabled ? "cursor-not-allowed opacity-50" : "cursor-pointer"} ${customClass}`
  );
</script>

<button class={buttonClasses} {onclick} {disabled} type="button">
  {#if children}
    {@render children()}
  {:else}
    {label}
  {/if}
</button>
