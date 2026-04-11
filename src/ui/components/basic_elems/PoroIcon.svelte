<script lang="ts">
  interface Props {
    src: string;
    alt?: string;
    size?: string;
    class?: string;
    decorative?: boolean;
    fit?: "contain" | "cover";
    inlineStyle?: string;
    title?: string;
  }

  let {
    src,
    alt = "",
    size = "1rem",
    class: customClass = "",
    decorative = true,
    fit = "contain",
    inlineStyle = "",
    title = undefined,
  }: Props = $props();
</script>

<img
  src={src}
  alt={decorative ? "" : alt}
  aria-hidden={decorative}
  title={title}
  class={`ui-icon ${customClass}`}
  style={`--icon-size: ${size}; object-fit: ${fit}; ${inlineStyle}`.trim()}
/>
