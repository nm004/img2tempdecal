/*
 * This file is a part of img2tempdecal by Nozomi Miyamori.
 * img2tempdecal is distributed under the MIT-0 license and the Public Domain.
 */

use img2tempdecal::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn convert(
    dst: &mut [u8],
    texture: &[u8],
    width: usize,
    height: usize,
    use_point_resample: bool,
) -> usize {
    set_panic_hook();

    let data = convert_texture_to_tempdecal(
        texture,
        width,
        height,
        use_point_resample,
    );
    let n = data.len();
    dst[..n].copy_from_slice(&data);
    n
}

const fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
