import { resolve } from 'path'
import { defineConfig } from 'vite'

export default defineConfig({
  build: {
    rollupOptions: {
      input: {
	main: resolve(__dirname, 'index.html'),
	'terms-of-use': resolve(__dirname, 'terms-of-use.html'),
	'privacy-policy': resolve(__dirname, 'privacy-policy.html'),
      }
    }
  }
})
