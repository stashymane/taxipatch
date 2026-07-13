<script lang="ts">
  import Slideshow from "$lib/components/Slideshow.svelte";
  import bg1 from "$assets/backgrounds/bg1.jpg?enhanced";
  import bg2 from "$assets/backgrounds/bg2.jpg?enhanced";
  import bg3 from "$assets/backgrounds/bg3.jpg?enhanced";
  import bg4 from "$assets/backgrounds/bg4.jpg?enhanced";
  import bg5 from "$assets/backgrounds/bg5.jpg?enhanced";
  import bg6 from "$assets/backgrounds/bg6.jpg?enhanced";
  import bg7 from "$assets/backgrounds/bg7.jpg?enhanced";
  import bg8 from "$assets/backgrounds/bg8.jpg?enhanced";
  import bg9 from "$assets/backgrounds/bg9.jpg?enhanced";
  import bg10 from "$assets/backgrounds/bg10.jpg?enhanced";
  import bg11 from "$assets/backgrounds/bg11.jpg?enhanced";
  import bg12 from "$assets/backgrounds/bg12.jpg?enhanced";
  import bg13 from "$assets/backgrounds/bg13.jpg?enhanced";

  const images = [
    bg1,
    bg2,
    bg3,
    bg4,
    bg5,
    bg6,
    bg7,
    bg8,
    bg9,
    bg10,
    bg11,
    bg12,
    bg13,
  ];
</script>

<Slideshow {images}></Slideshow>
