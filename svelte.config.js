import adapter from "@sveltejs/adapter-static";

/** @type {import('@sveltejs/kit').Config} */
export default {
    kit: {
        adapter: adapter({
            pages: "dist",
            assets: "dist",
            fallback: undefined,
            precompress: false,
            strict: true,
        }),
        alias: {
            "@/*": "./src/*",
            "$assets": "./src/assets",
        },
    },
};
