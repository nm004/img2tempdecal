import { resolve } from 'path'
import { defineConfig } from 'vite'
import { VitePWA } from 'vite-plugin-pwa'

export default defineConfig({
	root: resolve(__dirname, 'pkg'),
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
				main: resolve(__dirname, 'pkg/index.html'),
				'terms-of-use': resolve(__dirname, 'pkg/terms-of-use/index.html'),
				'privacy-policy': resolve(__dirname, 'pkg/privacy-policy/index.html'),
			}
		}
	}
})
