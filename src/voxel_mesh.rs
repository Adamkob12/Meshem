//! A module containing the "default block", it is used in the examples,
//! it is simple and easy to work with.
use crate::prelude::*;
use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;

/// Function that generates the mesh of a voxel.
pub fn generate_voxel_mesh(
    voxel_dims: [f32; 3],
    texture_atlas_dims: [usize; 2],
    texture: [(Face, [usize; 2]); 6],
) -> Mesh {
    let mut cube_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let y = voxel_dims[1] / 2.0;
    let x = voxel_dims[0] / 2.0;
    let z = voxel_dims[2] / 2.0;

    let u: f32 = 1.0 / (texture_atlas_dims[0] as f32);
    let v: f32 = 1.0 / (texture_atlas_dims[1] as f32);

    let mut uvs: [[f32; 2]; 6] = [[0.0, 0.0]; 6];
    texture
        .iter()
        .for_each(|(f, [a, b])| uvs[*f as usize] = [*a as f32 * u, *b as f32 * v]);

    #[rustfmt::skip]
    cube_mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        // Each array is an [x, y, z] coordinate in local space.
        // Meshes always rotate around their local [0, 0, 0] when a rotation is applied to their Transform.
        // By centering our mesh around the origin, rotating the mesh preserves its center of mass.
        vec![
            // top (facing towards +y)
            [-x,y, z],
            [x,y, z],
            [x,y, -z],
            [-x,y, -z],
            // bottom   (-y)
            [-x,-y, -z],
            [x,-y, -z],
            [x,-y, z],
            [-x,-y, z],
            // right    (+x)
            [x,y, -z],
            [x,y, z],
            [x,-y, z],
            [x,-y, -z],
            // left     (-x)
            [-x,-y, -z],
            [-x,-y, z],
            [-x,y, z],
            [-x,y, -z],
            // back     (+z)
            [x,y, z],
            [-x,y, z],
            [-x,-y, z],
            [x,-y, z],
            // forward  (-z)
            [x,-y, -z],
            [-x,-y, -z],
            [-x,y, -z],
            [x,y, -z],
        ],
    );

    #[rustfmt::skip]
    cube_mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            // Assigning the UV coords for the top side.
            uvs[0], [uvs[0][0] + u, uvs[0][1]], [uvs[0][0] + u, uvs[0][1] + v], [uvs[0][0], uvs[0][1] + v],           
            // Assigning the UV coords for the bottom side
            uvs[1], [uvs[1][0] + u, uvs[1][1]], [uvs[1][0] + u, uvs[1][1] + v], [uvs[1][0], uvs[1][1] + v],           
            // Assigning the UV coords for the right side.
            uvs[2], [uvs[2][0] + u, uvs[2][1]], [uvs[2][0] + u, uvs[2][1] + v], [uvs[2][0], uvs[2][1] + v],           
            // Assigning the UV coords for the left side.
            uvs[3], [uvs[3][0] + u, uvs[3][1]], [uvs[3][0] + u, uvs[3][1] + v], [uvs[3][0], uvs[3][1] + v],           
            // Assigning the UV coords for the back side.
            uvs[4], [uvs[5][0] + u, uvs[4][1]], [uvs[4][0] + u, uvs[4][1] + v], [uvs[4][0], uvs[4][1] + v],           
            // Assigning the UV coords for the forward side.
            uvs[5], [uvs[5][0] + u, uvs[5][1]], [uvs[5][0] + u, uvs[5][1] + v], [uvs[5][0], uvs[5][1] + v],           
        ],
    );

    // For meshes with flat shading, normals are orthogonal (pointing out) from the direction of
    // the surface.
    // Normals are required for correct lighting calculations.
    // Each array represents a normalized vector, which length should be equal to 1.0.
    #[rustfmt::skip]
    cube_mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            // Normals for the top side (towards +y)
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            // Normals for the bottom side (towards -y)
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            // Normals for the right side (towards +x)
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            // Normals for the left side (towards -x)
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            // Normals for the back side (towards +z)
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            // Normals for the forward side (towards -z)
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
        ],
    );

    // Create the triangles out of the 24 vertices we created.
    // To construct a square, we need 2 triangles, therefore 12 triangles in total.
    // To construct a triangle, we need the indices of its 3 defined vertices, adding them one
    // by one, in a counter-clockwise order (relative to the position of the viewer, the order
    // should appear counter-clockwise from the front of the triangle, in this case from outside the cube).
    // Read more about how to correctly build a mesh manually in the Bevy documentation of a Mesh,
    // further examples and the implementation of the built-in shapes.
    #[rustfmt::skip]
    cube_mesh.set_indices(Some(Indices::U32(vec![
        0,1,3 , 2,3,1, // triangles making up the top (+y) facing side.
        4,5,7 , 6,7,5, // bottom (-y)
        8,9,11 , 10,11,9, // right (+x)
        12,13,15 , 14,15,13, // left (-x)
        16,17,19 , 18,19,17, // back (+z)
        20,21,23 , 22,23,21, // forward (-z)
    ])));

    cube_mesh
}
