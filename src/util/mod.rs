pub(crate) mod compressed_voxel_grid;
pub mod vav;

use crate::prelude::{Dimensions, Face};

pub(crate) fn in_range(x: usize, bot: usize, top: usize) -> bool {
    return bot <= x && x < top;
}

use crate::prelude::*;
use bevy::prelude::Vec3;

pub fn position_to_chunk(pos: Vec3, chunk_dims: (usize, usize, usize)) -> [i32; 2] {
    let chunk_width = chunk_dims.0;
    let chunk_length = chunk_dims.2;
    let x = pos.x + 0.5;
    let z = pos.z + 0.5;
    [
        (x / chunk_width as f32 + (x.signum() - 1.0) / 2.0) as i32,
        (z / chunk_length as f32 + (z.signum() - 1.0) / 2.0) as i32,
    ]
}

// the bool is for whether or not the pos is within the height bounds
pub fn position_to_chunk_position(
    pos: Vec3,
    chunk_dims: (usize, usize, usize),
) -> ([i32; 2], [usize; 3], bool) {
    let chunk_width = chunk_dims.0;
    let chunk_length = chunk_dims.2;
    let chunk_height = chunk_dims.1;
    let chunk = position_to_chunk(pos, chunk_dims);

    let x = pos.x + 0.5;
    let z = pos.z + 0.5;
    let y = pos.y + 0.5;

    let chunk_pos = [
        (x - chunk[0] as f32 * chunk_width as f32) as usize,
        y as usize,
        (z - chunk[1] as f32 * chunk_length as f32) as usize,
    ];

    let flag = y >= 0.0 && y <= chunk_height as f32;
    (chunk, chunk_pos, flag)
}

pub const fn three_d_cords(oned: usize, dims: (usize, usize, usize)) -> (usize, usize, usize) {
    let height = dims.1;
    let length = dims.2;
    let width = dims.0;

    let h = (oned / (length * width)) as usize;
    let l = ((oned - h * (length * width)) / width) as usize;
    let w = (oned - h * (length * width) - l * width) as usize;

    assert!(w < width, "Out of bounds to convert into 3d coordinate.");
    assert!(h < height, "Out of bounds to convert into 3d coordinate.");
    assert!(l < length, "Out of bounds to convert into 3d coordinate.");

    (w, h, l)
}

pub const fn three_d_cords_arr(oned: usize, dims: (usize, usize, usize)) -> [usize; 3] {
    let height = dims.1;
    let length = dims.2;
    let width = dims.0;

    let h = (oned / (length * width)) as usize;
    let l = ((oned - h * (length * width)) / width) as usize;
    let w = (oned - h * (length * width) - l * width) as usize;

    assert!(w < width, "Out of bounds to convert into 3d coordinate.");
    assert!(h < height, "Out of bounds to convert into 3d coordinate.");
    assert!(l < length, "Out of bounds to convert into 3d coordinate.");

    [w, h, l]
}

pub const fn one_d_cords(threed: [usize; 3], dims: (usize, usize, usize)) -> usize {
    assert!(threed[0] < dims.0, "3d coordinate out of dimension bounds.");
    assert!(threed[1] < dims.1, "3d coordinate out of dimension bounds.");
    assert!(threed[2] < dims.2, "3d coordinate out of dimension bounds.");
    threed[1] * (dims.0 * dims.2) + threed[2] * dims.0 + threed[0]
}

pub const fn one_d_cords_safe(threed: [usize; 3], dims: (usize, usize, usize)) -> Option<usize> {
    if threed[0] >= dims.0 || threed[1] >= dims.1 || threed[2] >= dims.2 {
        None
    } else {
        Some(threed[1] * (dims.0 * dims.2) + threed[2] * dims.0 + threed[0])
    }
}

// Extract the vertex data for the physics engine.
use bevy::render::mesh::{Mesh, VertexAttributeValues};
pub fn extract_position_vertex_data(mesh: &Mesh) -> Vec<Vec3> {
    let VertexAttributeValues::Float32x3(pos_vertices) =
        mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap()
    else {
        panic!("Vertex position data should be in `VertexAttributeValues::Float32x3`")
    };
    pos_vertices
        .iter()
        .map(|[x, y, z]| Vec3::new(*x, *y, *z))
        .collect()
}

