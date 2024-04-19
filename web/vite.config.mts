// This code is in the public domain.

import { VitePWA } from 'vite-plugin-pwa'

export default {
	server: {
		watch: {
			depth: 2
		}
	},
	build: {
		rollupOptions: {
			external: [
				'workbox-precaching'
			],
			input: [
				'index.html',
				'notice/index.html',
				'privacy-policy/index.html',
			]
		}
	},
	plugins: [
		VitePWA({
			strategies: 'injectManifest',
			srcDir: 'src',
			filename: 'sw.ts',
			manifest: false,
			injectManifest: {
				globPatterns: [
					'assets/**/*.{js,wasm,css}',
					'index.html'
				]
			},
		})
	]
};
