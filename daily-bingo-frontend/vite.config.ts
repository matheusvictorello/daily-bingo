import adapter from '@sveltejs/adapter-static';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [
		sveltekit({
			compilerOptions: {
				// Force runes mode for the project, except for libraries. Can be removed in svelte 6.
				runes: ({ filename }) =>
					filename.split(/[/\\]/).includes('node_modules') ? undefined : true
			},

			// SPA: served as static files by nginx, which proxies /api to the backend.
			adapter: adapter({ fallback: '200.html' })
		})
	],
	server: {
		// Backend has no CORS; proxy /api/* to it in dev.
		proxy: {
			'/api': { target: 'http://localhost:3000', rewrite: (p) => p.replace(/^\/api/, '') }
		}
	}
});
