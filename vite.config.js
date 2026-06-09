import {createSvgIconsPlugin} from "vite-plugin-svg-icons-ng";

export default {
    plugins: [
        createSvgIconsPlugin({
            iconDirs: ['src/assets/icons']
        })
    ]
}
