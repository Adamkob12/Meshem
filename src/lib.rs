pub(crate) mod adj;
pub(crate) mod face;
pub(crate) mod mesh_metadata;
pub(crate) mod meshem;
pub(crate) mod pbs;
pub(crate) mod update;
pub(crate) mod util;
pub(crate) mod voxel_mesh;

use bevy::log::warn;
use bevy::render::mesh::{Mesh, MeshVertexAttribute};

pub mod prelude {
    pub use crate::adj::*;
    pub use crate::face::Face::*;
    pub use crate::face::*;
    pub use crate::mesh_metadata::*;
    pub use crate::meshem::*;
    pub use crate::pbs::*;
    pub use crate::update::*;
    pub(crate) use crate::util::compressed_voxel_grid::*;
    pub(crate) use crate::util::vav::*;
    pub use crate::util::*;
    pub use crate::voxel_mesh::*;
    pub use crate::VoxelRegistry;
    pub use crate::*;
}

/// Implementing this trait for your own data-structure is the most important
/// prerequesite if you want to use the function.
pub trait VoxelRegistry {
    type Voxel: std::fmt::Debug + Eq + PartialEq + Sized + Clone + Copy;
    /// Returns None if the mesh is "irrelevant" as in it's air or not a Voxel.
    fn get_mesh(&self, voxel: &Self::Voxel) -> VoxelMesh<&Mesh>;
    /// Would this voxel cover the voxel that's located on it's `side`? for example, an air block
    /// would not cover any side, but a slab would only cover the bottom.
    fn is_covering(&self, voxel: &Self::Voxel, side: prelude::Face) -> bool;
    /// The center of the voxel (physical center, the center of the default block is 0,0,0 eg)
    fn get_center(&self) -> [f32; 3];
    /// All the voxels must have standard and equal dimesions (y is up).
    fn get_voxel_dimensions(&self) -> [f32; 3];
    /// The attributes we are considering while meshing the grid.
    fn all_attributes(&self) -> Vec<MeshVertexAttribute>;
}

/// (width, height, length) - note that bevy considers the "y position" to be height.
pub type Dimensions = (usize, usize, usize);

pub enum VoxelMesh<T> {
    NormalCube(T),
    CustomMesh(T),
    Null,
}

impl<T> VoxelMesh<T> {
    pub fn unwrap(self) -> T {
        match self {
            Self::NormalCube(t) => t,
            Self::CustomMesh(t) => {
                warn!("Custom Meshes are still not properly implemented!");
                t
            }
            Self::Null => panic!("Tried unwrapping a Null VoxelMesh type."),
        }
    }

    pub fn expect(self, msg: &str) -> T {
        match self {
            Self::NormalCube(t) => t,
            Self::CustomMesh(t) => {
                warn!("Custom Meshes are still not properly implemented!");
                t
            }
            Self::Null => panic!("{}", msg),
        }
    }
}

/// [+y, -y, +x, -x, +z, -z], true if that face is not covered.
pub(crate) type Neighbors = [bool; 6];
