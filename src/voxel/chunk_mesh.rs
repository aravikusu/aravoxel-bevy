use std::collections::HashMap;
use bevy::math::IVec3;
use rand::Rng;
use crate::voxel::chunk::Chunk;
use crate::voxel::util::{CHUNK_SIZE, voxel_index};

pub struct ChunkMesh {
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
    pub colors: Vec<[f32; 4]>,
}

impl Default for ChunkMesh {
    fn default() -> Self {
        Self {
            vertices: Vec::new(),
            normals: Vec::new(),
            indices: Vec::new(),
            colors: Vec::new(),
        }
    }
}

impl ChunkMesh {
    /// Builds a ChunkMesh out of a chunk.
    pub fn build_chunk_mesh(
        &mut self,
        chunk: &Chunk,
        chunks: &HashMap<IVec3, Chunk>,
    ) {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let index = voxel_index(x, y, z);

                    if let Some(voxel) = chunk.voxels.get(index) {
                        if !voxel.is_solid {
                            continue;
                        }

                        let local_pos = IVec3::new(x, y, z);
                        let world_pos = IVec3::new(
                            x + chunk.position.x * CHUNK_SIZE,
                            y + chunk.position.y * CHUNK_SIZE,
                            z + chunk.position.z * CHUNK_SIZE,
                        );

                        self.create_voxel_data(chunk, local_pos, world_pos, chunks);
                    }
                }
            }
        }

        // Clown vomit colors
        // Obviously temporary...
        for _v in &self.vertices {
            let mut rng = rand::thread_rng();
            let color1 = rng.gen_range(0.0..1.0) as f32;
            let color2 = rng.gen_range(0.0..1.0) as f32;
            let color3 = rng.gen_range(0.0..1.0) as f32;
            self.colors.push([color1, color2, color3, 1.0]);
        }
    }

    /// Creates the ModelVertex vector as well as the index vector for our current Voxel.
    ///
    /// * `chunk` - The Chunk this Voxel resides within.
    /// * `local_pos` - This Voxel's local position within this Chunk.
    /// * `world_pos` - This Voxel's *world position*. Necessary to correctly draw the vertices.
    /// * `start_index` - The current amount of Vertices. Used to set the indices correctly.
    /// * `world_chunks` - All the chunks inside our world. Used so we can access another Chunk's
    /// Voxels while we draw in case the neighboring Voxel isn't local to our current Chunk.
    fn create_voxel_data(
        &mut self,
        chunk: &Chunk,
        local_pos: IVec3,
        world_pos: IVec3,
        world_chunks: &HashMap<IVec3, Chunk>,
    ) {
        let lx = local_pos.x;
        let ly = local_pos.y;
        let lz = local_pos.z;

        // World position of Voxel
        let wx = world_pos.x as f32;
        let wy = world_pos.y as f32;
        let wz = world_pos.z as f32;

        // Check if there is a solid voxel above
        if chunk.is_void(IVec3::new(lx, ly + 1, lz), world_chunks) {
            self.set_indices(vec![0, 3, 1, 1, 3, 2]);
            self.vertices.extend(vec![
                [wx + -0.5, wy + 0.5, wz + -0.5],
                [wx + 0.5, wy + 0.5, wz + -0.5],
                [wx + 0.5, wy + 0.5, wz + 0.5],
                [wx + -0.5, wy + 0.5, wz + 0.5],
            ]);

            self.normals.extend([[0.0, 1.0, 0.0]; 4])
        }

        // Check under...
        if chunk.is_void(IVec3::new(lx, ly - 1, lz), world_chunks) {
            self.set_indices(vec![0, 1, 3, 1, 2, 3]);
            self.vertices.extend(
                vec!(
                    [wx + -0.5, wy + -0.5, wz + -0.5],
                    [wx + 0.5, wy + -0.5, wz + -0.5],
                    [wx + 0.5, wy + -0.5, wz + 0.5],
                    [wx + -0.5, wy + -0.5, wz + 0.5],
                )
            );
            self.normals.extend([[0.0, -1.0, 0.0]; 4])
        }

        // Right
        if chunk.is_void(IVec3::new(lx + 1, ly, lz), world_chunks) {
            self.set_indices(vec![0, 3, 1, 1, 3, 2]);
            self.vertices.extend(
                vec!(
                    [wx + 0.5, wy + -0.5, wz + -0.5],
                    [wx + 0.5, wy + -0.5, wz + 0.5],
                    [wx + 0.5, wy + 0.5, wz + 0.5],
                    [wx + 0.5, wy + 0.5, wz + -0.5],
                )
            );

            self.normals.extend([[1.0, 0.0, 0.0]; 4])
        }

        // Left
        if chunk.is_void(IVec3::new(lx - 1, ly, lz), world_chunks) {
            self.set_indices(vec![0, 1, 3, 1, 2, 3]);
            self.vertices.extend(
                vec!(
                    [wx + -0.5, wy + -0.5, wz + -0.5],
                    [wx + -0.5, wy + -0.5, wz + 0.5],
                    [wx + -0.5, wy + 0.5, wz + 0.5],
                    [wx + -0.5, wy + 0.5, wz + -0.5],
                )
            );
            self.normals.extend([[-1.0, 0.0, 0.0]; 4])
        }

        // Behind
        if chunk.is_void(IVec3::new(lx, ly, lz + 1), world_chunks) {
            self.set_indices(vec![0, 3, 1, 1, 3, 2]);
            self.vertices.extend(
                vec!(
                    [wx + -0.5, wy + -0.5, wz + 0.5],
                    [wx + -0.5, wy + 0.5, wz + 0.5],
                    [wx + 0.5, wy + 0.5, wz + 0.5],
                    [wx + 0.5, wy + -0.5, wz + 0.5],
                )
            );
            self.normals.extend([[0.0, 0.0, 1.0]; 4])
        }

        // In front
        if chunk.is_void(IVec3::new(lx, ly, lz - 1), world_chunks) {
            self.set_indices(vec![0, 1, 3, 1, 2, 3]);
            self.vertices.extend(
                vec!(
                    [wx + -0.5, wy + -0.5, wz + -0.5],
                    [wx + -0.5, wy + 0.5, wz + -0.5],
                    [wx + 0.5, wy + 0.5, wz + -0.5],
                    [wx + 0.5, wy + -0.5, wz + -0.5],
                )
            );

            self.normals.extend([[1.0, 0.0, -1.0]; 4])
        }
    }

    fn set_indices(&mut self, indices: Vec<u32>) {
        let idx = self.vertices.len() as u32;
        let vec: Vec<_> = indices.into_iter()
            .map(|i| {
                //println!("{}", i);
                i + idx
            })
            .collect();

        self.indices.extend(vec);
    }
}