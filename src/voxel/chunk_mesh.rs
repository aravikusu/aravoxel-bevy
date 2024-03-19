use std::collections::HashMap;
use bevy::math::IVec3;
use bevy::prelude::{Res};
use rand::Rng;
use crate::global::Settings;
use crate::voxel::chunk::Chunk;
use crate::voxel::mesh::Mesh;
use crate::voxel::util::{CHUNK_SIZE, get_ao, voxel_index};
use crate::voxel::voxel::{Voxel, VoxelType};

/// The ChunkMesh holds all relevant data for this specific Chunk.
pub struct ChunkMesh {
    /// All solid ground.
    pub mesh: Mesh,

    /// All liquids. Since liquids are all transparent, they need
    /// to be in their own mesh.
    pub liquid_mesh: Mesh,
}

impl Default for ChunkMesh {
    fn default() -> Self {
        Self {
            mesh: Mesh::default(),
            liquid_mesh: Mesh::default(),
        }
    }
}

impl ChunkMesh {
    /// Builds a ChunkMesh out of a chunk.
    pub fn build_chunk_mesh(
        &mut self,
        chunk: &Chunk,
        chunks: &HashMap<IVec3, Chunk>,
        settings: &Res<Settings>,
    ) {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let index = voxel_index(x, y, z);

                    if let Some(voxel) = chunk.voxels.get(index) {
                        if !voxel.voxel_type.is_visible() {
                            continue;
                        }

                        self.create_voxel_data(chunk, voxel, chunks);
                    }
                }
            }
        }

        if settings.clown_vomit {
            self.mesh.clown_vomit();
            self.liquid_mesh.clown_vomit();
        }
    }

    /// Creates the ModelVertex vector as well as the index vector for our current Voxel.
    ///
    /// * `chunk`: The Chunk this Voxel resides within.
    /// * `voxel`: The Voxel itself.
    /// * `start_index`: The current amount of Vertices. Used to set the indices correctly.
    /// * `world_chunks`: All the chunks inside our world. Used so we can access another Chunk's
    /// Voxels while we draw in case the neighboring Voxel isn't local to our current Chunk.
    fn create_voxel_data(
        &mut self,
        chunk: &Chunk,
        voxel: &Voxel,
        world_chunks: &HashMap<IVec3, Chunk>,
    ) {
        let lx = voxel.local_position.x;
        let ly = voxel.local_position.y;
        let lz = voxel.local_position.z;

        // World position of Voxel
        let wx = voxel.world_position.x as f32;
        let wy = voxel.world_position.y as f32;
        let wz = voxel.world_position.z as f32;

        // Determine if this should go to the liquid mesh or normal one
        let mut mesh = if voxel.voxel_type.is_liquid() {
            &mut self.liquid_mesh
        } else {
            &mut self.mesh
        };


        // Check if there is a solid voxel above
        if chunk.is_void(&voxel.voxel_type, IVec3::new(lx, ly + 1, lz), world_chunks) {
            mesh.set_indices(vec![0, 3, 1, 1, 3, 2]);
            mesh.set_vertices(vec![
                [wx + -0.5, wy + 0.5, wz + -0.5],
                [wx + 0.5, wy + 0.5, wz + -0.5],
                [wx + 0.5, wy + 0.5, wz + 0.5],
                [wx + -0.5, wy + 0.5, wz + 0.5],
            ]);

            mesh.set_normals(IVec3::Y);
            let aos = get_ao(chunk, &voxel.voxel_type, voxel.local_position, IVec3::Y, world_chunks);
            mesh.aos.extend_from_slice(&aos);
            mesh.colorize_vertices(&voxel.voxel_type);
        }

        // Check under...
        if chunk.is_void(&voxel.voxel_type, IVec3::new(lx, ly - 1, lz), world_chunks) {
            mesh.set_indices(vec![0, 1, 3, 1, 2, 3]);
            mesh.set_vertices(
                vec!(
                    [wx + -0.5, wy + -0.5, wz + -0.5],
                    [wx + 0.5, wy + -0.5, wz + -0.5],
                    [wx + 0.5, wy + -0.5, wz + 0.5],
                    [wx + -0.5, wy + -0.5, wz + 0.5],
                )
            );
            mesh.set_normals(IVec3::NEG_Y);
            let aos = get_ao(chunk, &voxel.voxel_type, voxel.local_position, IVec3::NEG_Y, world_chunks);
            mesh.aos.extend_from_slice(&aos);
            mesh.colorize_vertices(&voxel.voxel_type);
        }

        // Right
        if chunk.is_void(&voxel.voxel_type, IVec3::new(lx + 1, ly, lz), world_chunks) {
            mesh.set_indices(vec![0, 3, 1, 1, 3, 2]);
            mesh.set_vertices(
                vec!(
                    [wx + 0.5, wy + -0.5, wz + -0.5],
                    [wx + 0.5, wy + -0.5, wz + 0.5],
                    [wx + 0.5, wy + 0.5, wz + 0.5],
                    [wx + 0.5, wy + 0.5, wz + -0.5],
                )
            );

            mesh.set_normals(IVec3::X);
            let aos = get_ao(chunk, &voxel.voxel_type, voxel.local_position, IVec3::X, world_chunks);
            mesh.aos.extend_from_slice(&aos);
            mesh.colorize_vertices(&voxel.voxel_type);
        }

        // Left
        if chunk.is_void(&voxel.voxel_type, IVec3::new(lx - 1, ly, lz), world_chunks) {
            mesh.set_indices(vec![0, 1, 3, 1, 2, 3]);
            mesh.set_vertices(
                vec!(
                    [wx + -0.5, wy + -0.5, wz + -0.5],
                    [wx + -0.5, wy + -0.5, wz + 0.5],
                    [wx + -0.5, wy + 0.5, wz + 0.5],
                    [wx + -0.5, wy + 0.5, wz + -0.5],
                )
            );
            mesh.set_normals(IVec3::X);
            let aos = get_ao(chunk, &voxel.voxel_type, voxel.local_position, IVec3::X, world_chunks);
            mesh.aos.extend_from_slice(&aos);
            mesh.colorize_vertices(&voxel.voxel_type);
        }

        // Behind
        if chunk.is_void(&voxel.voxel_type, IVec3::new(lx, ly, lz + 1), world_chunks) {
            mesh.set_indices(vec![0, 3, 1, 1, 3, 2]);
            mesh.set_vertices(
                vec!(
                    [wx + -0.5, wy + -0.5, wz + 0.5],
                    [wx + -0.5, wy + 0.5, wz + 0.5],
                    [wx + 0.5, wy + 0.5, wz + 0.5],
                    [wx + 0.5, wy + -0.5, wz + 0.5],
                )
            );
            mesh.set_normals(IVec3::Z);
            let aos = get_ao(chunk, &voxel.voxel_type,  voxel.local_position, IVec3::Z, world_chunks);
            mesh.aos.extend_from_slice(&aos);
            mesh.colorize_vertices(&voxel.voxel_type);
        }

        // In front
        if chunk.is_void(&voxel.voxel_type, IVec3::new(lx, ly, lz - 1), world_chunks) {
            mesh.set_indices(vec![0, 1, 3, 1, 2, 3]);
            mesh.set_vertices(
                vec!(
                    [wx + -0.5, wy + -0.5, wz + -0.5],
                    [wx + -0.5, wy + 0.5, wz + -0.5],
                    [wx + 0.5, wy + 0.5, wz + -0.5],
                    [wx + 0.5, wy + -0.5, wz + -0.5],
                )
            );

            mesh.set_normals(IVec3::NEG_Z);
            let aos = get_ao(chunk, &voxel.voxel_type, voxel.local_position, IVec3::NEG_Z, world_chunks);
            mesh.aos.extend_from_slice(&aos);
            mesh.colorize_vertices(&voxel.voxel_type);
        }
    }
}