//! PBS means proximity-based shadowing. It is a form of shadowing specific to Voxel-Based games,
//! where, regardless of a light source, the game applies a shadow to voxels based on their
//! proximity to other voxels.
use crate::prelude::*;
use bevy::render::mesh::{Mesh, MeshVertexAttribute, VertexAttributeValues};

const PBS_VALUE_PER_CLOSE_VOXEL: f32 = 0.16;

pub(crate) fn apply_pbs_quad(
    mesh: &mut Mesh,
    vivi: &VIVI,
    index: usize,
    face: Face,
    prox_count: i32,
) {
    let quad = vivi
        .get_quad_index(face, index)
        .expect("Couldn't find quad in vivi for pbs");
    let colors = mesh
        .attribute_mut(Mesh::ATTRIBUTE_COLOR)
        .expect("Cannot apply proximity-based-shadowing without the color attribute present");
    let VertexAttributeValues::Float32x4(ref mut colors) = colors else { panic!("Unexpected Format for the color attribute")};
    let count = prox_count as f32;
    let color = [
        1.0 - PBS_VALUE_PER_CLOSE_VOXEL * count,
        1.0 - PBS_VALUE_PER_CLOSE_VOXEL * count,
        1.0 - PBS_VALUE_PER_CLOSE_VOXEL * count,
        1.0,
    ];
    let quad = quad as usize;
    colors[quad] = color;
    colors[quad + 1] = color;
    colors[quad + 2] = color;
    colors[quad + 3] = color;
}

pub(crate) fn apply_pbs(mesh: &mut Mesh, vivi: &VIVI, dims: Dimensions) {
    for (i, quads) in vivi.vivi.iter().enumerate() {
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
                apply_pbs_quad(mesh, vivi, i, face, count);
            }
        }
    }
}
