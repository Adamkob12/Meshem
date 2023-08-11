//! This file contains the function itself, and some added utilities and defs.
use super::{Dimensions, Neighbors, VoxelRegistry};
use crate::util::vav::*;
use bevy::prelude::*;
use bevy::render::mesh::{
    Indices, MeshVertexAttribute, MeshVertexAttributeId, VertexAttributeValues,
};
use bevy::render::render_resource::PrimitiveTopology;
use bevy::utils::hashbrown::HashMap;

#[derive(Debug, Clone)]
pub enum MeshingAlgorithm {
    Stupid,
    Culling,
}

/// Arguments:
/// - [`dims`](Dimensions): the dimensions of the grid, (width, len, height). (eg: (16, 16, 256))
/// - [`grid`](Vec<T>): one dimentional array of voxels, to turn into a single mesh, the function
///     assumes the real grid is 3 dimentional, and that the width, height and length math the
///     dimensions given with the dims argument.
/// - [`reg`](VoxelRegistry): this is a data structure that will return the desired mesh attribute
///     we need, but(!) the size of each of the voxels MUST be the same across the entire grid.
///     if this condition is not met, the grid will not be properly meshified.
///     An example to create a [`VoxelRegistry`] is in the examples folder.
/// - ['ma'](MeshingAlgorithm): The meshing algorithm to use - currently supports Culling and
///     Stupid. (Culling is always better than Stupid)
/// Return:
/// - [`Some(mesh)`](Mesh): the mesh
/// - [`None`]: couldn't create mesh
pub fn meshem<T>(
    dims: Dimensions,
    grid: Vec<T>,
    reg: &impl VoxelRegistry<Voxel = T>,
    ma: MeshingAlgorithm,
) -> Option<Mesh> {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let ch_len = grid.len();
    assert_eq!(
        ch_len,
        dims.0 * dims.1 * dims.2,
        "The product of the value of each of the dimensions must be the length of 
        the one dimentional grid array."
    );

    let width = dims.0;
    let length = dims.1;
    let height = dims.2;
    let t = width * length * height;

    let mut indices: Vec<u32> = vec![];
    let mut vertices: Vec<(MeshVertexAttribute, VertexAttributeValues)> = vec![];
    for att in reg.all_attributes().iter() {
        vertices.push((att.clone(), VertexAttributeValues::new(att.format.clone())));
    }

    for i in 0..width {
        for j in 0..length {
            for k in 0..height {
                let cord = k * length * width + j * width + i;
                let above = cord + length * width;
                let below = cord.checked_sub(width * length).unwrap_or(usize::MAX);
                let right = cord + 1;
                let left = cord.checked_sub(1).unwrap_or(usize::MAX);
                let back = cord + width;
                let forward = cord.checked_sub(width).unwrap_or(usize::MAX);
                let mut neig = [false; 6];
                let center = reg.get_center();
                let position_offset = (i, k, j);

                if in_range(k + 1, 0, height) {
                    neig[0] = !reg.is_voxel(&grid[above]);
                } else {
                    neig[0] = true;
                }
                if in_range(k, 1, t) {
                    neig[1] = !reg.is_voxel(&grid[below]);
                } else {
                    neig[1] = true;
                }
                if in_range(i + 1, 0, width) {
                    neig[2] = !reg.is_voxel(&grid[right]);
                } else {
                    neig[2] = true;
                }
                if in_range(i, 1, t) {
                    neig[3] = !reg.is_voxel(&grid[left]);
                } else {
                    neig[3] = true;
                }
                if in_range(j + 1, 0, length) {
                    neig[4] = !reg.is_voxel(&grid[back]);
                } else {
                    neig[4] = true;
                }
                if in_range(j, 1, t) {
                    neig[5] = !reg.is_voxel(&grid[forward]);
                } else {
                    neig[5] = true;
                }

                match ma {
                    MeshingAlgorithm::Stupid => neig = [true; 6],
                    MeshingAlgorithm::Culling => {}
                }

                if neig == [false, false, false, false, false, false] {
                    continue;
                }
                if in_range(cord, 0, t) {
                    if let Some(v_mesh) = reg.get_mesh(&grid[cord]) {
                        add_vertices(
                            neig,
                            &mut indices,
                            &mut vertices,
                            v_mesh,
                            center,
                            position_offset,
                        );
                    }
                }
            }
        }
    }

    for (att, vals) in vertices {
        mesh.insert_attribute(att, vals);
    }
    mesh.set_indices(Some(Indices::U32(indices)));

    Some(mesh)
}

/// Important helper function to add the vertices and indices of each voxel into the running count of vertices
/// and indices, preserving their attributes, and (important!) assigning a custom offset to the
/// position attributes, we are assuming this is only needed for the position attributes (because
/// it usually is).
fn add_vertices(
    neig: Neighbors,
    indices_main: &mut Vec<u32>,
    vertices: &mut Vec<(MeshVertexAttribute, VertexAttributeValues)>,
    voxel: &Mesh,
    center: [f32; 3],
    position_offset: (usize, usize, usize),
) {
    let vertices_count = vertices[0].1.len();
    let pos_attribute = voxel
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .expect("couldn't get voxel mesh data");
    let VertexAttributeValues::Float32x3(positions) = pos_attribute else {
        panic!("Unexpected vertex format, expected Float32x3.");
    };
    let Indices::U32(indices) = voxel.indices()
        .expect("couldn't get indices data") else {
        panic!("Expected U32 indices format");
    };
    let triangles = indices
        .chunks(3)
        .map(|chunk| (chunk[0], chunk[1], chunk[2]));

    let mut indices_to_save: Vec<u32> = vec![];
    let mut vertices_to_save: Vec<(bool, u32)> = vec![(false, 0); positions.len()];

    for (a, b, c) in triangles {
        let v1 = positions[a as usize];
        let v2 = positions[b as usize];
        let v3 = positions[c as usize];
        let mut save = false;

        for i in 0..3 {
            if v1[i] == v2[i] && v2[i] == v3[i] && v1[i] == v3[i] {
                match (i, center[i] > v1[i]) {
                    (0, true) if neig[3] => save = true,
                    (0, false) if neig[2] => save = true,
                    (1, true) if neig[1] => save = true,
                    (1, false) if neig[0] => save = true,
                    (2, true) if neig[5] => save = true,
                    (2, false) if neig[4] => save = true,
                    _ => save = false,
                }
            }
        }

        if save {
            indices_to_save.push(a);
            indices_to_save.push(b);
            indices_to_save.push(c);
            vertices_to_save[a as usize].0 = true;
            vertices_to_save[b as usize].0 = true;
            vertices_to_save[c as usize].0 = true;
        }
    }

    let mut offset = 0;
    for (b, i) in vertices_to_save.iter_mut() {
        *i = offset;
        if !*b {
            offset += 1;
        }
    }

    for i in indices_to_save.iter_mut() {
        *i -= vertices_to_save[*i as usize].1;
        *i += vertices_count as u32;
    }

    for (id, vals) in voxel.attributes() {
        let mut i = 0;
        i = {
            while id != vertices[i].0.id {
                i += 1;
            }
            i
        };
        if id == Mesh::ATTRIBUTE_POSITION.id {
            vertices[i].1.extend(
                &vals
                    .filter_bool_array(vertices_to_save.iter().map(|(b, x)| *b).collect())
                    .offset_all(position_offset),
            );
        } else {
            vertices[i].1.extend(
                &vals.filter_bool_array(vertices_to_save.iter().map(|(b, x)| *b).collect()),
            );
        }
    }
    indices_main.extend(indices_to_save);
}
