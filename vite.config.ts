import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";
import tailwindcss from "@tailwindcss/vite";
import { enhancedImages } from "@sveltejs/enhanced-img";

// https://vite.dev/config/
export default defineConfig({
  plugins: [tailwindcss(), enhancedImages(), sveltekit()],
});
