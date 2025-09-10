// vite.config.ts
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
    plugins: [sveltekit()],
    server: {
        proxy: {
            // Forward all requests starting with /api to live backend
            '/api': {
                target: 'http://91.99.122.106:8001',
                changeOrigin: true,
            }
        }
    }
});