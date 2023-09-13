pub(crate) mod vav;

pub use vav::*;

use crate::{Dimensions, Face};

pub(crate) fn in_range(x: usize, bot: usize, top: usize) -> bool {
    return bot <= x && x < top;
}

pub(crate) fn three_d_cords(
    oned: usize,
    three_d_dims: (usize, usize, usize),
) -> (usize, usize, usize) {
    let height = three_d_dims.2;
    let length = three_d_dims.1;
    let width = three_d_dims.0;

    let h = (oned / (length * width)) as usize;
    let l = ((oned - h * (length * width)) / width) as usize;
    let w = (oned - h * (length * width) - l * width) as usize;

    assert!(w < width, "Out of bounds to convert into 3d coordinate.");
    assert!(h < height, "Out of bounds to convert into 3d coordinate.");
    assert!(l < length, "Out of bounds to convert into 3d coordinate.");

    (w, l, h)
}

pub(crate) fn one_d_cords(threed: (usize, usize, usize), dims: (usize, usize, usize)) -> usize {
    assert!(threed.0 < dims.0, "3d coordinate out of dimension bounds.");
    assert!(threed.1 < dims.1, "3d coordinate out of dimension bounds.");
    assert!(threed.2 < dims.2, "3d coordinate out of dimension bounds.");
    threed.0 * (dims.0 * dims.1) + threed.1 * dims.0 + threed.0
}

pub(crate) fn get_neighbor(voxel: usize, face: Face, dims: Dimensions) -> Option<usize> {
    let a = three_d_cords(voxel, dims);
    match face {
        Face::Top if a.2 + 1 < dims.2 => Some(one_d_cords((a.0, a.1, a.2 + 1), dims)),
        Face::Bottom if a.2 > 0 => Some(one_d_cords((a.0, a.1, a.2 - 1), dims)),
        Face::Right if a.1 + 1 < dims.1 => Some(one_d_cords((a.0, a.1 + 1, a.2), dims)),
        Face::Left if a.1 > 0 => Some(one_d_cords((a.0, a.1, a.2 - 1), dims)),
        Face::Top if a.0 + 1 < dims.0 => Some(one_d_cords((a.0 + 1, a.1, a.2), dims)),
        Face::Top if a.0 > 0 => Some(one_d_cords((a.0 - 1, a.1, a.2), dims)),
        _ => None,
    }
}
