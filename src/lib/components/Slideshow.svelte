<script lang="ts">
  import { fade } from "svelte/transition";
  import { onMount } from "svelte";

  import type { Picture } from "vite-imagetools";

  interface Props {
    images: Picture[];
  }

  const { images }: Props = $props();

  let currentIndex = $state(0);

  onMount(() => {
    const interval = setInterval(() => {
      currentIndex = (currentIndex + 1) % images.length;
    }, 6000);

    return () => clearInterval(interval);
  });
</script>

<div class="slideshow" aria-hidden="true">
  {#key currentIndex}
    <div
      class="background-layer"
      in:fade={{ duration: 1500 }}
      out:fade={{ duration: 1500 }}
    >
      <enhanced:img src={images[currentIndex]} alt="background" />
    </div>
  {/key}
</div>

<style>
  .slideshow {
    position: fixed;
    inset: 0;
    z-index: -2;
    overflow: hidden;
    background: #000;
  }

  .background-layer {
    display: block;
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
  }

  .background-layer :global(img) {
    display: block;
    width: 100%;
    height: 100%;
    object-fit: cover;
    object-position: center;
  }
</style>
