//! This module contains the main functions themself, and some added utilities and defs.
use super::dynamic_mesh::*;
use super::{Dimensions, Neighbors, VoxelRegistry};
use crate::util::vav::*;
use crate::{face_to_u32, Face, Face::*};
use bevy::prelude::*;
use bevy::render::mesh::{
    Indices, MeshVertexAttribute, MeshVertexAttributeId, VertexAttributeValues,
};
use bevy::render::render_resource::PrimitiveTopology;

/// All the variants for the Meshing algorithm.
#[derive(Debug, Clone)]
pub enum MeshingAlgorithm {
    Naive,
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
///     Naive. (Culling is always better than Naive)
///
/// Return:
/// - [`Some(mesh)`](Mesh): the mesh
/// - [`None`]: couldn't create mesh
pub fn mesh_grid<T>(
    dims: Dimensions,
    grid: Vec<T>,
    reg: &impl VoxelRegistry<Voxel = T>,
    ma: MeshingAlgorithm,
) -> Option<(Mesh, MeshMD<T>)> {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let ch_len = grid.len();
    let mut vivi = vec![vec![]; ch_len];
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
    for att in reg.all_attributes().iter() {
        mesh.insert_attribute(att.clone(), VertexAttributeValues::new(att.format));
    }
    let mut vertices: Vec<(MeshVertexAttributeId, &mut VertexAttributeValues)> =
        mesh.attributes_mut().collect();

    for k in 0..height {
        for j in 0..length {
            for i in 0..width {
                let cord = k * length * width + j * width + i;
                // print!("{} , ", cord);
                let above = cord + length * width;
                let below = cord.checked_sub(width * length).unwrap_or(usize::MAX);
                let right = cord + 1;
                let left = cord.checked_sub(1).unwrap_or(usize::MAX);
                let back = cord + width;
                let forward = cord.checked_sub(width).unwrap_or(usize::MAX);
                let mut neig = [false; 6];
                let center = reg.get_center();
                let voxel_dims = reg.get_voxel_dimensions();
                let position_offset = (
                    i as f32 * voxel_dims[0],
                    k as f32 * voxel_dims[1],
                    j as f32 * voxel_dims[0],
                );

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
                    MeshingAlgorithm::Naive => neig = [true; 6],
                    MeshingAlgorithm::Culling => {}
                }

                if neig == [false, false, false, false, false, false] {
                    continue;
                }
                if in_range(cord, 0, t) {
                    if let Some(v_mesh) = reg.get_mesh(&grid[cord]) {
                        // add_vertices() is a private function that adds the vertices and
                        // indices to the running count of vertices and indices.
                        add_voxel(
                            neig,
                            &mut indices,
                            &mut vertices,
                            v_mesh,
                            &mut vivi,
                            cord,
                            center,
                            position_offset,
                        );
                    }
                }
            }
        }
    }

    mesh.set_indices(Some(Indices::U32(indices)));
    dbg!(&vivi);
    let d_mesh = MeshMD {
        dims,
        vivi,
        changed_voxels: vec![],
    };

    Some((mesh, d_mesh))
}

