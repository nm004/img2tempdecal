/*
 * This file is a part of img2tempdecal by Nozomi Miyamori.
 * img2tempdecal is distributed under the MIT-0 license and the Public Domain.
 */

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
	= resize_to_fit_into_tempdecal(texture.as_rgba(), width, height, use_point_resample);
    let (palette, index_map) = remap_to_indexed_color(&texture, width, height);
    make_tempdecal(&palette, &index_map, width, height)
}

/// This resizes a given texture to fit into tempdecal.
fn resize_to_fit_into_tempdecal(
    texture: &[RGBA8],
    width: usize,
    height: usize,
    use_point_resample: bool,
) -> (Vec<RGBA8>, usize, usize) {
    // First, we extend an input texture. The resulting width and height are multiples of 16.
    // Padded pixels are copied from the edge of an original texture, but their alpha channel
    // is 0. By doing so, undesirable resizing aliasing on the edges can be avoided.
    let (pad_x, pad_y) = ((16 - (width % 16)) % 16  , (16 - (height % 16)) % 16);

    let (texture, width, height) = if (pad_x, pad_y) == (0, 0) {
	(texture.into(), width, height)
    } else {
	let (w1, h1) = (width + pad_x, height + pad_y);
	let mut texture1 = vec![RGBA8::new(0, 0, 0, 0); w1 * h1];
	let (dx, dy) = (pad_x / 2, pad_y / 2);

	// This copies original textures
	let rows0 = texture.chunks_exact(width);
	let rows1 = texture1.chunks_exact_mut(w1).skip(dy).take(height);
	for (r0, r1) in rows0.zip(rows1) {
            r1[dx..(dx + width)].copy_from_slice(r0);
	}

	// The following code fills padded pixels with edge pixels.
	// Assume that the following diagram, where 5 is the
	// original texture area, and other areas are padded pixels.
	// 1 2 3
	// 4 5 6
	// 7 8 9
	// Left and right (4 and 6)
	if pad_x != 0 {
            for r in texture1.chunks_exact_mut(w1) {
		let a = r[dx];
		let b = r[w1 - (pad_x - dx)];
		r[..dx].fill(RGBA8::new(a.r, a.g, a.b, 0));
		r[(w1 - dx)..].fill(RGBA8::new(b.r, b.g, b.b, 0));
            }
	}

	// Top and bottom (1,2,3, and 7,8,9)
	if pad_y != 0 {
            let r0: Box<_> = texture1[dy * w1..(dy + 1) * w1].into_iter()
		.map(|x| RGBA8::new(x.r, x.g, x.b, 0))
		.collect();
            for r in texture1[..w1 * dy].chunks_exact_mut(w1) {
		r.copy_from_slice(&r0);
            }
            let r0: Box<_> = texture1[(w1 * (dy + height - 1))..(w1 * (dy + height))].into_iter()
		.map(|x| RGBA8::new(x.r, x.g, x.b, 0))
		.collect();
            for r in texture1[w1 * (dy + height)..].chunks_exact_mut(w1) {
		r.copy_from_slice(&r0);
            }
	}
        (texture1, w1, h1)
    };

    // If it already fits to tempdecal we do nothing here, or
    // we resize it.
    let (texture, width, height) = if width * height < 14337 {
        (texture.into(), width, height)
    } else {
	// Ref. https://www.the303.org/tutorials/goldsrcspraylogo.html
	const RATIO_TABLE: &[f64] = &[
	    16./ 16.,  16./ 32.,  16./ 48.,  16./ 64.,  16./ 80.,  16./ 96.,  16./112.,  16./128.,
	    16./144.,  16./160.,  16./176.,  16./192.,  16./208.,  16./224.,  16./240.,  16./256.,

	    32./ 16.,  32./ 32.,  32./ 48.,  32./ 64.,  32./ 80.,  32./ 96.,  32./112.,  32./128.,
	    32./144.,  32./160.,  32./176.,  32./192.,  32./208.,  32./224.,  32./240.,  32./256.,

	    48./ 16.,  48./ 32.,  48./ 48.,  48./ 64.,  48./ 80.,  48./ 96.,  48./112.,  48./128.,
	    48./144.,  48./160.,  48./176.,  48./192.,  48./208.,  48./224.,  48./240.,  48./256.,

	    64./ 16.,  64./ 32.,  64./ 48.,  64./ 64.,  64./ 80.,  64./ 96.,  64./112.,  64./128.,
	    64./144.,  64./160.,  64./176.,  64./192.,  64./208.,  64./224.,  f64::NAN,  f64::NAN,

	    80./ 16.,  80./ 32.,  80./ 48.,  80./ 64.,  80./ 80.,  80./ 96.,  80./112.,  80./128.,
	    80./144.,  80./160.,  80./176.,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,

	    96./ 16.,  96./ 32.,  96./ 48.,  96./ 64.,  96./ 80.,  96./ 96.,  96./112.,  96./128.,
	    96./144.,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,

	    112./ 16., 112./ 32., 112./ 48., 112./ 64., 112./ 80., 112./ 96., 112./112., 112./128.,
	    f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,

	    128./ 16., 128./ 32., 128./ 48., 128./ 64., 128./ 80., 128./ 96., 128./112., f64::NAN,
	    f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,

	    144./ 16., 144./ 32., 144./ 48., 144./ 64., 144./ 80., 144./ 96., f64::NAN,  f64::NAN,
	    f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,

	    160./ 16., 160./ 32., 160./ 48., 160./ 64., 160./ 80., f64::NAN,  f64::NAN,  f64::NAN,
	    f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,

	    176./ 16., 176./ 32., 176./ 48., 176./ 64., 176./ 80., f64::NAN,  f64::NAN,  f64::NAN,
	    f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,

	    192./ 16., 192./ 32., 192./ 48., 192./ 64., f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,
	    f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,

	    208./ 16., 208./ 32., 208./ 48., 208./ 64., f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,
	    f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,

	    224./ 16., 224./ 32., 224./ 48., 224./ 64., f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,
	    f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,

	    240./ 16., 240./ 32., 240./ 48., f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,
	    f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,

	    256./ 16., 256./ 32., 256./ 48., f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,
	    f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,
	];
	// This finds the biggest and most similar width and height from RATIO_TABLE.
	let r = width as f64 / height as f64;
	let i = RATIO_TABLE.into_iter().enumerate().fold((0xff, f64::NAN), |x, y| {
	    if y.1.is_nan() {
		return x
	    }
	    let z = (y.1 - r).abs();
	    if x.1 < z {
		x
	    } else if x.1 == z {
		if x.0 < y.0 {
		    (y.0, z)
		} else {
		    x
		}
	    } else {
		(y.0, z)
	    }
	}).0;

	let w1 = ((i / 16) + 1) * 16;
	let h1 = ((i % 16) + 1) * 16;

	let mut texture1 = vec![RGBA8::new(0, 0, 0xff, 0); w1 * h1];
	let mut resizer = resize::new(
            width,
            height,
            w1,
            h1,
            resize::Pixel::RGBA8,
            if use_point_resample {
		resize::Type::Point
            } else {
		resize::Type::Lanczos3
            },
	).unwrap();
	resizer.resize(&texture, &mut texture1).unwrap();

	// This makes each color fully opaque if its opacity is above 50%,
	// otherwise makes it fully transparent. It is especially needed for images that
	// uses transparency color gradient to avoid an undesirable quantization result.
	for i in texture1.iter_mut() {
	    i.a = i.a / 0x80 * 0xff
	}

	(texture1, w1, h1)
    };

    (texture, width, height)
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
    let mut rgb_palette: [RGB8; 256]
	= [RGB8 {r:0, g:0, b:0}; 256];
    let n = rgba_palette.len();
    let p: Box<_> = rgba_palette.into_iter().map(|c| c.rgb()).collect();
    rgb_palette[..n].copy_from_slice(&p);
    // Last index is for transparent color.
    *rgb_palette.last_mut().unwrap() = RGB8 {r:0, g:0, b:0xff};
    (rgb_palette, index_map)
}

