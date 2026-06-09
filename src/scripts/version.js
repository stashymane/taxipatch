const versionElement = document.querySelector("#patch-version");

async function updatePatchVersion() {
    if (!versionElement) {
        return;
    }

    try {
        const response = await fetch("/metadata.json", {
            cache: "no-store",
        });

        if (!response.ok) {
            throw new Error(`Failed to load metadata.json: ${response.status}`);
        }

        const metadata = await response.json();

        if (metadata.latest_version) {
            versionElement.textContent = metadata.latest_version;
        }
    } catch (error) {
        console.error(error);
        versionElement.textContent = "unknown";
    }
}

updatePatchVersion();
