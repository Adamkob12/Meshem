// use crate::pbs::*;
use crate::prelude::*;
use bevy::mesh::{Indices, VertexAttributeValues};

/// The function updates the mesh according to the change log in the mesh meta data.
pub fn update_mesh<T: std::fmt::Debug>(
    mesh: &mut Mesh,
    metadata: &mut MeshMD<T>,
    reg: &impl VoxelRegistry<Voxel = T>,
) {
    let mut min = usize::MAX;
    let mut max = usize::MIN;
    let voxel_dims = reg.get_voxel_dimensions();
    for (voxel, index, change, neighbors) in metadata.changed_voxels.iter() {
        if *index < min {
            min = *index;
        }
        if *index > max {
            max = *index;
        }
        let temp = three_d_cords(*index, metadata.dims);
        let position_offset = (
            temp.0 as f32 * voxel_dims[0],
            temp.1 as f32 * voxel_dims[1],
            temp.2 as f32 * voxel_dims[2],
        );
        let neig: Neighbors = match change {
            VoxelChange::AddFaces => neighbors
                .iter()
                .map(|x| x.is_some())
                .collect::<Vec<bool>>()
                .try_into()
                .unwrap(),
            _ => {
                let mut n = [false; 6];
                for (i, j) in neighbors.iter().enumerate() {
                    match j {
                        None => n[i] = true,
                        Some(t) if !reg.is_covering(&t, Face::from(i).opposite()) => n[i] = true,
                        Some(_) => {}
                    }
                }
                n
            }
        };

        let covering: Neighbors = {
            let mut n = [false; 6];
            for i in 0..6 {
                n[i] = reg.is_covering(voxel, Face::from(i));
            }
            n
        };

        let neighboring_voxels: Vec<(Face, &Mesh)> = {
            let mut r: Vec<(Face, &Mesh)> = vec![];
            for (i, j) in neighbors.iter().enumerate() {
                match j {
                    None => continue,
                    Some(t)
                        if /* reg.is_covering(&t, Face::from(i).opposite()) && */
                            reg.is_covering(voxel, Face::from(i)) =>
                    {
                        if let VoxelMesh::NormalCube(mesh) = reg.get_mesh(&t) {
                            r.push((
                                Face::from(i),
                                mesh,
                            ));
                        }
                    }
                    _ => continue,
                }
            }
            r
        };

        match *change {
            VoxelChange::Added => {
                if let VoxelMesh::NormalCube(voxel_mesh) = reg.get_mesh(voxel) {
                    remove_voxel(mesh, &mut metadata.vivi, *index, [true; 6]);
                    add_voxel_after_gen(
                        neig,
                        mesh,
                        voxel_mesh,
                        &mut metadata.vivi,
                        *index,
                        reg.get_center(),
                        position_offset,
                    );
                    remove_quads_facing(mesh, &mut metadata.vivi, *index, metadata.dims, covering);
                }
            }
            VoxelChange::Broken => {
                remove_voxel(mesh, &mut metadata.vivi, *index, [true; 6]);
                add_quads_facing(
                    mesh,
                    &mut metadata.vivi,
                    *index,
                    neighboring_voxels,
                    reg.get_center(),
                    reg.get_voxel_dimensions(),
                    metadata.dims,
                );
            }
            VoxelChange::CullFaces => {
                remove_voxel(
                    mesh,
                    &mut metadata.vivi,
                    *index,
                    neighbors
                        .iter()
                        .map(|x| x.is_some())
                        .collect::<Vec<bool>>()
                        .try_into()
                        .unwrap(),
                );
            }
            VoxelChange::AddFaces => {
                if let VoxelMesh::NormalCube(voxel_mesh) = reg.get_mesh(voxel) {
                    add_voxel_after_gen(
                        neig,
                        mesh,
                        voxel_mesh,
                        &mut metadata.vivi,
                        *index,
                        reg.get_center(),
                        position_offset,
                    );
                }
            }
        }
    }

    metadata.changed_voxels.clear();
    // if metadata.pbs.is_some() {
    //     apply_pbs(
    //         mesh,
    //         &metadata.vivi,
    //         metadata.dims,
    //         min.checked_sub(metadata.dims.0 * metadata.dims.2 * 2)
    //             .unwrap_or(0),
    //         max.checked_add(metadata.dims.0 * metadata.dims.2 * 2)
    //             .unwrap_or(usize::MAX),
    //         metadata.pbs.unwrap(),
    //         reg.get_voxel_dimensions(),
    //     );
    // }
}

