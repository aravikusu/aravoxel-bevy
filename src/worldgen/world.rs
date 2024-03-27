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
            world_noise: Fbm::<Perlin>::new(6346547).set_octaves(6).set_lacunarity(2.0).set_persistence(0.5),
            verticality: Perlin::new(6346547),
            spline_points: Spline::from_vec(vec![
                splines::Key::new(-1., 0.3, splines::Interpolation::Linear),
                splines::Key::new(-0.6, 0.35, splines::Interpolation::Linear),
                splines::Key::new(-0.4, 1.0, splines::Interpolation::Linear),
                splines::Key::new(-0.2, 1.0, splines::Interpolation::Linear),
                splines::Key::new(0.0, 1.3, splines::Interpolation::Linear),
                splines::Key::new(0.1, 1.4, splines::Interpolation::Linear),
                splines::Key::new(0.2, 1.4, splines::Interpolation::Linear),
                splines::Key::new(0.3, 1.4, splines::Interpolation::Linear),
                splines::Key::new(0.4, 1.7, splines::Interpolation::Linear),
                splines::Key::new(0.5, 2.0, splines::Interpolation::Linear),
                splines::Key::new(0.6, 2.1, splines::Interpolation::Linear),
                splines::Key::new(0.7, 3.0, splines::Interpolation::Linear),
                splines::Key::new(1.1, 3.0, splines::Interpolation::default())
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

        let world_pos = chunk_pos * CHUNK_SIZE;

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let wx = (x + world_pos.x) as f64;
                    let wy = (y + world_pos.y) as f64;
                    let wz = (z + world_pos.z) as f64;

                    let nx = wx / ((CHUNK_SIZE_F64 * 4.0) * (CHUNK_SIZE_F64 * 0.1));
                    let nz = wz / ((CHUNK_SIZE_F64 * 4.0) * (CHUNK_SIZE_F64 * 0.1));

                    let mut sample = self.world_noise.get([nz, nx]);
                    let vert = self.verticality.get([nx, nz]);
                    sample *= (CHUNK_SIZE_F64 * 4.0) * self.spline_points.sample(vert).unwrap();

                    let index = voxel_index(x, y, z);
                    chunk.voxels[index].local_position = IVec3::new(x, y, z);
                    chunk.voxels[index].world_position = IVec3::new(
                        x + chunk.position.x * CHUNK_SIZE,
                        y + chunk.position.y * CHUNK_SIZE,
                        z + chunk.position.z * CHUNK_SIZE,
                    );

                    if sample < wy && wy <= 20.0 {
                        chunk.voxels[index].voxel_type = VoxelType::WATER;
                        continue;
                    }

                    if wy <= sample {
                        chunk.voxels[index].voxel_type = VoxelType::STONE;
                    }
                }
            }
        }

        self.chunks.insert(chunk_pos, chunk);
    }
}