/// This makes tempdecal.wad from the given indexed color map.
/// Only primary mipmap is used, and other mips are filled with 0.
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
    // texture_header + mips + palette_count + palette
    let texture_size = 0x30 + m0size + m1size + m2size + m3size + 2 + 0x300;
    let texture_size_aligned = texture_size + (16 - texture_size % 16) % 16;

    [
	// Header

	&b"WAD3"[..],
	// count of directory entries
	&1u32.to_le_bytes(),
	// offset to directory
	&(0x10 + texture_size_aligned as u32).to_le_bytes(),
	// padding for alignment
	&[0; 4],

	// Texture

	// name
	NAME,
	// width
	&(width as u32).to_le_bytes(),
	// height
	&(height as u32).to_le_bytes(),
	// mips offsets from the begining of texture
	&0x30_u32.to_le_bytes(),
	&(0x30 + (m0size) as u32).to_le_bytes(),
	&(0x30 + (m0size + m1size) as u32).to_le_bytes(),
	&(0x30 + (m0size + m1size + m2size) as u32).to_le_bytes(),
	// padding for alignment
	&[0; 8],
	// mipmaps
	&index_map,
	&vec![0; m1size],
	&vec![0; m2size],
	&vec![0; m3size],
	// count of colors in a palette (always 256)
	&256_u16.to_le_bytes(),
	// palette
	palette.as_bytes(),
	// padding
	&vec![0; (16 - texture_size % 16) % 16],

	// Directory

	// offset to texture from the begining of WAD file
	// (deader + directory)
	&0x10u32.to_le_bytes(),
	// compressed file size (same with file size in disk)
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
