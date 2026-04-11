<script lang="ts">
  import PoroIcon from "./PoroIcon.svelte";

  interface Props {
    iconSrc: string;
    ariaLabel: string;
    title?: string;
    onclick?: (event: MouseEvent) => void;
    disabled?: boolean;
    iconSize?: string;
    buttonSize?: string;
    variant?: "ghost" | "outline" | "gold" | "bare";
    class?: string;
    iconClass?: string;
    iconStyle?: string;
    pulse?: boolean;
  }

  let {
    iconSrc,
    ariaLabel,
    title = undefined,
    onclick = () => {},
    disabled = false,
    iconSize = "1rem",
    buttonSize = "2.25rem",
    variant = "ghost",
    class: customClass = "",
    iconClass = "",
    iconStyle = "",
    pulse = false,
  }: Props = $props();

  const variants = {
    ghost:
      "ui-icon-button rounded-[10px] border border-white/10 text-fog/44 hover:border-white/14 hover:text-soft-ivory",
    outline:
      "ui-icon-button rounded-[10px] border border-white/12 text-fog/54 hover:border-white/18 hover:text-soft-ivory",
    gold:
      "ui-icon-button rounded-[16px] border-0 text-[#111113] shadow-[0_10px_30px_rgba(213,161,42,0.18)]",
    bare: "ui-icon-button ui-icon-button-bare rounded-[10px] border-0 text-fog/48 hover:text-soft-ivory",
  };

  const variantStyles = {
    ghost: "--icon-button-bg: transparent; --icon-button-hover-bg: rgb(255 255 255 / 0.03);",
    outline:
      "--icon-button-bg: transparent; --icon-button-hover-bg: rgb(255 255 255 / 0.03);",
    gold:
      "--icon-button-bg: var(--color-accent-gold); --icon-button-hover-bg: var(--color-warning-amber);",
    bare: "--icon-button-bg: transparent; --icon-button-hover-bg: transparent;",
  };
</script>

<button
  type="button"
  class={`${variants[variant]} ${customClass}`}
  style={`--icon-size: ${iconSize}; --icon-button-size: ${buttonSize}; ${variantStyles[variant]}`}
  {disabled}
  aria-label={ariaLabel}
  title={title}
  {onclick}
>
  <PoroIcon
    src={iconSrc}
    size={iconSize}
    class={`${iconClass} ${pulse ? "animate-pulse" : ""}`.trim()}
    inlineStyle={iconStyle}
  />
</button>
