mod chunk;
mod chunk_map;
mod configuration;
#[cfg(feature = "display")]
mod debug;
#[cfg(feature = "display")]
mod mesh_cache;
#[cfg(feature = "display")]
mod meshing;
mod plugin;
mod voxel;
mod voxel_material;
mod voxel_traversal;
mod voxel_world;
mod voxel_world_internal;

pub mod prelude {
    pub use crate::chunk::{Chunk, NeedsDespawn};
    pub use crate::configuration::*;
    #[cfg(feature = "display")]
    pub use crate::debug::{ChunkAabbGizmo, VoxelWorldGizmoPlugin};
    pub use crate::plugin::VoxelWorldPlugin;
    pub use crate::voxel::{VoxelFace, WorldVoxel, VOXEL_SIZE};
    pub use crate::voxel_world::{ChunkWillDespawn, ChunkWillRemesh, ChunkWillSpawn};
    pub use crate::voxel_world::{VoxelRaycastResult, VoxelWorld, VoxelWorldCamera};
}

#[cfg(feature = "display")]
pub mod rendering {
    pub use crate::plugin::VoxelWorldMaterialHandle;
    pub use crate::voxel_material::vertex_layout;
    pub use crate::voxel_material::VOXEL_TEXTURE_SHADER_HANDLE;
}

pub mod traversal_alg {
    pub use crate::voxel_traversal::*;
}

#[cfg(test)]
mod test;
