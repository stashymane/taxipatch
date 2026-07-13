<script lang="ts">
    import Slideshow from "$lib/components/Slideshow.svelte";
    import Button from "$lib/components/Button.svelte";

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
                        <svg class="icon" aria-hidden="true">
                            <use xlink:href="#icon-logo-github"></use>
                        </svg>
                    </Button>
                    <a class="link" href="https://github.com/stashymane/taxipatch#taxipatch" aria-label="About">
                        <svg class="icon" aria-hidden="true">
                            <use xlink:href="#icon-question-mark-24dp-w300"></use>
                        </svg>
                        About
                    </a>
                </div>

                <div class="end">
                    <Button href="https://github.com/stashymane/taxipatch/releases/latest" variant="primary" size="lg">
                        <svg class="icon" aria-hidden="true">
                            <use xlink:href="#icon-download-24dp-w300"></use>
                        </svg>
                        <span>Download</span>
                    </Button>
                </div>
            </footer>
        </div>
    </div>
</div>
