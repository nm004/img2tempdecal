use rgb::RGBA8;
use std::iter::{repeat, zip};

use crate::denoise;

/// This resizes the given texture to fit into tempdecal.
/// If `larger_size` is true, the resulting texture can be bigger,
/// but is only valid in Sven Co-op.
pub(super) fn resize_to_fit_into_tempdecal(
    texture: &[RGBA8],
    width: usize,
    height: usize,
    larger_size: bool,
    use_point_resample: bool
) -> (Box<[RGBA8]>, usize, usize) {
    // According to https://www.the303.org/tutorials/goldsrcspraylogo.html
    let size_sup = if larger_size { 14336 + 1 } else { 12288 };

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
        if use_point_resample { resize::Type::Point } else { resize::Type::Lanczos3 }
    )
    .unwrap();
    resizer.resize(texture, &mut ntxt).unwrap();

    denoise(&mut ntxt);

    (ntxt, nw, nh)
}

/// This finds biggest and most similar texture size that fits into tempdecal.wad,
/// which holds 16 =< result width, result height =< 256.
/// If the texture already fits, this returns the width and the height as it is.
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
