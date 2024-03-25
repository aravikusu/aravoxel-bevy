use std::collections::HashMap;
use bevy::prelude::*;
use noise::{Fbm, MultiFractal, NoiseFn, Perlin};
use splines::Spline;
use crate::voxel::chunk::Chunk;
use crate::voxel::util::{CHUNK_SIZE, CHUNK_SIZE_F64, voxel_index};
use crate::voxel::voxel::VoxelType;

#[derive(Resource)]
pub struct VoxelWorld {
    pub world_noise: Fbm<Perlin>,
    pub verticality: Perlin,
    pub spline_points: Spline<f64, f64>,
    pub chunks: HashMap<IVec3, Chunk>,
    //meshes: HashMap<IVec3, ChunkMesh>
}

impl Default for VoxelWorld {
    fn default() -> Self {
        Self {
            world_noise: Fbm::<Perlin>::new(42069).set_octaves(6),
            verticality: Perlin::new(42069),
            spline_points: Spline::from_vec(vec![
                splines::Key::new(-1., 0.6, splines::Interpolation::Cosine),
                splines::Key::new(-0.9, 0.7, splines::Interpolation::Cosine),
                splines::Key::new(0., 0.8, splines::Interpolation::Cosine),
                splines::Key::new(0.5, 0.85, splines::Interpolation::Cosine),
                splines::Key::new(0.8, 0.9, splines::Interpolation::Cosine),
                splines::Key::new(0.9, 1., splines::Interpolation::Cosine),
                splines::Key::new(1.1, 1.5, splines::Interpolation::default())
            ]),
            chunks: HashMap::new(),
            //meshes: HashMap::new(),
        }
    }
}

impl VoxelWorld {
    /// Generates a Chunk using the world parameters.
    pub fn generate_chunk(&mut self, chunk_pos: IVec3) {
        let mut chunk = Chunk::new(chunk_pos);

        let new_pos = chunk.position * CHUNK_SIZE;

        for x in 0..CHUNK_SIZE {
            let wx = x + new_pos.x;
            for z in 0..CHUNK_SIZE {
                let wz = z + new_pos.z;

                let some_pos = Vec2::new(wx as f32, wz as f32) * 0.01;
                let mut sample = self.world_noise.get([some_pos.x as f64, some_pos.y as f64]);

                let vert = self.verticality.get([some_pos.x as f64, some_pos.y as f64]);

                sample *= (CHUNK_SIZE_F64 * 4.0) * self.spline_points.sample(vert).unwrap();

                // Determine which voxels are going to be solid
                let local_height = i32::min((sample as i32) - new_pos.y, CHUNK_SIZE);

                for y in 0..CHUNK_SIZE {
                    let index = voxel_index(x, y, z);
                    chunk.voxels[index].local_position = IVec3::new(x, y, z);
                    chunk.voxels[index].world_position = IVec3::new(
                        x + chunk.position.x * CHUNK_SIZE,
                        y + chunk.position.y * CHUNK_SIZE,
                        z + chunk.position.z * CHUNK_SIZE,
                    );

                    if y < local_height {
                        chunk.voxels[index].voxel_type = VoxelType::GRASS;
                    }
                }
            }
        }

        // Test
        for voxel in &mut chunk.voxels {
            if !voxel.voxel_type.is_visible() && voxel.world_position.y < 10 {
                voxel.voxel_type = VoxelType::WATER;
            }

            if voxel.voxel_type.is_visible() && voxel.world_position.y > 23 {
                voxel.voxel_type = VoxelType::STONE;
            }
        }

        self.chunks.insert(chunk_pos, chunk);
    }
}