// vite.config.ts
import { defineConfig } from 'vite'
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

export default defineConfig({
    base: "./",
    plugins: [
        wasm(),
        topLevelAwait(),
    ],
    server: {
        host: '0.0.0.0'
    }
})
