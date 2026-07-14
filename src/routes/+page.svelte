<script lang="ts">
  import Slideshow from "$lib/components/Slideshow.svelte";
  import Button from "$lib/components/Button.svelte";
  import Icon from "$lib/components/Icon.svelte";
  import BackgroundSlideshow from "$lib/components/BackgroundSlideshow.svelte";

  let { data } = $props();
</script>

<BackgroundSlideshow />

<div class="container">
  <div class="content-border">
    <div class="content">
      <div class="details">
        <div class="detail">
          <span>for Crazy Taxi 3</span>
        </div>
        <span class="separator"></span>
        <div class="detail">
          <span>version</span>
          <span class="bold">{data.patchVersion}</span>
        </div>
      </div>

      <header>
        <a href="https://github.com/stashymane/taxipatch" class="logo">
          taxipatch
        </a>
      </header>

      <footer>
        <div class="start">
          <Button
            href="https://github.com/stashymane/taxipatch"
            size="lg"
            label="GitHub"
          >
            <Icon name="logo-github" />
          </Button>
          <Button
            href="https://github.com/stashymane/taxipatch"
            size="lg"
            label="About"
          >
            <Icon name="help-24dp-w300" />
            <span class="label">About</span>
          </Button>
        </div>

        <div class="end">
          <Button
            href="https://github.com/stashymane/taxipatch/releases/latest"
            variant="primary"
            size="lg"
            bold
          >
            <Icon name="download-24dp-w300" />
            <span class="label">Download</span>
          </Button>
        </div>
      </footer>
    </div>
  </div>
</div>

<style>
  @reference "tailwindcss";

  .container {
    @apply min-h-screen min-w-screen flex flex-col gap-4 items-center justify-center;
    background-color: rgba(0 0 0 / 0.5);
  }

  .content-border {
    @apply inline-flex;

    --border-radius: 1em;
    border-radius: var(--border-radius);
    --padding: 0.1em;
    padding: var(--padding);

    background-color: var(--container-color);
    background-image: linear-gradient(
      rgb(255 255 255 / 0.2),
      rgb(150 150 150 / 0.1)
    );

    box-shadow: 0 0 5em rgba(0 0 0 / 0.5);
  }

  .content {
    @apply w-200;
    border-radius: calc(var(--border-radius) - var(--padding));
    background-color: var(--container-color);
    background-image: linear-gradient(
      20deg,
      transparent,
      rgb(255 255 255 / 0.02) 20%,
      rgb(255 255 255 / 0.05) 80%,
      rgb(255 255 255 / 0.02)
    );
  }

  .details {
    @apply flex flex-row items-center justify-end text-[0.9em];
    color: var(--text-muted-color);

    .detail {
      @apply py-[0.75em] px-[1em];

      .bold {
        @apply font-bold;
        color: var(--text-color);
      }
    }

    .separator {
      @apply h-[0.1em] w-[0.5em];
      background-color: color-mix(
        in oklch,
        var(--text-muted-color),
        transparent 50%
      );
    }
  }

  header {
    @apply flex flex-row items-center justify-center;
    @apply font-light text-[3.5em];
    font-family: "Bitcount", sans-serif;
    font-weight: 300;

    background-color: var(--background-color);
    background-image: radial-gradient(
      circle at center in oklch,
      color-mix(in oklch, var(--primary-color), transparent 85%),
      color-mix(in oklch, var(--primary-color), transparent 95%)
    );

    border-width: 1px 0;
    border-style: solid;
    border-color: rgba(255 255 255 / 0.1);

    box-shadow: 0 0 0.5em rgba(0 0 0 / 0.5) inset;

    a {
      @apply inline-flex items-center justify-center w-full p-[1rem];
      @apply transition-[text-shadow] duration-200 ease-out;

      color: color-mix(in oklch, var(--primary-color), white 90%);

      --text-shadow-color: color-mix(
        in oklch,
        var(--primary-color),
        transparent 70%
      );
      text-shadow:
        0 0 0.2em var(--text-shadow-color),
        0 0 0.4em var(--text-shadow-color),
        0 0 0.6em var(--text-shadow-color),
        0 0 0.8em var(--text-shadow-color);

      &:hover {
        --text-shadow-color: color-mix(
          in oklch,
          var(--primary-color),
          transparent 60%
        );
      }
    }
  }

  footer {
    @apply flex flex-row items-center justify-between gap-[1em] p-[1em];

    .start,
    .end {
      @apply flex flex-row items-center gap-[1em];
    }
  }

  @media (max-width: 840px) {
    .content {
      @apply w-150;
    }
  }

  @media (max-width: 640px) {
    .content {
      @apply w-full;
    }

    .content-border {
      @apply w-full mx-[1em];
    }

    footer {
      @apply flex-col;
    }
  }
</style>
