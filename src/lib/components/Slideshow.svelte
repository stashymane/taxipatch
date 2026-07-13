<script lang="ts">
  import { fade } from "svelte/transition";
  import { onMount } from "svelte";

  const images = [
    "backgrounds/bg1.jpg",
    "backgrounds/bg2.jpg",
    "backgrounds/bg3.jpg",
    "backgrounds/bg4.jpg",
    "backgrounds/bg5.jpg",
    "backgrounds/bg6.jpg",
    "backgrounds/bg7.jpg",
    "backgrounds/bg8.jpg",
    "backgrounds/bg9.jpg",
    "backgrounds/bg10.jpg",
    "backgrounds/bg11.jpg",
    "backgrounds/bg12.jpg",
    "backgrounds/bg13.jpg",
  ];

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
