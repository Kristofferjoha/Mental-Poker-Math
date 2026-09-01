// vite.config.ts
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

// No /api proxy: the browser calls the backend directly in dev as well as in
// production, so a CORS misconfiguration shows up locally instead of after a
// deploy. The backend's allow-list lives in CORS_ALLOWED_ORIGINS.
export default defineConfig({
	plugins: [sveltekit()]
});
