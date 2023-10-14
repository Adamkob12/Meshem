//! PBS means proximity-based shadowing. It is a form of shadowing specific to Voxel-Based games,
//! where, regardless of a light source, the game applies a shadow to voxels based on their
//! proximity to other voxels.
use crate::prelude::*;
use bevy::render::mesh::{Mesh, MeshVertexAttribute, VertexAttributeValues};

#[derive(Copy, Clone)]
pub struct PbsParameters {
    // recommended range: 0.0 - 0.2
    pub pbs_value: f32,
    // recommended range: 0.0 - 0.5
    pub pbs_smoothing: f32,
}

pub(crate) fn apply_pbs_quad(
    mesh: &mut Mesh,
    vivi: &VIVI,
    index: usize,
    face: Face,
    prox_count: i32,
    pbs: PbsParameters,
) {
    let quad = vivi
        .get_quad_index(face, index)
        .expect("Couldn't find quad in vivi for pbs");
    let colors = mesh
        .attribute_mut(Mesh::ATTRIBUTE_COLOR)
        .expect("Cannot apply proximity-based-shadowing without the color attribute present");
    let VertexAttributeValues::Float32x4(ref mut colors) = colors else { panic!("Unexpected Format for the color attribute")};
    let pbs_final = {
        let mut r = 0.0;
        for i in 0..prox_count {
            r += pbs.pbs_value * pbs.pbs_smoothing.powi(i);
        }
        r
    };
    let color = [1.0 - pbs_final, 1.0 - pbs_final, 1.0 - pbs_final, 1.0];
    let quad = quad as usize;
    colors[quad] = color;
    colors[quad + 1] = color;
    colors[quad + 2] = color;
    colors[quad + 3] = color;
}

pub(crate) fn apply_pbs(
    mesh: &mut Mesh,
    vivi: &VIVI,
    dims: Dimensions,
    lower_bound: usize,
    upper_bound: usize,
    pbs_value: PbsParameters,
) {
    for (i, quads) in vivi.vivi.iter().enumerate().skip(lower_bound) {
        if i > upper_bound {
            break;
        }
        for q in quads {
            let face = face_from_u32(q & REVERSE_OFFSET_CONST);
            let mut count = 0;
            if let Some(neigbhor) = get_neighbor(i, face, dims) {
                for i in 0..6 {
                    if let Some(tmp) = get_neighbor(neigbhor, Face::from(i), dims) {
                        if !vivi.vivi[tmp].is_empty() {
                            count += 1;
                        }
                    }
                }
            }
            if count != 0 {
                apply_pbs_quad(mesh, vivi, i, face, count, pbs_value);
            }
        }
    }
}

// pub(crate) fn apply_pbs_voxel(mesh: &mut Mesh, vivi: &VIVI, dims: Dimensions, voxel: usize) {
//     let mut voxels = vec![];
//     let (back, voxel, forward) = (
//         get_neighbor(voxel, Forward, dims).unwrap_or(voxel),
//         voxel,
//         get_neighbor(voxel, Back, dims).unwrap_or(voxel),
//     );
//     for j in 0..4 {
//         if let Some(t) = get_neighbor(voxel, Face::from(j), dims) {
//             voxels.push(t);
//         }
//         if let Some(t) = get_neighbor(back, Face::from(j), dims) {
//             voxels.push(t);
//         }
//         if let Some(t) = get_neighbor(forward, Face::from(j), dims) {
//             voxels.push(t);
//         }
//     }
//     voxels.push(voxel);
//     voxels.push(forward);
//     voxels.push(back);
//
//     for (i, quads) in voxels.iter().map(|x| (*x, &vivi.vivi[*x])) {
//         for q in quads {
//             let face = face_from_u32(q & REVERSE_OFFSET_CONST);
//             let mut count = 0;
//             if let Some(neigbhor) = get_neighbor(i, face, dims) {
//                 for i in 0..6 {
//                     if let Some(tmp) = get_neighbor(neigbhor, Face::from(i), dims) {
//                         if !vivi.vivi[tmp].is_empty() {
//                             count += 1;
//                         }
//                     }
//                 }
//             }
//             if count != 0 {
//                 apply_pbs_quad(mesh, vivi, i, face, count, );
//             }
//         }
//     }
// }
