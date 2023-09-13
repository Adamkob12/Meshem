use super::{face_to_u32, Dimensions, Face, Neighbors, VoxelRegistry};
use crate::util::vav::*;
use crate::OFFSET_CONST;
use bevy::prelude::*;
use bevy::render::mesh::{
    Indices, MeshVertexAttribute, MeshVertexAttributeId, VertexAttributeValues,
};
use bevy::render::render_resource::PrimitiveTopology;
use bevy::utils::hashbrown::HashMap;

// Data structure "Voxel index to Vertex index", check it out in visuals
pub(crate) struct VIVI {
    vivi: Vec<Vec<u32>>,
    pub(crate) map: HashMap<u32, u32>,
}

impl VIVI {
    pub(crate) fn new(voxel_count: usize) -> VIVI {
        VIVI {
            vivi: vec![vec![]; voxel_count],
            map: HashMap::new(),
        }
    }

    pub(crate) fn insert(&mut self, face: Face, voxel_index: usize, vertex: u32) {
        self.vivi[voxel_index].push((vertex) | face_to_u32(face));
        self.map
            .insert(vertex, voxel_index as u32 | face_to_u32(face));
    }

    pub(crate) fn get_quad_index(&self, face: Face, voxel_index: usize) -> Option<u32> {
        for quad in self.vivi[voxel_index].iter() {
            let tmp = quad & !OFFSET_CONST;
            if tmp == face_to_u32(face) {
                return Some(quad & (OFFSET_CONST));
            }
        }
        None
    }

    pub(crate) fn change_quad_index(&mut self, old_vertex: usize, new_vertex: usize) {
        let voxel = self
            .map
            .remove(&(old_vertex as u32))
            .expect("Couldn't find voxel matching vertex");
        let q = voxel & !OFFSET_CONST;
        let v = voxel & OFFSET_CONST;
        let old_vertex = old_vertex as u32 | q;
        for v in self.vivi[v as usize].iter_mut() {
            if *v == old_vertex {
                *v = new_vertex as u32 | q;
                self.map.insert(new_vertex as u32, voxel);
                return;
            }
        }
        panic!("Couldn't find vertex index in VIVI");
    }
}

/// This enum represents all the way a voxel could be changed.
#[derive(Clone, Copy)]
pub enum VoxelChange {
    Broken,
    Added,
}

/// Mesh meta-data struct.
/// T is the voxel type, it needs to be the same as the voxel registry.
pub struct MeshMD<T> {
    pub(crate) vivi: VIVI,
    // The grid dimensions
    pub dims: Dimensions,
    // T: the voxel type,
    // u32: the index of the voxel in the grid,
    // ChangeInVoxel: whether the voxel was added or removed,
    // [Option<T>; 6]: the neighbors of the voxel, in the same order as
    //    in the `Neighbors` data-type, if the voxel is "empty"- None.
    pub(crate) changed_voxels: Vec<(T, usize, VoxelChange, [Option<T>; 6])>,
}

impl<T> MeshMD<T> {
    pub fn log(
        &mut self,
        voxel_change: VoxelChange,
        voxel_index: usize,
        voxel: T,
        neighboring_voxels: [Option<T>; 6],
    ) {
        self.changed_voxels
            .push((voxel, voxel_index, voxel_change, neighboring_voxels));
    }
}
