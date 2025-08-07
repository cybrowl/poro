<!-- AddressToInput.svelte -->
<script>
  import Icon from "../basic_elems/Icon.svelte";

  let {
    value = "",
    placeholder = "",
    icon = "bank",
    width = "w-full",
    disabled = false,
    class: customClass = "",
    oninput = () => {},
  } = $props();

  function handleInput(event) {
    value = event.target.value;
    oninput(value);
  }
</script>

<div class="{width} {customClass}">
  <p class="text-white text-base mb-2">Address To Transfer</p>
  <div class="flex items-center rounded-lg border border-silver-mist px-6 py-4">
    <Icon
      name={icon}
      clickable={false}
      class="stroke_gray"
      size="2rem"
      viewSize={{ width: 32, height: 32 }}
    />

    <input
      class="flex-1 bg-transparent text-white placeholder-gray-500 outline-none border-none font-sans text-base pl-4"
      type="text"
      {placeholder}
      bind:value
      {disabled}
      oninput={handleInput}
    />
  </div>
</div>
