<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    href?: string;
    variant?: "default" | "primary";
    size?: "md" | "lg";
    label?: string;
    children?: Snippet;
  }

  let {
    href,
    variant = "default",
    size = "md",
    label,
    children,
  }: Props = $props();
</script>

{#if href}
  <a
    {href}
    aria-label={label}
    class="button"
    class:primary={variant === "primary"}
    class:lg={size === "lg"}
  >
    {@render children?.()}
  </a>
{:else}
  <button
    aria-label={label}
    class="button"
    class:primary={variant === "primary"}
    class:lg={size === "lg"}
  >
    {@render children?.()}
  </button>
{/if}

<style>
  @reference "../../app.css";

  .button {
    @apply inline-flex flex-row items-center justify-center;
    @apply px-[0.75em] gap-[0.75em];
    @apply font-bold rounded-[0.75em];

    line-height: 2.25em;
    height: 1lh;
    min-width: 1lh;

    --button-color: color-mix(in oklch, var(--container-color), white 80%);
    --button-text-color: color-mix(
      in oklch,
      var(--button-color),
      var(--text-color) 70%
    );
    --button-background-color: color-mix(
      in oklch,
      var(--button-color),
      var(--container-color) 80%
    );
    --button-border-color: color-mix(
      in oklch,
      var(--button-color),
      transparent 80%
    );
    --button-shadow-color: color-mix(
      in oklch,
      var(--button-background-color),
      var(--container-color) 25%
    );

    color: var(--button-text-color);
    background-color: var(--button-background-color);
    border: 0.1em solid var(--button-border-color);

    --button-lift: 0.2em;
    --button-lift-offset: 0em;
    --button-total-offset: calc(var(--button-lift) + var(--button-lift-offset));
    box-shadow: 0 var(--button-total-offset) 0 var(--button-shadow-color);

    --animation-duration: 0.2s;
    transition:
      box-shadow var(--animation-duration) var(--animation-fn),
      transform var(--animation-duration) var(--animation-fn),
      background-color var(--animation-duration) var(--animation-fn),
      border var(--animation-duration) var(--animation-fn);

    transform: translateY(calc(-1 * var(--button-lift-offset)));

    :global(.icon) {
      width: 0.7lh;
      height: 0.7lh;
      margin: 0 -0.25em;
    }

    :global(.icon:only-child) {
      margin: 0 -0.5em;
    }
  }

  .button:hover {
    --button-text-color: color-mix(
      in oklch,
      var(--button-color),
      var(--text-color) 80%
    );
    --button-background-color: color-mix(
      in oklch,
      var(--button-color),
      var(--container-color) 75%
    );
    --button-border-color: color-mix(
      in oklch,
      var(--button-color),
      transparent 75%
    );
    --button-lift-offset: 0.1em;
  }

  .button:active {
    --animation-duration: 0.05s;
    --button-lift-offset: calc(-1 * var(--button-lift));
  }

  .button.primary {
    --button-color: color-mix(in oklch, var(--primary-color), white 20%);
  }

  .button.lg {
    font-size: 1.1em;
  }
</style>
