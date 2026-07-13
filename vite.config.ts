import {defineConfig} from "vite";
import {sveltekit} from "@sveltejs/kit/vite";
import tailwindcss from "@tailwindcss/vite";
import {createSvgIconsPlugin} from "vite-plugin-svg-icons-ng";
import * as path from "node:path";

// https://vite.dev/config/
export default defineConfig({
    plugins: [
        tailwindcss(),
        sveltekit(),
        createSvgIconsPlugin({iconDirs: [path.resolve("src/assets/icons")]}),
    ],
});
