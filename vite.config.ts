import {defineConfig} from "vite";
import {svelte} from "@sveltejs/vite-plugin-svelte";
import tailwindcss from "@tailwindcss/vite";
import {createSvgIconsPlugin} from "vite-plugin-svg-icons-ng";
import * as path from "node:path";

// https://vite.dev/config/
export default defineConfig({
    plugins: [
        tailwindcss(),
        svelte(),
        createSvgIconsPlugin({iconDirs: [path.resolve("src/assets/icons")]}),
    ],
    resolve: {
        alias: {
            "@": path.resolve("./src"),
        },
    },
});
