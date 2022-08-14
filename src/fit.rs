use rgb::RGBA8;
use std::iter::{repeat, zip};

/// This resizes a texture to fit into tempdecal. If `for_svencoop` is true,
/// the texture can be bigger, but is not compatible with other GoldSrc games.
/// Besides, this applies denoise to the resized texture.
pub(super) fn resize_to_fit_into_tempdecal(
    texture: &[RGBA8],
    width: usize,
    height: usize,
    out_larger_size: bool,
) -> (Vec<RGBA8>, usize, usize) {
    // According to https://www.the303.org/tutorials/goldsrcspraylogo.html
    let sup_size = if out_larger_size { 14336 + 1 } else { 12288 };

    let (nw, nh) = calc_optimal_size(width, height, sup_size);
    let new_tex = if (nw, nh) == (width, height) {
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

    (new_tex, nw, nh)
}

/// This finds biggest, most similar size that fits into tempdecal.wad,
/// s.t. 64 =< result width, result height =< 256.
/// Though, if already fits into tempdecal, then return width and height as it is.
fn calc_optimal_size(width: usize, height: usize, sup_size: usize) -> (usize, usize) {
    if (width % 16, height % 16) == (0, 0)
        && width * height < sup_size
        && width >= 64
        && height >= 64
    {
        return (width, height);
    }

    let wh_r = width as f64 / height as f64;
    let r = (64..256 + 1).step_by(16);
    const COUNT: usize = (256 - 64) / 16;
    // 64, 80, 96, ..., 256, 64, 80, ...
    let w: Box<_> = repeat(r.clone()).take(COUNT).flatten().collect();
    // 64, 64, 64, ..., 64, 80, 80, ...
    let h: Box<_> = r.map(|i| [i; COUNT]).flatten().collect();

    let (i, _) = zip(w.iter(), h.iter())
        .map(|c| {
            let (nw, nh) = (*c.0, *c.1);
            let nwh_r = nw as f64 / nh as f64;
            let ceil_max = ((nw * nh / sup_size) as f64) * f64::MAX;

            (nwh_r - wh_r).abs() + ceil_max
        })
        .enumerate()
        .reduce(|a, b| if a.1 < b.1 { a } else { b })
        .unwrap();

    (w[i], h[i])
}
