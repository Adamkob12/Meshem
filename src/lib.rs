#![allow(dead_code, unused_imports, unused_variables)]
pub mod default_block;
pub mod dynamic_mesh;
pub mod meshem;

mod util;

use crate::default_block::*;
use crate::dynamic_mesh::*;
use crate::meshem::*;
use bevy::render::mesh::{
    Indices, Mesh, MeshVertexAttribute, MeshVertexAttributeId, VertexAttributeValues,
};

/// Implementing this trait for your own data-structure is the most important
/// prerequesite if you want to use the function.
pub trait VoxelRegistry {
    type Voxel;
    /// Returns None if the mesh is "irrelevant" as in it's air or not a Voxel.
    fn get_mesh(&self, voxel: &Self::Voxel) -> Option<&Mesh>;
    /// Should the algorithm consider this Voxel "full"?, for example, in Minecraft,
    /// "Air" would not be a full block because it doesn't block the view.
    fn is_voxel(&self, voxel: &Self::Voxel) -> bool;
    /// The center of the voxel (physical center, the center of the default block is 0,0,0 eg)
    fn get_center(&self) -> [f32; 3];
    /// All the voxels must have standard and equal dimesions (y is up).
    fn get_voxel_dimensions(&self) -> [f32; 3];
    /// The attributes we are considering while meshing the grid.
    fn all_attributes(&self) -> Vec<MeshVertexAttribute>;
}

/// (width, length, height) - note that bevy considers the "y position" to be height.
pub type Dimensions = (usize, usize, usize);

/// [+y, -y, +x, -x, +z, -z], true if that face is not covered.
pub(crate) type Neighbors = [bool; 6];

#[derive(Copy, Clone)]
pub(crate) enum Face {
    Top,
    Bottom,
    Right,
    Left,
    Back,
    Forward,
}

pub(crate) fn face_to_u32(f: Face) -> u32 {
    match f {
        // 101 -> 2^31 + 2^29
        Face::Top => 2_684_354_560,
        // 100 -> 2^31
        Face::Bottom => 2_147_483_648,
        // 011 -> 2^30 + 2^29
        Face::Right => 1_610_612_736,
        // 010 -> 2^30
        Face::Left => 1_073_741_824,
        // 001 -> 2^29
        Face::Back => 536_870_912,
        // 000 -> 0
        Face::Forward => 0,
    }
}

impl Into<usize> for Face {
    fn into(self) -> usize {
        match self {
            Face::Top => 0,
            Face::Bottom => 1,
            Face::Right => 2,
            Face::Left => 3,
            Face::Back => 4,
            Face::Forward => 5,
        }
    }
}
