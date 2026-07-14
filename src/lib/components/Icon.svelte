<script lang="ts">
  import logoGithub from "../../assets/icons/logo-github.svg?raw";
  import download from "../../assets/icons/download-24dp-w300.svg?raw";
  import questionMark from "../../assets/icons/question-mark-24dp-w300.svg?raw";
  import help from "../../assets/icons/help-24dp-w300.svg?raw";

  interface Props {
    name:
      | "logo-github"
      | "download-24dp-w300"
      | "question-mark-24dp-w300"
      | "help-24dp-w300";
    style?: string;
  }

  let { name, style }: Props = $props();

  const icons: Record<string, string> = {
    "logo-github": logoGithub,
    "download-24dp-w300": download,
    "question-mark-24dp-w300": questionMark,
    "help-24dp-w300": help,
  };
</script>

<span class="icon" {style}>{@html icons[name]}</span>

<style>
  .icon :global(svg) {
    width: 1lh;
    height: 1lh;
    display: block;
  }
</style>
