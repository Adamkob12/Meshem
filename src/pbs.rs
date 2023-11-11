//! This module is responsible for Smooth Lighting. Smooth Lighting is a technique often used in
/// voxel based games that resembles Ambient Occlusion, but it is static- which means the
/// shadows are computed only once, when the mesh is generated (or updated).
use crate::prelude::*;
use bevy::math::Vec3;
use bevy::render::mesh::{Mesh, VertexAttributeValues};
use std::sync::{Arc, RwLock};

#[derive(Copy, Clone)]
/// Parameters for Smooth Lighting
pub struct SmoothLightingParameters {
    /// How intense the shadow is. 0.0 - 1.0
    pub intensity: f32,
    /// The max intensity value of the shadow (0.0 - 1.0) 1.0 is very dark, and depending on
    /// your intensity it may reach that level. Recommended: 0.6 - 0.8
    pub max: f32,
    /// Smoothing will often lower the overall intensity of the shadowing, but in return
    /// the scene will look more uniform. Recommended: 1.0 - 2.0
    pub smoothing: f32,
    /// True => Apply automatically after generating.
    /// False => The user will apply it manually using the smooth lighting API. (ex: `apply_smooth_lighting`)
    pub apply_at_gen: bool,
}

pub(crate) fn apply_sl_quad(
    mesh: &mut Mesh,
    vivi: &VIVI,
    index: usize,
    face: Face,
    surrounding_blocks: [bool; 3 * 3 * 3],
    slparams: SmoothLightingParameters,
    voxel_dims: [f32; 3],
    dims: Dimensions,
) {
    let quad = vivi
        .get_quad_index(face, index)
        .expect("Couldn't find quad in vivi for smooth lighting");

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

    let og: [i32; 3] = match face {
        Top => [1, 0, 1],
        Bottom => [1, 2, 1],
        Right => [0, 1, 1],
        Left => [2, 1, 1],
        Back => [1, 1, 0],
        Forward => [1, 1, 2],
    };
    let grid_dims = (3, 3, 3);
    let [ogx, ogy, ogz] = og;
    for i in 0..4 {
        let ver = i + quad;
        let diff = positions[i as usize] - voxel_center;
        let mut total: f32 = 0.0;
        let (dx, dy, dz) = (
            diff.x.signum() as i32,
            diff.y.signum() as i32,
            diff.z.signum() as i32,
        );
        let nx = (ogx + dx) as usize;
        let ny = (ogy + dy) as usize;
        let nz = (ogz + dz) as usize;

        let tmp = [nx, ny, nz];
        if surrounding_blocks[one_d_cords(tmp, grid_dims)] {
            total += 0.75;
        }
        let tmp = [nx, ny, ogz as usize];
        if surrounding_blocks[one_d_cords(tmp, grid_dims)] {
            total += 1.0;
        }
        let tmp = [nx, ogy as usize, nz];
        if surrounding_blocks[one_d_cords(tmp, grid_dims)] {
            total += 1.0;
        }
        let tmp = [ogx as usize, ny, nz];
        if surrounding_blocks[one_d_cords(tmp, grid_dims)] {
            total += 1.0;
        }

        total = total.min(2.0);
        let color = total * slparams.intensity;
        let color = (1.0 - color.min(1.0).powf(slparams.smoothing)).max(1.0 - slparams.max);
        colors[ver as usize] = [color, color, color, 1.0]
    }
}

pub fn apply_smooth_lighting<T, const N: usize>(
    reg: &impl VoxelRegistry<Voxel = T>,
    mesh: &mut Mesh,
    metadata: &MeshMD<T>,
    dims: Dimensions,
    lower_bound: usize,
    upper_bound: usize,
    this_chunk: &[T; N],
) {
    apply_smooth_lighting_with_connected_chunks(
        reg,
        mesh,
        metadata,
        dims,
        lower_bound,
        upper_bound,
        this_chunk,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );
}

