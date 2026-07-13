<script lang="ts">
    import logoGithub from "../../assets/icons/logo-github.svg?raw";
    import download from "../../assets/icons/download-24dp-w300.svg?raw";
    import questionMark from "../../assets/icons/question-mark-24dp-w300.svg?raw";

    interface Props {
        name: "logo-github" | "download-24dp-w300" | "question-mark-24dp-w300";
    }

    let {name}: Props = $props();

    const icons: Record<string, string> = {
        "logo-github": logoGithub,
        "download-24dp-w300": download,
        "question-mark-24dp-w300": questionMark,
    };
</script>

<span class="icon">{@html icons[name]}</span>

<style>
    .icon :global(svg) {
        width: 100%;
        height: 100%;
        display: block;
    }
</style>
