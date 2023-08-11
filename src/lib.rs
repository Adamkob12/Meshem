#![allow(dead_code, unused_imports, unused_variables)]
pub mod default_block;
pub mod meshem;
mod util;

pub use crate::default_block::*;
pub use crate::meshem::*;
use bevy::render::mesh::{
    Indices, Mesh, MeshVertexAttribute, MeshVertexAttributeId, VertexAttributeValues,
};

pub trait VoxelRegistry {
    type Voxel;
    /// returns None if the mesh is "irrelevant" as in it's air or not a Voxel.
    fn get_mesh(&self, voxel: &Self::Voxel) -> Option<&Mesh>;
    fn is_voxel(&self, voxel: &Self::Voxel) -> bool;
    fn get_center(&self) -> [f32; 3];
    fn all_attributes(&self) -> Vec<MeshVertexAttribute>;
}

/// (width, length, height) - note that bevy considers the "y position" to be height.
pub type Dimensions = (usize, usize, usize);

/// [+y, -y, +x, -x, +z, -z], true if that face is not covered.
pub(crate) type Neighbors = [bool; 6];
