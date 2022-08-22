use rgb::RGBA8;
use std::iter::{repeat, zip};

/// This resizes a texture to fit into tempdecal. If `for_svencoop` is true,
/// the texture can be bigger, but is not compatible with other GoldSrc games.
/// Besides, this applies denoise to the resized texture.
pub(super) fn resize_to_fit_into_tempdecal(
    texture: &[RGBA8],
    width: usize,
    height: usize,
    larger_size: bool,
) -> (Vec<RGBA8>, usize, usize) {
    // According to https://www.the303.org/tutorials/goldsrcspraylogo.html
    let size_sup = if larger_size { 14336 + 1 } else { 12288 };

    let (nw, nh) = calc_optimal_size(width, height, size_sup);
    let ntxt = if (nw, nh) == (width, height) {
        texture.to_owned()
    } else {
        let mut dst = vec![RGBA8::default(); nw * nh];
        let mut resizer = resize::new(
            width,
            height,
            nw,
            nh,
            resize::Pixel::RGBA8,
            resize::Type::Lanczos3,
        )
        .unwrap();
        resizer.resize(texture, &mut dst).unwrap();

        // This applies 50% threshold to alpha channel of each pixels to denoise.
        for i in dst.iter_mut() {
            i.a = i.a / 0x80 * 0xff
        }

        dst
    };

    (ntxt, nw, nh)
}

/// This finds biggest and most similar texture size that fits into tempdecal.wad,
/// which holds 16 =< result width, result height =< 256.
/// Though, if already fits into tempdecal, then return width and height as it is.
fn calc_optimal_size(width: usize, height: usize, size_sup: usize) -> (usize, usize) {
    if (width % 16, height % 16) == (0, 0) && width * height < size_sup {
        return (width, height);
    }

    let wh_r = width as f64 / height as f64;
    let r = (16..256 + 1).step_by(16);
    const COUNT: usize = 256 / 16;
    // 16, 32, 48, ..., 224, 240, 256, 16, 32, ...
    let w: Box<_> = repeat(r.clone()).take(COUNT).flatten().collect();
    // 16, 16, 16, ..., 16, 16, 16, 32, 32...
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
