use super::{Dimensions, Neighbors, VoxelRegistry};
use crate::util::vav::*;
use bevy::prelude::*;
use bevy::render::mesh::{
    Indices, MeshVertexAttribute, MeshVertexAttributeId, VertexAttributeValues,
};
use bevy::render::render_resource::PrimitiveTopology;

// Data structure "Voxel index to Vertex index", check it out in visuals
pub(crate) type VIVI = Vec<Vec<u32>>;

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
    pub(crate) changed_voxels: Vec<(T, u32, VoxelChange, [Option<T>; 6])>,
}
