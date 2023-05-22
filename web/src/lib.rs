use img2tempdecal::*;
use std::io::Cursor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn convert(
    texture: &[u8],
    width: usize,
    height: usize,
    out_larger_size: bool,
    use_point_resample: bool,
    dst: &mut [u8],
) -> usize {
    set_panic_hook();

    convert_texture_to_tempdecal(
        &texture,
        width,
        height,
        out_larger_size,
        use_point_resample,
        &mut Cursor::new(dst),
    )
    .expect("Should not fail to write result. Maybe out of memory?")
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
