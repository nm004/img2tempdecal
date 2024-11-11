/*
 * This file is a part of img2tempdecal by Nozomi Miyamori.
 * img2tempdecal is distributed under the MIT-0 license and the Public Domain.
 */

use core::iter::{repeat, zip};
use imagequant::Histogram;
use rgb::{FromSlice, ComponentBytes, RGB8, RGBA8};

/// This converts the given texture into tempdecal.wad by calling the subsequent functions.
/// Texture must be raw RGBA8 format. This is an entry point of this library.
pub fn convert_texture_to_tempdecal(
    texture: &[u8],
    width: usize,
    height: usize,
    use_point_resample: bool,
) -> Vec<u8> {
    let (texture, width, height)
	= adjust_size(texture.as_rgba(), width, height, use_point_resample,
	    96 * 96, 112 * 128 + 1);
    let (palette, index_map) = remap_to_indexed_color(&texture, width, height);
    make_tempdecal(&palette, &index_map, width, height)
}

/// This resizes a given texture to fit into tempdecal.
fn adjust_size(
    texture: &[RGBA8],
    width: usize,
    height: usize,
    use_point_resample: bool,
    min_limit: usize,
    max_limit: usize
) -> (Vec<RGBA8>, usize, usize) {
    let (rem_w, rem_h) = (width % 16, height % 16);
    let (pad_w, pad_h) = ((16 - rem_w) % 16  , (16 - rem_h) % 16);

    if (rem_w, rem_h) == (0, 0) && width * height < max_limit {
	(texture.into(), width, height)
    } else if (width + pad_w) * (height + pad_h) < max_limit {
	// We extend an input texture if it already fits to tempdecal but the both width
	// and height are not multiples of 16. Padded pixels' alpha channel are 0.
	let (w, h) = (width + pad_w, height + pad_h);
	let (dx, dy) = (pad_w / 2, pad_h / 2);
	let mut texture1 = vec![RGBA8::new(0, 0, 0, 0); w * h];

	// Let's copy original textures.
	let rows0 = texture.chunks_exact(width);
	let rows1 = texture1.chunks_exact_mut(w).skip(dy).take(height);
	for (r0, r1) in rows0.zip(rows1) {
	    r1[dx..(dx + width)].copy_from_slice(r0);
	}
	(texture1, w, h)
    } else {
	// We have to resize the texture.
	//
	// First, let's find the largest width and height that have the most similar
	// aspect ratio that is valid for GoldSrc.

	// We make the ratio table.
	// Ref. https://www.the303.org/tutorials/goldsrcspraylogo.html
	let r = (16..=256).step_by(16);
	const N: usize = 256 / 16;
	// 16, 32, ..., 256, 16, 32, ..., 256
	let w: Box<_> = repeat(r.clone()).take(N).flatten().collect();
	// 16, 16, ...,  16, 16, 32, ..., 256
	let h: Box<_> = r.map(|i| [i; N]).flatten().collect();

	let ratio = width as f64 / height as f64;
	let (w, h, _) = zip(w.iter(), h.iter()).filter(|(&w, &h)| {
	    let s = w * h;
	    min_limit < s && s < max_limit
	}).map(|(&w, &h)| {
	    let r = w as f64 / h as f64;
	    let r = r - ratio;
	    (w, h, r * r)

	// rev(): If several elements are equally minimum, the first element is
	// returned (excerpt from the Rust doc).
	}).rev().min_by(|x, y|
	    x.2.partial_cmp(&y.2).unwrap()
	).unwrap();

	// Let's do resize the texture.
	let mut texture1 = vec![RGBA8::new(0, 0, 0xff, 0); w * h];
	let mut resizer = resize::new(
	    width,
	    height,
	    w,
	    h,
	    resize::Pixel::RGBA8,
	    if use_point_resample {
		resize::Type::Point
	    } else {
		resize::Type::Lanczos3
	    },
	).unwrap();
	resizer.resize(&texture, &mut texture1).unwrap();

	// This makes each color fully opaque if its opacity is above 50%,
	// otherwise makes it fully transparent. It is especially needed for
	// semitransparent images to avoid an undesirable quantization result.
	for i in texture1.iter_mut() {
	    i.a = i.a / 0x80 * 0xff
	}

	(texture1, w, h)
    }
}

