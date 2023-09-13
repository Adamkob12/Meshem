#![allow(dead_code, unused_imports, unused_variables)]
pub mod default_block;
pub mod mesh_metadata;
pub mod meshem;
pub mod update;
pub mod util;

use crate::default_block::*;
pub use crate::mesh_metadata::*;
use crate::meshem::*;
use bevy::render::mesh::{
    Indices, Mesh, MeshVertexAttribute, MeshVertexAttributeId, VertexAttributeValues,
};
use update::*;

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
pub enum Face {
    Top,
    Bottom,
    Right,
    Left,
    Back,
    Forward,
}

const OFFSET_CONST: u32 = 0b0001_1111_1111_1111_1111_1111_1111_1111;

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

impl Face {
    pub(crate) fn opposite(&self) -> Face {
        match *self {
            Face::Top => Face::Bottom,
            Face::Bottom => Face::Top,
            Face::Right => Face::Left,
            Face::Left => Face::Right,
            Face::Back => Face::Forward,
            Face::Forward => Face::Back,
        }
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

impl From<usize> for Face {
    fn from(i: usize) -> Face {
        match i {
            0 => Face::Top,
            1 => Face::Bottom,
            2 => Face::Right,
            3 => Face::Left,
            4 => Face::Back,
            5 => Face::Forward,
            _ => panic!("Face can only be infered from 6 values, 0..5 (inclucive)"),
        }
    }
}
