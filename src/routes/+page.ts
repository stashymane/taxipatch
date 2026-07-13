export const prerender = true;

export async function load({ fetch }) {
  let patchVersion = "???";

  try {
    const response = await fetch(
      "https://api.github.com/repos/stashymane/taxipatch/releases/latest",
      { cache: "no-store" },
    );

    if (response.ok) {
      const metadata = await response.json();
      if (metadata.name) {
        patchVersion = metadata.name;
      }
    }
  } catch (e) {
    console.error("Failed to retrieve patch version", e);
  }

  return { patchVersion };
}