// Extract the indices for the physics engine.
use bevy::render::mesh::Indices;
pub fn extract_indices_data(mesh: &Mesh) -> Vec<[u32; 3]> {
    let Indices::U32(indices) = mesh.indices().unwrap() else {
        panic!("Indices data shoud be in `Indices::U32` format")
    };
    indices
        .chunks(3)
        .map(|chunk| [chunk[0], chunk[1], chunk[2]])
        .collect()
}

pub fn iter_faces_of_chunk(dims: Dimensions, face: Face) -> impl Iterator<Item = usize> {
    let all = 0..(dims.0 * dims.1 * dims.2);
    let m = [dims.1 - 1, 0, dims.0 - 1, 0, dims.2 - 1, 0];
    let i = match face {
        Top | Bottom => 1,
        Right | Left => 0,
        Forward | Back => 2,
    };
    let all = all.filter(move |x| three_d_cords_arr(*x, dims)[i] == m[face as usize]);
    all
}

pub fn is_block_on_edge(dims: Dimensions, index: usize) -> Vec<Face> {
    let mut to_return = vec![];
    let m = [dims.1 - 1, 0, dims.0 - 1, 0, dims.2 - 1, 0];
    for i in 0..6 {
        let face = Face::from(i);
        let j = match face {
            Top | Bottom => 1,
            Right | Left => 0,
            Forward | Back => 2,
        };
        if three_d_cords_arr(index, dims)[j] == m[face as usize] {
            to_return.push(face);
        }
    }
    to_return
}

pub fn get_neigbhor_across_chunk(dims: Dimensions, index: usize, face: Face) -> usize {
    for f in is_block_on_edge(dims, index) {
        if f as usize == face as usize {
            return match face {
                Right => index - dims.0 + 1,
                Left => index + dims.0 - 1,
                Back => index - dims.0 * (dims.2 - 1),
                Forward => index + dims.0 * (dims.2 - 1),
                _ => panic!("Shouldn't happen"),
            };
        }
    }
    panic!("`get_neigbhor_across_chunk` was called on a block that wasn't on the edge of a chunk");
}

/// Function to get the neighbor towards the `Face` in a 3d grid.
/// None if the neighbor is out of bounds. For example:
/// assert_eq!(get_neighbor(0, Top, (2,2,2)), Some(5));
/// assert_eq!(get_neighbor(5, Top, (2,2,2)), None);
pub fn get_neighbor(voxel: usize, face: Face, dims: Dimensions) -> Option<usize> {
    let a = three_d_cords(voxel, dims);
    match face {
        Face::Top if a.1 + 1 < dims.1 => Some(one_d_cords([a.0, a.1 + 1, a.2], dims)),
        Face::Bottom if a.1 > 0 => Some(one_d_cords([a.0, a.1 - 1, a.2], dims)),
        Face::Back if a.2 + 1 < dims.2 => Some(one_d_cords([a.0, a.1, a.2 + 1], dims)),
        Face::Forward if a.2 > 0 => Some(one_d_cords([a.0, a.1, a.2 - 1], dims)),
        Face::Right if a.0 + 1 < dims.0 => Some(one_d_cords([a.0 + 1, a.1, a.2], dims)),
        Face::Left if a.0 > 0 => Some(one_d_cords([a.0 - 1, a.1, a.2], dims)),
        _ => None,
    }
}

pub fn get_neigbhors_from_across_chunks(dims: Dimensions, index: usize) -> Vec<(Face, usize)> {
    let edges = is_block_on_edge(dims, index);
    if edges.is_empty() {
        return vec![];
    }
    edges
        .iter()
        .filter_map(|f| {
            if (*f as usize) < 2 {
                None
            } else {
                Some((*f, get_neigbhor_across_chunk(dims, index, *f)))
            }
        })
        .collect()
}
