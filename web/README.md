Img2tempdecal Web
====================

Image tempdecal.wad converter running in a web browser.

Build Requirements
----------------

 * [Bun](https://bun.sh/)
 * [Rust](https://www.rust-lang.com/)
 * [vite+](https://viteplus.dev/)
 * [wasm-pack](https://wasm-bindgen.github.io/wasm-pack/)

Build
----------------

```
# Init build environment
bun run wasm-pack:dev && bun install --no-save

# Development build
bun run dev

# Production build (output into 'dist')
bun run wasm-pack && bun run build

# Preview production build output
bun run preview
```

License
----------------

MIT-0 or Public domain (whichever you want).
