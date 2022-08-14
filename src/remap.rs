use imagequant::Histogram;
use rgb::{ComponentBytes, RGBA8};

/// This creates indexed color texture and its palette.
/// Besides, this will do some denoise to the palette.
pub(super) fn remap_to_wad_texture(
    texture: &[RGBA8],
    width: usize,
    height: usize,
) -> (Vec<u8>, [u8; 256 * 3]) {
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

    // Let's get indexed color mips and a palette.
    let (mut palette, mut mips0) = res.remapped(&mut img).unwrap();

    // This applies 50% threshold to alpha channel for the palette.
    for i in palette.iter_mut() {
        i.a = i.a / 0x80 * 0xff
    }

    // If a pixel refers to transparent color, then make it refer to 0xff.
    for p in mips0.iter_mut() {
        if palette[*p as usize].a == 0 {
            *p = 0xff;
        }
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
