
import { defineConfig } from 'vite';
import { resolve } from 'path';

export default defineConfig({
    clearScreen: false,
    server: {
        strictPort: true,
    },
    envPrefix: ['VITE_', 'TAURI_'],
    build: {
        target: ['es2021', 'chrome97', 'safari13'],
        minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
        sourcemap: !!process.env.TAURI_DEBUG,
        rollupOptions: {
            input: {
                main: resolve(__dirname, 'index.html'),
                metadata: resolve(__dirname, 'metadata.html'),
            },
        },
    },
});
