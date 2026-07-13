<script lang="ts">
  import { fade } from "svelte/transition";
  import { onMount } from "svelte";

  interface Props {
    images: string[];
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
      style:background-image="url('{images[currentIndex]}')"
      in:fade={{ duration: 1500 }}
      out:fade={{ duration: 1500 }}
    ></div>
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
    position: absolute;
    inset: 0;
    background-position: center;
    background-size: cover;
    background-repeat: no-repeat;
  }
</style>
