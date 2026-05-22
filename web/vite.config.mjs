/*
 * This file is a part of img2tempdecal by Nozomi Miyamori.
 * img2tempdecal is distributed under the MIT-0 license and the Public Domain.
 */
import { sveltekit } from "@sveltejs/kit/vite";
import { SvelteKitPWA } from "@vite-pwa/sveltekit";
import { defineConfig } from "vite-plus";

export default defineConfig({
  fmt: {},
  lint: { options: { typeAware: true, typeCheck: true } },
  plugins: [
    sveltekit(),
    SvelteKitPWA({
      manifest: false,
      filename: "service-worker.js",
      workbox: {
        globPatterns: ["**/*.{js,css,html,wasm,webmanifest}"],
      },
    }),
  ],
});
