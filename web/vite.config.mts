/*
 * This file is a part of img2tempdecal by Nozomi Miyamori.
 * img2tempdecal is distributed under the MIT-0 license and the Public Domain.
 */

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
