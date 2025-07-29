Img2tempdecal Web
====================

Image tempdecal.wad converter running in a web browser.

Build Requirements
----------------

 * [Bun](https://bun.sh/)
 * [Rust](https://www.rust-lang.com/)
 * [wasm-bindgen v0.2.95](https://github.com/rustwasm/wasm-bindgen/)
 * [wasm-opt (from binaryen)](https://github.com/WebAssembly/binaryen/)

Build
----------------

```
# Init build environment
bun install --no-save

# Development build
bun run dev

# Production build (output into 'dist')
bun run build

# Preview production build output
bun run preview
```

License
----------------

MIT-0 or Public domain (whichever you want).