/// This creates indexed color map
fn remap_to_indexed_color(
    texture: &[RGBA8],
    width: usize,
    height: usize,
) -> ([RGB8; 256], Vec<u8>) {
    // First, we set quantization parameters.
    let mut iq = imagequant::new();
    iq.set_last_index_transparent(true);
    let mut img = iq.new_image_borrowed(&texture, width, height, 0.0).unwrap();

    let mut hist = Histogram::new(&iq);
    hist.add_image(&iq, &mut img).unwrap();

    // Now, let's do the job.
    let mut r = hist.quantize(&iq).unwrap();
    r.set_dithering_level(1.0).unwrap();

    // This gets indexed color map and its palette.
    let (rgba_palette, mut index_map) = r.remapped(&mut img).unwrap();

    // This makes each pixel refer to the last index if it refers to transparent color.
    for p in index_map.iter_mut() {
        *p = if rgba_palette[*p as usize].a == 0 {
            0xff
        } else {
            *p
        };
    }

    // This makes a RGB pallet
    let mut rgb_palette: [RGB8; 256] = [RGB8 {r:0, g:0, b:0}; 256];
    let n = rgba_palette.len();
    let p: Box<_> = rgba_palette.into_iter().map(|c| c.rgb()).collect();
    rgb_palette[..n].copy_from_slice(&p);
    // Last index is for transparent color.
    *rgb_palette.last_mut().unwrap() = RGB8 {r:0, g:0, b:0xff};
    (rgb_palette, index_map)
}

/// This makes tempdecal.wad from the given indexed color map.
/// Only primary mipmap is used, and other mips are filled with 0xff.
fn make_tempdecal<'a>(
    palette: &'a [RGB8; 256],
    index_map: &'a [u8],
    width: usize,
    height: usize,
) -> Vec<u8> {
    const NAME: &[u8; 16] = b"{LOGO\0\0\0\0\0\0\0\0\0\0\0";
    let m0size = width * height;
    let m1size = width * height / 4;
    let m2size = width * height / 16;
    let m3size = width * height / 64;
    // texture_size = texture_header + (mips) + palette_count + palette
    let texture_size = 0x28 + (m0size + m1size + m2size + m3size) + 2 + 0x300;
    let texture_padding_size = (16 - texture_size % 16) % 16;
    let texture_size_aligned = texture_size + texture_padding_size;

    [
	// Header

	&b"WAD3"[..],
	// count of directory entries
	&1u32.to_le_bytes(),
	// offset to directory
	&(0x10 + texture_size_aligned as u32).to_le_bytes(),
	// padding
	&[0; 4],

	// Texture

	// name
	NAME,
	// width
	&(width as u32).to_le_bytes(),
	// height
	&(height as u32).to_le_bytes(),
	// mips offsets from the begining of texture
	&0x28_u32.to_le_bytes(),
	&(0x28 + (m0size) as u32).to_le_bytes(),
	&(0x28 + (m0size + m1size) as u32).to_le_bytes(),
	&(0x28 + (m0size + m1size + m2size) as u32).to_le_bytes(),
	// mipmaps
	&index_map,
	&vec![0xff; m1size],
	&vec![0xff; m2size],
	&vec![0xff; m3size],
	// count of colors in a palette (always 256)
	&256_u16.to_le_bytes(),
	// palette
	palette.as_bytes(),
	// padding
	&vec![0; texture_padding_size],

	// Directory

	// offset to texture from the begining of WAD file
	&0x10u32.to_le_bytes(),
	// compressed file size (the same as file size in disk)
	&(texture_size as u32).to_le_bytes(),
	// file size in disk
	&(texture_size as u32).to_le_bytes(),
	// data type (0x43 == mipmap texture)
	&[0x43],
	// compression flag (0 == not used)
	&[0],
	// padding
	&[0; 2],
	// name
	NAME,
    ].concat()
}
