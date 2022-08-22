mod utils;

use img2tempdecal::*;
use std::io::Cursor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn convert(
    texture: &[u8],
    width: usize,
    height: usize,
    out_larger_size: bool,
    dst: &mut [u8],
) -> usize {
    crate::utils::set_panic_hook();

    convert_texture_to_tempdecal(
        &texture,
        width,
        height,
        out_larger_size,
        &mut Cursor::new(dst),
    )
    .expect("Should not fail to write result. Maybe out of memory?")
}