pub fn apply_smooth_lighting_with_connected_chunks<'a, T>(
    reg: &impl VoxelRegistry<Voxel = T>,
    mesh: &mut Mesh,
    metadata: &MeshMD<T>,
    dims: Dimensions,
    lower_bound: usize,
    upper_bound: usize,
    this_chunk: &'a [T],
    north_chunk: Option<&'a [T]>,
    south_chunk: Option<&'a [T]>,
    east_chunk: Option<&'a [T]>,
    west_chunk: Option<&'a [T]>,
    no_east_chunk: Option<&'a [T]>,
    no_west_chunk: Option<&'a [T]>,
    so_east_chunk: Option<&'a [T]>,
    so_west_chunk: Option<&'a [T]>,
) {
    if let Some(sl) = metadata.smooth_lighting_params {
        for (index, quads) in metadata.vivi.vivi.iter().enumerate().skip(lower_bound) {
            if index > upper_bound {
                break;
            }
            for q in quads {
                let mut surrounding_blocks = [false; 3 * 3 * 3];
                let cage_dims = (3, 3, 3);
                let face = face_from_u32(q & REVERSE_OFFSET_CONST);

                if (matches!(face, Bottom) || matches!(face, Top))
                    && is_block_on_edge(dims, index, face)
                {
                    continue;
                }
                let (neighbor, chunk_dir) = {
                    if is_block_on_edge(dims, index, face) {
                        (
                            get_neigbhor_across_chunk(dims, index, face),
                            Some(crate::util::Direction::from(face)),
                        )
                    } else {
                        (get_neighbor(index, face, dims).unwrap(), None)
                    }
                };
                // if reg.is_covering(&this_chunk[neighbor], face.opposite()) { continue; }

                let og_index_in_cage: [i32; 3] = match face {
                    Top => [0, -1, 0],
                    Bottom => [0, 1, 0],
                    Right => [-1, 0, 0],
                    Left => [1, 0, 0],
                    Back => [0, 0, -1],
                    Forward => [0, 0, 1],
                };
                let [og_x, og_y, og_z] = og_index_in_cage;

                for y in -1..=1 {
                    for z in -1..=1 {
                        for x in -1..=1 {
                            if (og_x == x && og_y == y)
                                || (og_x == x && og_z == z)
                                || (og_y == y && og_z == z)
                            {
                                continue;
                            }
                            if (og_x == x && og_x != 0)
                                || (og_y == y && og_y != 0)
                                || (og_z == z && og_z != 0)
                            {
                                continue;
                            }
                            if (og_x == x + 2 && og_x != 0)
                                || (og_y == y + 2 && og_y != 0)
                                || (og_z == z + 2 && og_z != 0)
                            {
                                continue;
                            }
                            if (og_x == x - 2 && og_x != 0)
                                || (og_y == y - 2 && og_y != 0)
                                || (og_z == z - 2 && og_z != 0)
                            {
                                continue;
                            }

                            let cage_index = one_d_cords(
                                [(x + 1) as usize, (y + 1) as usize, (z + 1) as usize],
                                cage_dims,
                            );
                            let faces = [y < 0, y > 0, x < 0, x > 0, z < 0, z > 0];

                            match get_block_n_away(dims, neighbor, x, y, z) {
                                None => {
                                    continue;
                                }
                                Some((dir, neighbor_index)) => {
                                    let final_dir = crate::prelude::util::Direction::add_direction(
                                        chunk_dir, dir,
                                    );
                                    // if chunk_dir.is_some() && dir.is_some() {
                                    //     dbg!(final_dir, chunk_dir.unwrap(), dir.unwrap());
                                    // }
                                    surrounding_blocks[cage_index] = match final_dir {
                                        None => covering_multiple_faces(
                                            reg,
                                            &this_chunk[neighbor_index],
                                            faces,
                                        ),
                                        Some(North) if north_chunk.is_some() => {
                                            covering_multiple_faces(
                                                reg,
                                                &north_chunk.unwrap()[neighbor_index],
                                                faces,
                                            )
                                        }
                                        Some(South) if south_chunk.is_some() => {
                                            covering_multiple_faces(
                                                reg,
                                                &south_chunk.unwrap()[neighbor_index],
                                                faces,
                                            )
                                        }
                                        Some(East) if east_chunk.is_some() => {
                                            covering_multiple_faces(
                                                reg,
                                                &east_chunk.unwrap()[neighbor_index],
                                                faces,
                                            )
                                        }
                                        Some(West) if west_chunk.is_some() => {
                                            covering_multiple_faces(
                                                reg,
                                                &west_chunk.unwrap()[neighbor_index],
                                                faces,
                                            )
                                        }
                                        Some(NoEast) if no_east_chunk.is_some() => {
                                            covering_multiple_faces(
                                                reg,
                                                &no_east_chunk.unwrap()[neighbor_index],
                                                faces,
                                            )
                                        }
                                        Some(NoWest) if no_west_chunk.is_some() => {
                                            covering_multiple_faces(
                                                reg,
                                                &no_west_chunk.unwrap()[neighbor_index],
                                                faces,
                                            )
                                        }
                                        Some(SoEast) if so_east_chunk.is_some() => {
                                            covering_multiple_faces(
                                                reg,
                                                &so_east_chunk.unwrap()[neighbor_index],
                                                faces,
                                            )
                                        }
                                        Some(SoWest) if so_west_chunk.is_some() => {
                                            covering_multiple_faces(
                                                reg,
                                                &so_west_chunk.unwrap()[neighbor_index],
                                                faces,
                                            )
                                        }
                                        _ => false,
                                    };
                                }
                            }
                        }
                    }
                }
                if surrounding_blocks != [false; 3 * 3 * 3] {
                    // dbg!(1);
                    // surrounding_blocks = [false ; 3 * 3 * 3];
                    apply_sl_quad(
                        mesh,
                        &metadata.vivi,
                        index,
                        face,
                        surrounding_blocks,
                        sl,
                        reg.get_voxel_dimensions(),
                        dims,
                    )
                }
            }
        }
    }
}

