// This software is in the public domain.

import { resolve } from 'path'
import { defineConfig } from 'vite'
import { VitePWA } from 'vite-plugin-pwa'

export default defineConfig({
	plugins: [
		VitePWA({
			registerType: 'autoUpdate',
			manifest: false,
			workbox: {
				globPatterns: ['**/*.{js,css,html,wasm}']
			}
		})
	],
	build: {
		rollupOptions: {
			input: {
				main: resolve(__dirname, 'index.html'),
				notice: resolve(__dirname, 'notice/index.html'),
				privacy_policy: resolve(__dirname, 'privacy-policy/index.html'),
			}
		}
	}
})
