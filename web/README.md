Img2tempdecal Web
====================

Image tempdecal.wad converter running in a web browser.

Build Requirements
----------------

 * [Deno](https://deno.com/)
 * [Rust](https://www.rust-lang.com/)
 * [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen/)
 * [wasm-opt (from binaryen)](https://github.com/WebAssembly/binaryen/)

Build
----------------

```
# Development build
deno task dev

# Production build (output into 'dist')
deno task build

# Preview production build output
deno task preview
```

License
----------------

MIT-0 or Public domain (whichever you want).
