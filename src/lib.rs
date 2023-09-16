#![allow(dead_code, unused_imports)]
pub(crate) mod default_block;
pub mod face;
pub(crate) mod mesh_metadata;
pub(crate) mod meshem;
pub(crate) mod update;
pub(crate) mod util;
pub(crate) mod voxel;

use bevy::render::mesh::{Mesh, MeshVertexAttribute};

pub mod prelude {
    pub use crate::default_block::*;
    pub use crate::face::Face::*;
    pub use crate::face::*;
    pub use crate::mesh_metadata::*;
    pub use crate::meshem::*;
    pub use crate::update::*;
    pub use crate::util::vav::*;
    pub use crate::util::*;
    pub use crate::VoxelRegistry;
    pub use crate::*;
}

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
