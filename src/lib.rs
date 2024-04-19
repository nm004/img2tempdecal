// This code is in the public domain.

pub use rgb::{FromSlice, RGBA8};

use imagequant::Histogram;
use rgb::ComponentBytes;
use std::io::{self, Write};
use std::iter::{repeat, zip};
use std::mem::{size_of, size_of_val};

const PALETTE_COUNT: u16 = 256;

/// This converts the given texture into tempdecal.wad by calling the subsequent functions.
/// Texture must be raw RGBA8 format. This is an entry point of this library.
pub fn convert_texture_to_tempdecal(
    write: &mut impl Write,
    texture: &[RGBA8],
    width: usize,
    height: usize,
    use_point_resample: bool,
) -> Result<usize, io::Error> {
    let (texture, width, height) = extend_to_m16(texture, width, height);
    let (texture, width, height) =
        resize_to_fit_into_tempdecal(&texture, width, height, use_point_resample);
    let (texture, palette) = remap_to_wad_texture(&texture, width, height);
    save_as_tempdecal(write, &texture, width, height, &palette)
}

/// This extends an input texture. The resulting width and height are multiples of 16.
/// Padded pixels are copied from the edge of an original texture, but their alpha channel
/// is 0. By doing so, undesirable aliasing on the edges can be avoided during resizing.
fn extend_to_m16(texture: &[RGBA8], width: usize, height: usize) -> (Box<[RGBA8]>, usize, usize) {
    let (pad_x, pad_y) = (width % 16, height % 16);
    if (pad_x, pad_y) == (0, 0) {
        return (texture.into(), width, height);
    }
    let (nw, nh) = (width + pad_x, height + pad_y);
    let mut ntxt = vec![RGBA8::new(0, 0, 0, 0); nw * nh].into_boxed_slice();
    let (dx, dy) = (pad_x / 2, pad_y / 2);

    let nt_rows = ntxt.chunks_exact_mut(nw).skip(dy);
    let t_rows = texture.chunks_exact(width);
    for (nr, r) in nt_rows.zip(t_rows) {
        nr[dx..dx + width].copy_from_slice(r);
    }

    if pad_x != 0 {
        for nr in ntxt.chunks_exact_mut(nw) {
            let a = nr[dx];
            let b = nr[nw - (pad_x - dx)];
            nr[..dx].fill(RGBA8::new(a.r, a.g, a.b, 0));
            nr[nw - dx..].fill(RGBA8::new(b.r, b.g, b.b, 0));
        }
    }

    if pad_y != 0 {
        let r = &ntxt[dy * nw..(dy + 1) * nw]
            .iter()
            .map(|x| RGBA8::new(x.r, x.g, x.b, 0))
            .collect::<Vec<_>>();
        for nr in ntxt[..dy * nw].chunks_exact_mut(nw) {
            nr.copy_from_slice(&r);
        }
        let r = &ntxt[nw * nh - (pad_y - dy + 1) * nw..nw * nh - (pad_y - dy) * nw]
            .iter()
            .map(|x| RGBA8::new(x.r, x.g, x.b, 0))
            .collect::<Vec<_>>();
        for nr in ntxt[nw * nh - dy * nw..].chunks_exact_mut(nw) {
            nr.copy_from_slice(&r);
        }
    }
    (ntxt, nw, nh)
}

/// This resizes a given texture to fit into tempdecal.
fn resize_to_fit_into_tempdecal(
    texture: &[RGBA8],
    width: usize,
    height: usize,
    use_point_resample: bool,
) -> (Box<[RGBA8]>, usize, usize) {
    // Ref. https://www.the303.org/tutorials/goldsrcspraylogo.html
    let size_sup = 14337;
    let (nw, nh) = calc_optimal_size(width, height, size_sup);
    if (nw, nh) == (width, height) {
        return (texture.into(), width, height);
    }

    let mut ntxt = vec![RGBA8::new(0, 0, 0xff, 0); nw * nh].into_boxed_slice();
    let mut resizer = resize::new(
        width,
        height,
        nw,
        nh,
        resize::Pixel::RGBA8,
        if use_point_resample {
            resize::Type::Point
        } else {
            resize::Type::Lanczos3
        },
    )
    .unwrap();
    resizer.resize(texture, &mut ntxt).unwrap();

    denoise(&mut ntxt);

    (ntxt, nw, nh)
}

/// This finds biggest and similar texture size that fits into tempdecal.wad.
fn calc_optimal_size(width: usize, height: usize, size_sup: usize) -> (usize, usize) {
    if (width % 16, height % 16) == (0, 0) && width * height < size_sup {
        return (width, height);
    }

    let wh_r = width as f64 / height as f64;
    let r = (16..256 + 1).step_by(16);
    const COUNT: usize = 256 / 16;
    // 16, 32, 48, ..., 224, 240, 256, 16, 32, ..., 240, 256
    let w: Box<_> = repeat(r.clone()).take(COUNT).flatten().collect();
    // 16, 16, 16, ..., 16, 16, 16, 32, 32..., 256, 256
    let h: Box<_> = r.map(|i| [i; COUNT]).flatten().collect();

    let (i, _) = zip(w.iter(), h.iter())
        .map(|c| {
            let (nw, nh) = (*c.0, *c.1);
            let nwh_r = nw as f64 / nh as f64;
            let ceil_max = ((nw * nh / size_sup) as f64) * f64::MAX;

            (nwh_r - wh_r).abs() + ceil_max
        })
        .enumerate()
        .reduce(|a, b| if a.1 < b.1 { a } else { b })
        .unwrap();

    (w[i], h[i])
}

