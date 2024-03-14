pub const CHUNK_SIZE: i32 = 32;
pub const CHUNK_SIZE_F32: f32 = CHUNK_SIZE as f32;
pub const CHUNK_AREA: i32 = CHUNK_SIZE * CHUNK_SIZE;
pub const CHUNK_VOL: i32 = CHUNK_AREA * CHUNK_SIZE;

/// Since we often want to get the index of a voxel inside a chunk
/// we might as well have a helper function for it.
pub fn voxel_index(x: i32, y: i32, z: i32) -> usize {
    (x + CHUNK_SIZE * z + CHUNK_AREA * y) as usize
}