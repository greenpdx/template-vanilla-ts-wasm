// vite.config.ts
import { defineConfig } from 'vite'
import wasm from "vite-plugin-wasm";
export default defineConfig({
    plugins: [
        wasm()
    ],
    server: {
        //host: '0.0.0.0'
    }
})