/// This creates indexed color texture and its palette.
fn remap_to_wad_texture(
    texture: &[RGBA8],
    width: usize,
    height: usize,
) -> (Box<[u8]>, [u8; (PALETTE_COUNT as usize) * 3]) {
    // First, let's set quantization parameters.
    let mut liq = imagequant::new();
    liq.set_speed(1).unwrap();
    liq.set_last_index_transparent(true);

    let mut hist = Histogram::new(&liq);
    hist.add_fixed_color(imagequant::RGBA::new(0, 0, 0xff, 0), 0.0)
        .unwrap();

    let mut img = liq
        .new_image_borrowed(&texture, width, height, 0.0)
        .unwrap();
    hist.add_image(&liq, &mut img).unwrap();

    // Do quantize.
    let mut res = hist.quantize(&liq).unwrap();
    res.set_dithering_level(1.0).unwrap();

    // Get indexed color mips and a palette.
    let (mut palette, mips0) = res.remapped(&mut img).unwrap();
    let mut mips0 = mips0.into_boxed_slice();

    denoise(&mut palette);

    // If a pixel refers to transparent color, then make it refer to 0xff.
    for p in mips0.iter_mut() {
        *p = if palette[*p as usize].a == 0 {
            0xff
        } else {
            *p
        };
    }

    // from [RGBA] to [RGB]
    let mut palette: Vec<_> = palette.into_iter().map(|c| c.rgb()).collect();

    // Palette size must be 256.
    palette.resize(256, [0, 0, 0xff].into());

    // Texture is masked if last (0xff) color is 0x0000ff (pure blue).
    *palette.last_mut().unwrap() = [0, 0, 0xff].into();

    let palette: [u8; 256 * 3] = palette.as_bytes().try_into().unwrap();

    (mips0, palette)
}

/// This makes alpha channel of each pixels 0xff
/// if it is above or equal to the half of the maximum value.
fn denoise(pixels: &mut [RGBA8]) {
    for i in pixels.iter_mut() {
        i.a = i.a / 0x80 * 0xff
    }
}

/// This writes tempdecal.wad with the `write` object.
/// Only most primary mipmap (i.e. mips0) is used,
/// whereas other mips are filled with 0xff.
fn save_as_tempdecal<'a>(
    write: &mut impl Write,
    mips0: &'a [u8],
    width: usize,
    height: usize,
    palette: &'a [u8; (PALETTE_COUNT as usize) * 3],
) -> Result<usize, io::Error> {
    type Magic = [u8; 4];
    type Name = [u8; 16];
    const MAGIC: &Magic = b"WAD3";
    let name: &Name = b"{LOGO\0\0\0\0\0\0\0\0\0\0\0";
    let w: u32 = width as u32;
    let h: u32 = height as u32;
    let m0size = w*h;
    let m1size = m0size / 4;
    let m2size = m0size / 16;
    let m3size = m0size / 64;
    let o0: u32 = 40;
    let o1: u32 = o0 + m0size;
    let o2: u32 = o1 + m1size;
    let o3: u32 = o2 + m2size;
    let mips1: &[u8] = &vec![0xff; m1size as usize];
    let mips2: &[u8] = &vec![0xff; m2size as usize];
    let mips3: &[u8] = &vec![0xff; m3size as usize];
    let header_size: u32 = (size_of::<Magic>() + size_of::<u32>() + size_of::<u32>()) as u32;
    let texture_size: u32 = (size_of::<Name>()
        + size_of::<u32>()
        + size_of::<u32>()
        + size_of::<u32>()
        + size_of::<u32>()
        + size_of::<u32>()
        + size_of::<u32>()
        + size_of_val(mips0)
        + size_of_val(mips1)
        + size_of_val(mips2)
        + size_of_val(mips3)
        + size_of::<u16>()
        + size_of::<[u8; 3 * PALETTE_COUNT as usize]>() // R, G, B
        + size_of::<[u8; 2]>()) as u32;
    let dir_entry_size: u32 = (size_of::<u32>()
        + size_of::<u32>()
        + size_of::<u32>()
        + size_of::<u8>()
        + size_of::<u8>()
        + size_of::<[u8; 2]>()
        + size_of::<Name>()) as u32;

    // header
    write.write_all(MAGIC)?;
    write.write_all(&1u32.to_le_bytes())?;
    write.write_all(&(header_size + texture_size).to_le_bytes())?; // offset to directory

    // texture
    write.write_all(name)?;
    write.write_all(&w.to_le_bytes())?;
    write.write_all(&h.to_le_bytes())?;
    write.write_all(&o0.to_le_bytes())?; // offset from begining of texture
    write.write_all(&o1.to_le_bytes())?;
    write.write_all(&o2.to_le_bytes())?;
    write.write_all(&o3.to_le_bytes())?;
    write.write_all(mips0)?; // mipmap data
    write.write_all(mips1)?;
    write.write_all(mips2)?;
    write.write_all(mips3)?;
    write.write_all(&PALETTE_COUNT.to_le_bytes())?; // always 256
    write.write_all(palette)?;
    write.write_all(&[0; 2])?; // padding

    // directory
    write.write_all(&header_size.to_le_bytes())?; // offset to texture from the begining of WAD file
    write.write_all(&texture_size.to_le_bytes())?; // compressed file size (same with file size in disk)
    write.write_all(&texture_size.to_le_bytes())?; // file size in disk
    write.write_all(&0x43u8.to_le_bytes())?; // data type is mipmap
    write.write_all(&0u8.to_le_bytes())?; // use compression (always 0: never used)
    write.write_all(&[0; 2])?; // padding
    write.write_all(name)?;

    Ok((header_size + texture_size + dir_entry_size) as usize)
}
