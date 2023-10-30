use crate::prelude::*;
use bevy::prelude::*;

/// This function will iron out any problems caused by generating two chunks that are adjacent to
/// each other seperatly. For example, it will cull the unneeded vertices between them, and fix
/// the proximity based shadowing.
/// reg: the Voxel Registry
/// main_mesh: the mesh to change
/// main_md: the metadata of the mesh to change
/// connection_side: from the POV of the main mesh, where is the adjacent mesh?
/// adjacent_chunk_grid: the grid of the chunk to introduce
pub fn introduce_adjacent_chunks<T: std::fmt::Debug + Sized + Copy>(
    reg: &impl VoxelRegistry<Voxel = T>,
    main_mesh: &mut Mesh,
    main_md: &mut MeshMD<T>,
    connection_side: Face,
    adjacent_chunk_grid: &[T],
) {
    assert_eq!(
        adjacent_chunk_grid.len(),
        main_md.vivi.vivi.len(),
        "Cannot introduce chunks with different sizes to each other"
    );
    let dims = main_md.dims;
    for index in iter_faces_of_chunk(dims, connection_side) {
        let adj_voxel_index = get_neigbhor_across_chunk(dims, index, connection_side);
        let adj_voxel = adjacent_chunk_grid[adj_voxel_index];
        if reg.is_covering(&adj_voxel, connection_side.opposite()) {
            let mut tmp = [None; 6];
            tmp[connection_side as usize] = Some(adj_voxel);
            main_md.log(VoxelChange::CullFaces, index, adj_voxel, tmp)
        }
    }
    update_mesh(main_mesh, main_md, reg);

    if let Some(pbs) = main_md.pbs {
        for index in iter_faces_of_chunk(dims, connection_side) {
            if main_md
                .vivi
                .get_quad_index(connection_side, index)
                .is_some()
            {
                let adj_voxel_index = get_neigbhor_across_chunk(dims, index, connection_side);
                let mut closest_voxels = [false; 6];
                for i in 0..6 {
                    let tmp_face = Face::from(i);
                    if let Some(neighbor) = get_neighbor(adj_voxel_index, tmp_face, dims) {
                        closest_voxels[i] =
                            reg.is_covering(&adjacent_chunk_grid[neighbor], tmp_face.opposite());
                    }
                }
                if closest_voxels != [false; 6] {
                    apply_pbs_quad(
                        main_mesh,
                        &main_md.vivi,
                        index,
                        connection_side,
                        closest_voxels,
                        pbs,
                        reg.get_voxel_dimensions(),
                        dims,
                    )
                }
            }
        }
    }
}
