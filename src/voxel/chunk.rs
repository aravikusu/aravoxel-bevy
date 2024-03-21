use std::collections::HashMap;
use bevy::math::{IVec3, Vec2};
use libnoise::{Generator, Source};
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

    /// Generates this Chunk using noise.
    pub fn generate(&mut self) {
        let noise = Source::improved_perlin(42069).fbm(
            1, 1.0, 2.0, 0.5,
        );

        let new_pos = self.position * CHUNK_SIZE;

        for x in 0..CHUNK_SIZE {
            let wx = x + new_pos.x;
            for z in 0..CHUNK_SIZE {
                let wz = z + new_pos.z;

                let some_pos = Vec2::new(wx as f32, wz as f32) * 0.01;
                let sample = noise.sample([some_pos.x as f64, some_pos.y as f64]);

                // Determine which voxels are going to be solid
                let world_height = (sample * 32.0 + 32.0) as i32;
                let local_height = i32::min(world_height - new_pos.y, CHUNK_SIZE);

                for y in 0..CHUNK_SIZE {
                    let index = voxel_index(x, y, z);
                    self.voxels[index].local_position = IVec3::new(x, y, z);
                    self.voxels[index].world_position = IVec3::new(
                        x + self.position.x * CHUNK_SIZE,
                        y + self.position.y * CHUNK_SIZE,
                        z + self.position.z * CHUNK_SIZE,
                    );

                    if y < local_height {
                        self.voxels[index].voxel_type = VoxelType::GRASS;
                    }
                }
            }
        }

        // Test
        for voxel in &mut self.voxels {
            if !voxel.voxel_type.is_visible() && voxel.world_position.y < 10 {
                voxel.voxel_type = VoxelType::WATER;
            }

            if voxel.voxel_type.is_visible() && voxel.world_position.y > 23 {
                voxel.voxel_type = VoxelType::STONE;
            }
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