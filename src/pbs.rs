//! PBS means proximity-based shadowing. It is a form of shadowing specific to Voxel-Based games,
//! where, regardless of a light source, the game applies a shadow to voxels based on their
//! proximity to other voxels.
use crate::prelude::*;
use bevy::math::Vec3;
use bevy::render::mesh::{Mesh, VertexAttributeValues};

#[derive(Copy, Clone)]
/// Parameters for Proximity Based Shadowing
pub struct PbsParameters {
    /// How intense the shadow is. recommended range: 0.0 - 0.2
    pub pbs_value: f32,
    /// The minimum value of the shadow (0.0 - 1.0) 0.0 is very dark, and depending on
    /// your pbs_value it may reach that level. Recommended: 0.2 - 0.4
    pub min: f32,
    /// Smoothing will often lower the overall intensity of the shadowing, but in return
    /// the scene will look more uniform. Recommended: Low / High
    pub smoothing: PbsSmoothing,
}

#[derive(Clone, Copy)]
pub enum PbsSmoothing {
    Disabled,
    Low,
    High,
    VeryHigh,
}

impl Into<f32> for PbsSmoothing {
    fn into(self) -> f32 {
        match self {
            Self::Disabled => 1.0,
            Self::Low => 2.0,
            Self::High => 2.5,
            Self::VeryHigh => 3.0,
        }
    }
}

pub(crate) fn apply_pbs_quad(
    mesh: &mut Mesh,
    vivi: &VIVI,
    index: usize,
    face: Face,
    close_voxels: Neighbors,
    pbs: PbsParameters,
    voxel_dims: [f32; 3],
    dims: Dimensions,
) {
    let quad = vivi
        .get_quad_index(face, index)
        .expect("Couldn't find quad in vivi for pbs");

    let positions = mesh
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .expect("Cannot apply proximity-based-shadowing without the color attribute present");
    let VertexAttributeValues::Float32x3(positions) = positions else {
        panic!("Unexpected Format for the position attribute")
    };
    let ddd = three_d_cords(index, dims);
    let voxel_center = Vec3::from([
        ddd.0 as f32 * voxel_dims[0],
        ddd.1 as f32 * voxel_dims[1],
        ddd.2 as f32 * voxel_dims[2],
    ]);
    let positions = {
        let mut r = vec![];
        for i in quad..(quad + 4) {
            r.push(Vec3::from(positions[i as usize]))
        }
        r
    };
    let colors = mesh
        .attribute_mut(Mesh::ATTRIBUTE_COLOR)
        .expect("Cannot apply proximity-based-shadowing without the color attribute present");
    let VertexAttributeValues::Float32x4(ref mut colors) = colors else {
        panic!("Unexpected Format for the color attribute")
    };
    for i in 0..4 {
        let ver = i + quad;
        let diff = positions[i as usize] - voxel_center;
        let mut color = 0.0;
        if close_voxels[Right as usize] && (matches!(face, Right) ^ (diff.x > 0.0)) {
            color += pbs.pbs_value;
        }
        if close_voxels[Left as usize] && (matches!(face, Left) ^ (diff.x < 0.0)) {
            color += pbs.pbs_value;
        }
        if close_voxels[Top as usize] && (matches!(face, Top) ^ (diff.y > 0.0)) {
            color += pbs.pbs_value;
        }
        if close_voxels[Bottom as usize] && (matches!(face, Bottom) ^ (diff.y < 0.0)) {
            color += pbs.pbs_value;
        }
        if close_voxels[Back as usize] && (matches!(face, Back) ^ (diff.z > 0.0)) {
            color += pbs.pbs_value;
        }
        if close_voxels[Forward as usize] && (matches!(face, Forward) ^ (diff.z < 0.0)) {
            color += pbs.pbs_value;
        }

        let color = (1.0 - color.min(1.0).powf(pbs.smoothing.into())).max(pbs.min);
        colors[ver as usize] = [color, color, color, 1.0]
    }
}

pub(crate) fn apply_pbs(
    mesh: &mut Mesh,
    vivi: &VIVI,
    dims: Dimensions,
    lower_bound: usize,
    upper_bound: usize,
    pbs_value: PbsParameters,
    voxel_dims: [f32; 3],
) {
    for (i, quads) in vivi.vivi.iter().enumerate().skip(lower_bound) {
        if i > upper_bound {
            break;
        }
        for q in quads {
            let mut close_voxels: Neighbors = [false; 6];
            let face = face_from_u32(q & REVERSE_OFFSET_CONST);
            let mut count = 0;
            if let Some(neigbhor) = get_neighbor(i, face, dims) {
                for j in 0..6 {
                    if let Some(tmp) = get_neighbor(neigbhor, Face::from(j), dims) {
                        if !vivi.vivi[tmp].is_empty() {
                            close_voxels[j] = true;
                            count += 1;
                        }
                    }
                }
            }
            if count != 0 {
                apply_pbs_quad(
                    mesh,
                    vivi,
                    i,
                    face,
                    close_voxels,
                    pbs_value,
                    voxel_dims,
                    dims,
                );
            }
        }
    }
}
