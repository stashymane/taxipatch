<script lang="ts">
    import Slideshow from "$lib/components/Slideshow.svelte";
    import Button from "$lib/components/Button.svelte";
    import Icon from "$lib/components/Icon.svelte";

    let {data} = $props();
</script>

<Slideshow/>

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
                <a href="https://github.com/stashymane/taxipatch" class="logo">taxipatch</a>
            </header>

            <footer>
                <div class="start">
                    <Button href="https://github.com/stashymane/taxipatch" size="lg" label="GitHub">
                        <Icon name="logo-github"/>
                    </Button>
                    <a class="link" href="https://github.com/stashymane/taxipatch#taxipatch" aria-label="About">
                        <Icon name="question-mark-24dp-w300"/>
                        About
                    </a>
                </div>

                <div class="end">
                    <Button href="https://github.com/stashymane/taxipatch/releases/latest" variant="primary" size="lg">
                        <Icon name="download-24dp-w300"/>
                        <span>Download</span>
                    </Button>
                </div>
            </footer>
        </div>
    </div>
</div>
