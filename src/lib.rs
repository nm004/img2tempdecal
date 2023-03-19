mod extend;
mod fit;
mod remap;

use crate::{extend::*, fit::*, remap::*};
use rgb::{FromSlice, RGBA8};
use std::io::{self, Write};
use wad3::{MipMap, Wad};

/// This converts the given texture into tempdecal.wad by calling
/// the subsequent functions. This is the only entry point.
pub fn convert_texture_to_tempdecal(
    texture: &[u8],
    width: usize,
    height: usize,
    larger_size: bool,
    use_point_resample: bool,
    write: &mut impl Write,
) -> Result<usize, io::Error> {
    let texture = texture.as_rgba();

    let (texture, width, height) = extend_to_m16(texture, width, height);
    let (texture, width, height) =
        resize_to_fit_into_tempdecal(&texture, width, height, larger_size, use_point_resample);
    let (texture, palette) = remap_to_wad_texture(&texture, width, height);
    save_as_tempdecal(&texture, width, height, palette, write)
}

/// This writes tempdecal.wad with the `write` object.
/// Only most primary mipmap (i.e. mips0) is used,
/// whereas other mips are filled with 0xff.
fn save_as_tempdecal<'a>(
    mips0: &'a [u8],
    width: usize,
    height: usize,
    palette: [u8; 256 * 3],
    write: &mut impl Write,
) -> Result<usize, io::Error> {
    let mipmaps = [
        mips0,
        &vec![0xff; width * height / 4],
        &vec![0xff; width * height / 16],
        &vec![0xff; width * height / 64],
    ];
    let mm = MipMap::new(width as u32, height as u32, mipmaps, &palette);

    let wad = Wad::new([(*b"{LOGO\0\0\0\0\0\0\0\0\0\0\0", mm.into())].into());
    wad.save(write)?;
    Ok(wad.size())
}

/// This applies 50% threshold to alpha channel of each pixels to denoise.
fn denoise(pixels: &mut [RGBA8]) {
    for i in pixels.iter_mut() {
        i.a = i.a / 0x80 * 0xff
    }
}
