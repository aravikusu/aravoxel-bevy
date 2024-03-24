use std::collections::HashMap;
use bevy::math::{IVec3};
use crate::voxel::util::{CHUNK_SIZE, CHUNK_VOL, voxel_index};
use crate::voxel::voxel::{Voxel, VoxelType};

#[derive(Clone)]
pub struct Chunk {
    pub position: IVec3,
    pub voxels: Vec<Voxel>,
}

impl Chunk {
    pub fn new(position: IVec3) -> Self {
        Self {
            position,
            voxels: vec![Voxel::default(); CHUNK_VOL as usize],
        }
    }

    /// Determines if a position is occupied by something "solid".
    /// Used to determine which sides of a voxel we render.
    ///
    /// * `current_voxel_type`: Some types, such as liquids, are solid but
    /// should still render blocks next to them under certain conditions.
    /// * `voxel_pos`: The voxel position we want to check.
    /// * `world_chunks`: All the chunks in our world. Used when the current
    /// voxel position is outside its local chunk's bounds.
    pub fn is_void(
        &self,
        current_voxel_type: &VoxelType,
        voxel_pos: IVec3,
        world_chunks: &HashMap<IVec3, Chunk>,
    ) -> bool {
        let x = voxel_pos.x;
        let y = voxel_pos.y;
        let z = voxel_pos.z;

        // First check if the position is local
        if (0..CHUNK_SIZE).contains(&x)
            && (0..CHUNK_SIZE).contains(&y)
            && (0..CHUNK_SIZE).contains(&z)
        {
            // Voxel exists inside our chunk. Get the index and check it.
            let idx = voxel_index(x, y, z);
            if let Some(voxel) = self.voxels.get(idx) {
                // If the current Voxel is liquid, we don't want to render any sides
                // that are within the body of water itself, so we only check if
                // the neighbor is air or not.
                return if current_voxel_type.is_liquid() {
                    !voxel.voxel_type.is_visible()
                } else {
                    voxel.voxel_type.should_render()
                };
            }
        } else {
            // Voxel exceeds chunk boundaries.
            // We need to know check the neighboring Chunk's voxel
            // to find out if we should draw or not.

            let c_pos = self.position;
            let mut neighbor_chunk_idx = IVec3::new(c_pos.x, c_pos.y, c_pos.z);
            let mut neighbor_voxel_pos = IVec3::new(x, y, z);
            if x > (CHUNK_SIZE - 1) {
                neighbor_chunk_idx.x += 1;
                neighbor_voxel_pos.x = 0
            } else if x < 0 {
                neighbor_chunk_idx.x -= 1;
                neighbor_voxel_pos.x = 31;
            }

            if y > (CHUNK_SIZE - 1) {
                neighbor_chunk_idx.y += 1;
                neighbor_voxel_pos.y = 0;
            } else if y < 0 {
                neighbor_chunk_idx.y -= 1;
                neighbor_voxel_pos.y = 31;
            }

            if z > (CHUNK_SIZE - 1) {
                neighbor_chunk_idx.z += 1;
                neighbor_voxel_pos.z = 0;
            } else if z < 0 {
                neighbor_chunk_idx.z -= 1;
                neighbor_voxel_pos.z = 31;
            }

            return Chunk::check_neighboring_chunk(current_voxel_type, neighbor_chunk_idx, neighbor_voxel_pos, world_chunks);
        }
        true
    }

    /// Tries to check the desired Voxel inside a specified Chunk.
    ///
    /// * `current_voxel_type`: VoxelType of the current voxel.
    /// Used so we can differentiate against normal solids and "liquids".
    /// * `chunk_idx`: The key for the Chunk we're interested in.
    /// * `voxel_pos`: The local position of the Voxel we're interested in.
    /// * `world_chunks`: All the loaded chunks located in our world.
    fn check_neighboring_chunk(
        current_voxel_type: &VoxelType,
        chunk_idx: IVec3,
        voxel_pos: IVec3,
        world_chunks: &HashMap<IVec3, Chunk>,
    ) -> bool {
        let x = voxel_pos.x;
        let y = voxel_pos.y;
        let z = voxel_pos.z;
        match world_chunks.get(&chunk_idx) {
            Some(chunk) => {
                let voxel_idx = voxel_index(x, y, z);

                if let Some(voxel) = chunk.voxels.get(voxel_idx) {
                    return if current_voxel_type.is_liquid() {
                        !voxel.voxel_type.is_visible()
                    } else {
                        voxel.voxel_type.should_render()
                    };
                }

                false
            }
            None => true
        }
    }
}