// The function removes all quads facing a voxel.
fn remove_quads_facing(
    mesh: &mut Mesh,
    vivi: &mut VIVI,
    voxel_index: usize,
    dims: Dimensions,
    covering: Neighbors,
) {
    let mut quad_to_remove: Neighbors;
    for i in 0..6 {
        let face = Face::from(i as usize);
        let n = match get_neighbor(voxel_index, face, dims) {
            None => continue,
            Some(i) => i,
        };
        quad_to_remove = [false; 6];
        quad_to_remove[face.opposite() as usize] = true;
        if covering[face.opposite() as usize] {
            remove_voxel(mesh, vivi, n, quad_to_remove);
        }
    }
}

/// Function removes voxel from the big mesh.
fn remove_voxel(mesh: &mut Mesh, vivi: &mut VIVI, voxel_index: usize, neig: Neighbors) {
    for (i, b) in neig.iter().enumerate() {
        if !b {
            continue;
        }
        let face = Face::from(i);
        let quad = match vivi.get_quad_index(face, voxel_index) {
            None => continue,
            Some(i) => i,
        } as usize;
        if quad + 25 >= mesh.count_vertices() {
            for (_, vals) in mesh.attributes_mut() {
                vals.remove(quad + 3);
                vals.remove(quad + 2);
                vals.remove(quad + 1);
                vals.remove(quad + 0);
            }
            vivi.remove_quad(quad);
            let mut tmp = quad;
            while tmp != mesh.count_vertices() {
                vivi.change_quad_index(tmp + 4, tmp);
                tmp += 4;
            }
        } else {
            for (_, vals) in mesh.attributes_mut() {
                vals.swap_remove(quad + 3);
                vals.swap_remove(quad + 2);
                vals.swap_remove(quad + 1);
                vals.swap_remove(quad + 0);
            }
            let ver_count = mesh.count_vertices();
            vivi.remove_quad(quad);
            vivi.change_quad_index(ver_count, quad);
        }

        let Indices::U32(indices) = mesh.indices_mut().expect("couldn't get indices data") else {
            panic!("Expected U32 indices format");
        };
        for _ in 0..6 {
            indices.pop();
        }
    }
}

/// Function adds quads facing voxel.
pub(crate) fn add_quads_facing(
    mesh: &mut Mesh,
    vivi: &mut VIVI,
    voxel_index: usize,
    neighboring_voxels: Vec<(Face, &Mesh)>,
    center: [f32; 3],
    voxel_dims: [f32; 3],
    dims: Dimensions,
) {
    let mut neig: Neighbors;
    for &(face, vmesh) in neighboring_voxels.iter() {
        neig = [false; 6];
        neig[face.opposite() as usize] = true;
        let i = match get_neighbor(voxel_index, face, dims) {
            None => continue,
            Some(j) => j,
        };
        let temp = three_d_cords(i, dims);
        let position_offset = (
            temp.0 as f32 * voxel_dims[0],
            temp.1 as f32 * voxel_dims[1],
            temp.2 as f32 * voxel_dims[2],
        );
        add_voxel_after_gen(neig, mesh, vmesh, vivi, i, center, position_offset)
    }
}

/// Function adds a voxel after the big mesh has already been generated.
fn add_voxel_after_gen(
    neig: Neighbors,
    main_mesh: &mut Mesh,
    voxel: &Mesh,
    vivi: &mut VIVI,
    voxel_index: usize,
    center: [f32; 3],
    position_offset: (f32, f32, f32),
) {
    // Make sure we are not adding quads that already exist
    let mut neig = neig;
    for (i, b) in neig.iter_mut().enumerate() {
        let face = Face::from(i);
        if *b && vivi.get_quad_index(face, voxel_index).is_some() {
            *b = false;
        }
    }
    let vertices_count = main_mesh.count_vertices();
    let Indices::U32(ref mut indices_main) =
        main_mesh.indices_mut().expect("Couldn't get indices data")
    else {
        panic!("Indices format should be U32");
    };

    let pos_attribute = voxel
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .expect("couldn't get voxel mesh data");
    let VertexAttributeValues::Float32x3(positions) = pos_attribute else {
        panic!("Unexpected vertex format for position attribute, expected Float32x3.");
    };
    let Indices::U32(indices) = voxel.indices().expect("couldn't get indices data") else {
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
    // just offsetting the vertices and indices and formatting them into the right data-structures.

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
                        vivi.insert(face, voxel_index, i + vertices_count as u32 - offset);
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
    indices_main.extend(indices_to_save);

    for (attr, vals) in main_mesh.attributes_mut() {
        let mut att = voxel
            .attribute(attr.id)
            .expect(format!("Couldn't retrieve voxel mesh attribute {:?}.", attr.id).as_str())
            .get_needed(&final_vertices);
        if attr.id == Mesh::ATTRIBUTE_POSITION.id {
            att = att.offset_all(position_offset);
        }
        vals.extend(&att);
    }
}
