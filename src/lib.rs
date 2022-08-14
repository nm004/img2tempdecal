mod fit;
mod remap;

use crate::{fit::*, remap::*};
use rgb::FromSlice;
use std::io::{self, Write};
use wad3::{MipMap, Wad};

pub fn convert_texture_to_tempdecal(
    texture: &[u8],
    width: usize,
    height: usize,
    out_larger_size: bool,
    write: &mut impl Write,
) -> Result<usize, io::Error> {
    let (texture, width, height) =
        resize_to_fit_into_tempdecal(texture.as_rgba(), width, height, out_larger_size);
    let (texture, palette) = remap_to_wad_texture(&texture, width, height);
    save_as_tempdecal(&texture, width, height, palette, write)
}

/// This writes tempdecal.wad with `write` object.
/// Most primary mipmap (i.e. mips0) is only valid,
/// though other mips are filled with 0xff.
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
