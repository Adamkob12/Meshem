use crate::dynamic_mesh::*;
use crate::meshem::*;
use crate::*;
use bevy::render::mesh::{
    Indices, MeshVertexAttribute, MeshVertexAttributeId, VertexAttributeValues,
};
use bevy::render::render_resource::PrimitiveTopology;

pub fn update_mesh<T>(mesh: &mut Mesh, metadata: MeshMD<T>, reg: &impl VoxelRegistry<Voxel = T>) {
    let center = reg.get_center();
    let voxel_dims = reg.get_voxel_dimensions();
}

fn remove_quads_facing(mesh: &mut Mesh, vivi: &VIVI, voxel_index: usize) {}

fn remove_voxel(mesh: &mut Mesh, vivi: &VIVI, voxel_index: usize) {}

fn add_quads_facing(mesh: &mut Mesh, vivi: &VIVI, voxel_index: usize) {}
