// This code is in the public domain.

use img2tempdecal::*;
use std::io::Cursor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn convert(
    dst: &mut [u8],
    texture: &[u8],
    width: usize,
    height: usize,
    larger_size: bool,
    use_point_resample: bool,
) -> usize {
    set_panic_hook();

    convert_texture_to_tempdecal(
        &mut Cursor::new(dst),
        texture.as_rgba(),
        width,
        height,
        larger_size,
        use_point_resample,
    )
    .expect("Should not fail to convert texture. Maybe out of memory?")
}

fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
