pub(crate) mod compressed_voxel_grid;
pub mod vav;

use crate::prelude::{Dimensions, Face};

pub(crate) fn in_range(x: usize, bot: usize, top: usize) -> bool {
    return bot <= x && x < top;
}

pub(crate) fn three_d_cords(oned: usize, dims: Dimensions) -> (usize, usize, usize) {
    let height = dims.2;
    let length = dims.1;
    let width = dims.0;

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
    threed.2 * (dims.0 * dims.1) + threed.1 * dims.0 + threed.0
}

/// Function to get the neighbor towards the `Face` in a 3d grid.
/// None if the neighbor is out of bounds. For example:
/// assert_eq!(get_neighbor(0, Top, (2,2,2)), Some(5));
/// assert_eq!(get_neighbor(5, Top, (2,2,2)), None);
pub fn get_neighbor(voxel: usize, face: Face, dims: Dimensions) -> Option<usize> {
    let a = three_d_cords(voxel, dims);
    match face {
        Face::Top if a.2 + 1 < dims.2 => Some(one_d_cords((a.0, a.1, a.2 + 1), dims)),
        Face::Bottom if a.2 > 0 => Some(one_d_cords((a.0, a.1, a.2 - 1), dims)),
        Face::Back if a.1 + 1 < dims.1 => Some(one_d_cords((a.0, a.1 + 1, a.2), dims)),
        Face::Forward if a.1 > 0 => Some(one_d_cords((a.0, a.1 - 1, a.2), dims)),
        Face::Right if a.0 + 1 < dims.0 => Some(one_d_cords((a.0 + 1, a.1, a.2), dims)),
        Face::Left if a.0 > 0 => Some(one_d_cords((a.0 - 1, a.1, a.2), dims)),
        _ => None,
    }
}
