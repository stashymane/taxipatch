const backgroundImages = [
    "/backgrounds/bg1.jpg",
    "/backgrounds/bg2.jpg",
    "/backgrounds/bg3.jpg",
    "/backgrounds/bg4.jpg",
    "/backgrounds/bg5.jpg",
    "/backgrounds/bg6.jpg",
    "/backgrounds/bg7.jpg",
    "/backgrounds/bg8.jpg",
    "/backgrounds/bg9.jpg",
    "/backgrounds/bg10.jpg",
    "/backgrounds/bg11.jpg",
    "/backgrounds/bg12.jpg",
    "/backgrounds/bg13.jpg",
];

const layerA = document.querySelector(".background-layer-a");
const layerB = document.querySelector(".background-layer-b");

let currentImageIndex = 0;
let showingLayerA = true;

layerA.style.backgroundImage = `url("${backgroundImages[0]}")`;
layerA.classList.add("is-visible");

function showNextBackground() {
    currentImageIndex = (currentImageIndex + 1) % backgroundImages.length;

    const visibleLayer = showingLayerA ? layerA : layerB;
    const hiddenLayer = showingLayerA ? layerB : layerA;

    hiddenLayer.style.backgroundImage = `url("${backgroundImages[currentImageIndex]}")`;

    hiddenLayer.classList.add("is-visible");
    visibleLayer.classList.remove("is-visible");

    showingLayerA = !showingLayerA;
}

setInterval(showNextBackground, 6000);