/// Important helper function to add the vertices and indices of each voxel into the running count of vertices
/// and indices, preserving their attributes, and (important!) assigning a custom offset to the
/// position attributes, we are assuming this is only needed for the position attributes (because
/// it usually is).
fn add_voxel(
    neig: Neighbors,
    indices_main: &mut Vec<u32>,
    vertices: &mut Vec<(MeshVertexAttributeId, &mut VertexAttributeValues)>,
    voxel: &Mesh,
    vivi: &mut VIVI,
    voxel_index: usize,
    center: [f32; 3],
    position_offset: (f32, f32, f32),
) {
    let vertices_count = vertices[0].1.len();
    let pos_attribute = voxel
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .expect("couldn't get voxel mesh data");
    let VertexAttributeValues::Float32x3(positions) = pos_attribute else {
        panic!("Unexpected vertex format for position attribute, expected Float32x3.");
    };
    let Indices::U32(indices) = voxel.indices()
        .expect("couldn't get indices data") else {
        panic!("Expected U32 indices format");
    };
    let triangles = indices
        .chunks(3)
        .map(|chunk| (chunk[0], chunk[1], chunk[2]));

    // define the indices and vertices we want to save of the voxel mesh
    let mut indices_to_save: Vec<u32> = vec![];
    // helper data structure
    let mut vertices_to_save: Vec<(bool, u32, Face)> = vec![(false, 0, Face::Top); positions.len()];
    // sorted vertices by the quad they are in
    let mut sorted_vertices: Vec<Option<Vec<u32>>> = vec![None; 6];
    // the final array of the vertices, it will be sorted, each 4 vertices will be a
    // part of one quad, we sort them this way to efficiently update the vivi.
    let mut final_vertices: Vec<u32> = vec![];

    // iterate over all the triangles in the mesh
    for (a, b, c) in triangles {
        let v1 = positions[a as usize];
        let v2 = positions[b as usize];
        let v3 = positions[c as usize];
        let mut save = (false, Top);

        // see which side of the voxel the triangle belongs to
        for i in 0..3 {
            if v1[i] == v2[i] && v2[i] == v3[i] && v1[i] == v3[i] {
                match (i, center[i] > v1[i]) {
                    (0, true) if neig[3] => save = (true, Left),
                    (0, false) if neig[2] => save = (true, Right),
                    (1, true) if neig[1] => save = (true, Bottom),
                    (1, false) if neig[0] => save = (true, Top),
                    (2, true) if neig[5] => save = (true, Forward),
                    (2, false) if neig[4] => save = (true, Back),
                    _ => save = (false, Top),
                }
                break;
            }
        }

        // save the vertices
        if save.0 {
            let quad: usize = save.1.into();
            indices_to_save.push(a);
            indices_to_save.push(b);
            indices_to_save.push(c);
            match sorted_vertices[quad] {
                None => {
                    sorted_vertices[quad] = Some(vec![a, b, c]);
                    vertices_to_save[a as usize].0 = true;
                    vertices_to_save[b as usize].0 = true;
                    vertices_to_save[c as usize].0 = true;
                    vertices_to_save[a as usize].1 = 0;
                    vertices_to_save[b as usize].1 = 1;
                    vertices_to_save[c as usize].1 = 2;
                    vertices_to_save[a as usize].2 = save.1;
                    vertices_to_save[b as usize].2 = save.1;
                    vertices_to_save[c as usize].2 = save.1;
                }
                Some(ref mut v) => {
                    for &i in [a, b, c].iter() {
                        if !vertices_to_save[i as usize].0 {
                            v.push(i);
                            vertices_to_save[i as usize].2 = save.1;
                            vertices_to_save[i as usize].1 = v.len() as u32 - 1;
                            vertices_to_save[i as usize].0 = true;
                        }
                    }
                }
            }
        }
    }

    // The code from now on is a little messy, but it is very simple in actuality. It is mostly
    // just offseting the vertices and indices and formatting them into the right data-structres.

    // offset the vertices, since we won't be using all the vertices of the the mesh,
    // we need to find out which of them we will be using first, and then filter out
    // the ones we dont need.
    let mut offset: u32 = 0;
    for q in sorted_vertices.iter() {
        match q {
            None => offset += 4,
            Some(ref v) => {
                let mut only_first = true;
                for &i in v.iter() {
                    let face = vertices_to_save[i as usize].2;
                    vertices_to_save[i as usize].1 += face as u32 * 4 - offset;
                    final_vertices.push(i);
                    // update the vivi
                    if only_first {
                        vivi[voxel_index].push((i + vertices_count as u32) | face_to_u32(face));
                        only_first = false;
                    }
                }
            }
        }
    }

    // offset the indices, we need to consider the fact that the indices wil be part of a big mesh,
    // with a lot of vertices, so we must the vertices to a running count and offset them accordingly.
    for i in indices_to_save.iter_mut() {
        *i = vertices_to_save[*i as usize].1 + vertices_count as u32;
    }

    for (id, vals) in vertices.iter_mut() {
        let mut att = voxel
            .attribute(*id)
            .expect(format!("Couldn't retrieve voxel mesh attribute {:?}.", id).as_str())
            .get_needed(&final_vertices);
        if *id == Mesh::ATTRIBUTE_POSITION.id {
            att = att.offset_all(position_offset);
        }
        vals.extend(&att);
    }
    indices_main.extend(indices_to_save);
}
