<script lang="ts">
    import Slideshow from "./lib/components/Slideshow.svelte";

    let patchVersion = $state("???");

    async function fetchPatchVersion() {
        const response = await fetch("https://api.github.com/repos/stashymane/taxipatch/releases/latest", {cache: "no-store"});

        if (!response.ok) {
            throw new Error(`Failed to load GitHub release metadata: ${response.status}`);
        }

        const metadata = await response.json();

        if (metadata.name) {
            patchVersion = metadata.name;
        }
    }

    fetchPatchVersion();
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
                    <span class="bold">{patchVersion}</span>
                </div>
            </div>

            <header>
                <a href="https://github.com/stashymane/taxipatch" class="logo">taxipatch</a>
            </header>

            <footer>
                <div class="start">
                    <a class="button lg" href="https://github.com/stashymane/taxipatch" aria-label="GitHub">
                        <svg class="icon" aria-hidden="true">
                            <use xlink:href="#icon-logo-github"></use>
                        </svg>
                    </a>
                    <a class="link" href="https://github.com/stashymane/taxipatch#taxipatch" aria-label="About">
                        <svg class="icon" aria-hidden="true">
                            <use xlink:href="#icon-question-mark-24dp-w300"></use>
                        </svg>
                        About
                    </a>
                </div>

                <div class="end">
                    <a class="button primary lg" href="https://github.com/stashymane/taxipatch/releases/latest">
                        <svg class="icon" aria-hidden="true">
                            <use xlink:href="#icon-download-24dp-w300"></use>
                        </svg>
                        <span>Download</span>
                    </a>
                </div>
            </footer>
        </div>
    </div>
</div>
