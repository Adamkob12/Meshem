use crate::prelude::*;
use bevy::prelude::*;
use bevy::render::mesh::{
    Indices, Mesh, MeshVertexAttribute, MeshVertexAttributeId, VertexAttributeValues,
};
use bevy::render::render_resource::PrimitiveTopology;

/// Funtion to normalize voxel mesh. Currently (0.2.0) bevy_meshem is will only work if the voxel
/// meshes meet a set of predefined requirments. These requirments are necessary for the
/// update_mesh function to perform its operation at O(1) time.
///
/// `center` is the 3d center of the voxel, so the halfway of each of the axis. e.g the center of
/// the default block is [0,0,0]
/// -----------------------------------------------------------------------
/// The default block in default_block.rs is already normalized, so it can be used as a template
/// for a normalized, valid and compatible mesh.
///
/// The requirments mentioned above are:
///     - Sorted indices: every two triangles that make up a quad must appear next to each other in
///       the `Indices`.
///     - Sorted vertices (by face): The vertices must appear sorted, the first 4 vertices
///       represent the quad of the `Top` face of the voxel, the second 4 vertices represent the
///       `Bottom` face of the voxel, and so on.
///     - Sorted vertices (by position): The vertices must be sorted by their physical position in
///       3d space, i.e: [Top left, Top right, Bottom right, Bottom left], 'Top' is relative to the
///       POV, look at the default block for an example.
///     - Standard indices order: [(Top Left, Top Right, Bottom Left), (BR, BL, TR)].
pub fn normalize_voxel_mesh(mesh: &mut Mesh, center: [f32; 3]) {
    assert_eq!(
        mesh.count_vertices(),
        24,
        "Expected 24 vertices in a voxel mesh to normalize. 4 for each face of the voxel."
    );
    assert_eq!(
        mesh.indices()
            .expect("Could not retrieve indices from mesh")
            .len(),
        36,
        "Expected 12 triangles or indices length of 36, 2 triangles for each face of the voxel."
    );

    let pos_attribute = mesh
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .expect("couldn't get voxel mesh data");
    let VertexAttributeValues::Float32x3(positions) = pos_attribute else {
        panic!("Unexpected vertex format for position attribute, expected Float32x3.");
    };
    let Indices::U32(indices) = mesh.indices()
        .expect("couldn't get indices data") else {
        panic!("Expected U32 indices format");
    };
    let triangles = indices
        .chunks(3)
        .map(|chunk| (chunk[0], chunk[1], chunk[2]));

    let mut quads = [[0; 4]; 6];

    dbg!(&positions);

    // Assign each vertex to its corresponding quad
    for (a, b, c) in triangles {
        let v1 = positions[a as usize];
        let v2 = positions[b as usize];
        let v3 = positions[c as usize];

        // see which side of the voxel the triangle belongs to
        for i in 0..3 {
            if v1[i] == v2[i] && v2[i] == v3[i] && v1[i] == v3[i] {
                match (i, center[i] > v1[i]) {
                    (0, true) => insert_if_not_in(&mut quads[3], [a, b, c]),
                    (0, false) => insert_if_not_in(&mut quads[2], [a, b, c]),
                    (1, true) => insert_if_not_in(&mut quads[1], [a, b, c]),
                    (1, false) => insert_if_not_in(&mut quads[0], [a, b, c]),
                    (2, true) => insert_if_not_in(&mut quads[5], [a, b, c]),
                    (2, false) => insert_if_not_in(&mut quads[4], [a, b, c]),
                    _ => {}
                }
                break;
            }
        }
    }
    quads.iter_mut().for_each(|x| x.sort());

    // Sort
    for (i, quad) in quads.iter_mut().enumerate() {
        let top = match i {
            0 => (2, 1),
            1 => (2, -1),
            2 => (1, 1),
            3 => (1, -1),
            4 => (1, 1),
            5 => (1, -1),
            _ => break,
        };
        #[allow(non_snake_case)]
        let (_, B): (f32, f32) = {
            let mut max = f32::MIN;
            let mut min = f32::MAX;
            for &i in quad.iter() {
                if positions[i as usize][top.0] > max {
                    max = positions[i as usize][top.0];
                }
                if positions[i as usize][top.0] < min {
                    min = positions[i as usize][top.0];
                }
            }
            if top.1 > 0 {
                (max, min)
            } else {
                (min, max)
            }
        };

        let left = match i {
            0 => (0, -1),
            1 => (0, 1),
            2 => (2, -1),
            3 => (2, 1),
            4 => (0, 1),
            5 => (0, -1),
            _ => break,
        };

        for k in 0..4 {
            for j in k..4 {
                let k_top = positions[quad[k] as usize][top.0] * (top.1 as f32);
                let j_top = positions[quad[j] as usize][top.0] * (top.1 as f32);
                let k_left = positions[quad[k] as usize][left.0] * (left.1 as f32);
                let j_left = positions[quad[j] as usize][left.0] * (left.1 as f32);

                if k_top < j_top {
                    quad.swap(k, j);
                    println!("{k}, {j}, 0");
                    dbg!(k_top);
                    dbg!(j_top);
                    dbg!(k_left);
                    dbg!(j_left);
                    dbg!(top);
                    dbg!(left);
                } else if k_top == j_top {
                    if k_left < j_left && k_top != B {
                        quad.swap(k, j);
                        println!("{k}, {j}, 1");
                        dbg!(k_top);
                        dbg!(j_top);
                        dbg!(k_left);
                        dbg!(j_left);
                        dbg!(top);
                        dbg!(left);
                    }
                }
            }
        }
    }
    dbg!(quads);

    //
    let mut final_vertices = vec![];
    quads
        .iter()
        .for_each(|x| final_vertices.extend_from_slice(x));

    for (id, vals) in mesh.attributes_mut() {
        *vals = vals.get_needed(&final_vertices);
        if id == Mesh::ATTRIBUTE_POSITION.id {
            // dbg!(vals);
        }
    }

    mesh.set_indices(Some(Indices::U32(vec![
        0, 1, 3, 2, 3, 1, // triangles making up the top (+y) facing side.
        4, 5, 7, 6, 7, 5, // bottom (-y)
        8, 9, 11, 10, 11, 9, // right (+x)
        12, 13, 15, 14, 15, 13, // left (-x)
        16, 17, 19, 18, 19, 17, // back (+z)
        20, 21, 23, 22, 23, 21, // forward (-z)
    ])));
}

fn insert_if_not_in(arr: &mut [u32; 4], vals: [u32; 3]) {
    for val in vals {
        if arr[0] != val && arr[1] != val && arr[2] != val && arr[3] != val {
            for j in arr.iter_mut() {
                if *j == 0 {
                    *j = val;
                    break;
                }
            }
        }
    }
}