fn covering_multiple_faces<T>(
    reg: &impl VoxelRegistry<Voxel = T>,
    voxel: &T,
    faces: [bool; 6],
) -> bool {
    for (i, b) in faces.iter().enumerate() {
        if !*b {
            continue;
        }
        if !reg.is_covering(voxel, Face::from(i)) {
            // dbg!(2);
            return false;
        }
    }
    true
}

pub fn apply_smooth_lighting_with_connected_chunks_arc<T, const N: usize>(
    reg: &impl VoxelRegistry<Voxel = T>,
    mesh: &mut Mesh,
    metadata: &MeshMD<T>,
    dims: Dimensions,
    lower_bound: usize,
    upper_bound: usize,
    this_chunk: &[T; N],
    north_chunk: Option<&Arc<RwLock<[T; N]>>>,
    south_chunk: Option<&Arc<RwLock<[T; N]>>>,
    east_chunk: Option<&Arc<RwLock<[T; N]>>>,
    west_chunk: Option<&Arc<RwLock<[T; N]>>>,
    no_east_chunk: Option<&Arc<RwLock<[T; N]>>>,
    no_west_chunk: Option<&Arc<RwLock<[T; N]>>>,
    so_east_chunk: Option<&Arc<RwLock<[T; N]>>>,
    so_west_chunk: Option<&Arc<RwLock<[T; N]>>>,
) {
    // let north_chunk = &*north_chunk.unwrap().read().unwrap();
    if let Some(sl) = metadata.smooth_lighting_params {
        for (index, quads) in metadata.vivi.vivi.iter().enumerate().skip(lower_bound) {
            if index > upper_bound {
                break;
            }
            for q in quads {
                let mut surrounding_blocks = [false; 3 * 3 * 3];
                let cage_dims = (3, 3, 3);
                let face = face_from_u32(q & REVERSE_OFFSET_CONST);

                if (matches!(face, Bottom) || matches!(face, Top))
                    && is_block_on_edge(dims, index, face)
                {
                    continue;
                }
                let (neighbor, chunk_dir) = {
                    if is_block_on_edge(dims, index, face) {
                        (
                            get_neigbhor_across_chunk(dims, index, face),
                            Some(crate::util::Direction::from(face)),
                        )
                    } else {
                        (get_neighbor(index, face, dims).unwrap(), None)
                    }
                };
                // if reg.is_covering(&this_chunk[neighbor], face.opposite()) { continue; }

                let og_index_in_cage: [i32; 3] = match face {
                    Top => [0, -1, 0],
                    Bottom => [0, 1, 0],
                    Right => [-1, 0, 0],
                    Left => [1, 0, 0],
                    Back => [0, 0, -1],
                    Forward => [0, 0, 1],
                };
                let [og_x, og_y, og_z] = og_index_in_cage;

                for y in -1..=1 {
                    for z in -1..=1 {
                        for x in -1..=1 {
                            if (og_x == x && og_y == y)
                                || (og_x == x && og_z == z)
                                || (og_y == y && og_z == z)
                            {
                                continue;
                            }
                            if (og_x == x && og_x != 0)
                                || (og_y == y && og_y != 0)
                                || (og_z == z && og_z != 0)
                            {
                                continue;
                            }
                            if (og_x == x + 2 && og_x != 0)
                                || (og_y == y + 2 && og_y != 0)
                                || (og_z == z + 2 && og_z != 0)
                            {
                                continue;
                            }
                            if (og_x == x - 2 && og_x != 0)
                                || (og_y == y - 2 && og_y != 0)
                                || (og_z == z - 2 && og_z != 0)
                            {
                                continue;
                            }

                            let cage_index = one_d_cords(
                                [(x + 1) as usize, (y + 1) as usize, (z + 1) as usize],
                                cage_dims,
                            );
                            let faces = [y < 0, y > 0, x < 0, x > 0, z < 0, z > 0];

                            match get_block_n_away(dims, neighbor, x, y, z) {
                                None => {
                                    continue;
                                }
                                Some((dir, neighbor_index)) => {
                                    let final_dir = crate::prelude::util::Direction::add_direction(
                                        chunk_dir, dir,
                                    );
                                    // if chunk_dir.is_some() && dir.is_some() {
                                    //     dbg!(final_dir, chunk_dir.unwrap(), dir.unwrap());
                                    // }
                                    surrounding_blocks[cage_index] = match final_dir {
                                        None => covering_multiple_faces(
                                            reg,
                                            &this_chunk[neighbor_index],
                                            faces,
                                        ),
                                        // Some(North) => covering_multiple_faces(
                                        //     reg,
                                        //     &north_chunk[neighbor_index],
                                        //     faces,
                                        // ),
                                        Some(North) if north_chunk.is_some() => {
                                            covering_multiple_faces(
                                                reg,
                                                &north_chunk.unwrap().read().unwrap()
                                                    [neighbor_index],
                                                faces,
                                            )
                                        }
                                        Some(South) if south_chunk.is_some() => {
                                            covering_multiple_faces(
                                                reg,
                                                &south_chunk.unwrap().read().unwrap()
                                                    [neighbor_index],
                                                faces,
                                            )
                                        }
                                        Some(East) if east_chunk.is_some() => {
                                            covering_multiple_faces(
                                                reg,
                                                &east_chunk.unwrap().read().unwrap()
                                                    [neighbor_index],
                                                faces,
                                            )
                                        }
                                        Some(West) if west_chunk.is_some() => {
                                            covering_multiple_faces(
                                                reg,
                                                &west_chunk.unwrap().read().unwrap()
                                                    [neighbor_index],
                                                faces,
                                            )
                                        }
                                        Some(NoEast) if no_east_chunk.is_some() => {
                                            covering_multiple_faces(
                                                reg,
                                                &no_east_chunk.unwrap().read().unwrap()
                                                    [neighbor_index],
                                                faces,
                                            )
                                        }
                                        Some(NoWest) if no_west_chunk.is_some() => {
                                            covering_multiple_faces(
                                                reg,
                                                &no_west_chunk.unwrap().read().unwrap()
                                                    [neighbor_index],
                                                faces,
                                            )
                                        }
                                        Some(SoEast) if so_east_chunk.is_some() => {
                                            covering_multiple_faces(
                                                reg,
                                                &so_east_chunk.unwrap().read().unwrap()
                                                    [neighbor_index],
                                                faces,
                                            )
                                        }
                                        Some(SoWest) if so_west_chunk.is_some() => {
                                            covering_multiple_faces(
                                                reg,
                                                &so_west_chunk.unwrap().read().unwrap()
                                                    [neighbor_index],
                                                faces,
                                            )
                                        }
                                        _ => false,
                                    };
                                }
                            }
                        }
                    }
                }
                if surrounding_blocks != [false; 3 * 3 * 3] {
                    // dbg!(1);
                    // surrounding_blocks = [false ; 3 * 3 * 3];
                    apply_sl_quad(
                        mesh,
                        &metadata.vivi,
                        index,
                        face,
                        surrounding_blocks,
                        sl,
                        reg.get_voxel_dimensions(),
                        dims,
                    )
                }
            }
        }
    }
}
