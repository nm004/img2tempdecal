use rgb::RGBA8;

/// This extends the size of the input texture. The resulting width and height
/// are multiples of 16.
pub(super) fn extend_to_m16(
    texture: &[RGBA8],
    width: usize,
    height: usize,
) -> (Vec<RGBA8>, usize, usize) {
    let (pad_x, pad_y) = (width % 16, height % 16);
    let (nw, nh) = (width + pad_x, height + pad_y);
    let mut ntxt = vec![RGBA8::new(0, 0, 0xff, 0); nw * nh];
    let (dx, dy) = (pad_x / 2, pad_y / 2);

    let nt_rows = ntxt.chunks_exact_mut(nw).skip(dy);
    let t_rows = texture.chunks_exact(width);
    for (nr, r) in nt_rows.zip(t_rows) {
        nr[dx..dx + width].copy_from_slice(r);
    }
    (ntxt, nw, nh)
}
