use std::collections::HashMap;
use bevy::math::IVec3;
use crate::voxel::chunk::Chunk;
use crate::voxel::voxel::VoxelType;

pub const CHUNK_SIZE: i32 = 32;
pub const CHUNK_SIZE_F32: f32 = CHUNK_SIZE as f32;
pub const CHUNK_AREA: i32 = CHUNK_SIZE * CHUNK_SIZE;
pub const CHUNK_VOL: i32 = CHUNK_AREA * CHUNK_SIZE;

/// Since we often want to get the index of a voxel inside a chunk
/// we might as well have a helper function for it.
pub fn voxel_index(x: i32, y: i32, z: i32) -> usize {
    (x + CHUNK_SIZE * z + CHUNK_AREA * y) as usize
}

/// Ambient Occlusion.
/// We have to check every voxel surrounding the origin position.
pub fn get_ao(
    chunk: &Chunk,
    current_voxel_type: &VoxelType,
    origin_pos: IVec3,
    plane: IVec3,
    world_chunks: &HashMap<IVec3, Chunk>
) -> [u32; 4] {
    let x = origin_pos.x;
    let y = origin_pos.y;
    let z = origin_pos.z;

    match plane {
        IVec3::NEG_X => side_ao([
            !chunk.is_void(current_voxel_type, IVec3::new(x - 1, y, z - 1), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x - 1, y - 1, z - 1), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x - 1, y - 1, z), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x - 1, y - 1, z + 1), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x - 1, y, z + 1), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x - 1, y + 1, z + 1), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x - 1, y + 1, z), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x - 1, y + 1, z - 1), world_chunks)
        ]),
        IVec3::X => side_ao([
            !chunk.is_void(current_voxel_type, IVec3::new(x + 1, y, z - 1), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x + 1, y - 1, z - 1), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x + 1, y - 1, z), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x + 1, y - 1, z + 1), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x + 1, y, z + 1), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x + 1, y + 1, z + 1), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x + 1, y + 1, z), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x + 1, y + 1, z - 1), world_chunks)
        ]),
        IVec3::NEG_Y => side_ao([
            !chunk.is_void(current_voxel_type, IVec3::new(x, y - 1, z - 1), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x - 1, y - 1, z - 1), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x - 1, y - 1, z), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x - 1, y - 1, z + 1), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x, y - 1, z + 1), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x + 1, y - 1, z + 1), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x + 1, y - 1, z), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x + 1, y - 1, z - 1), world_chunks)
        ]),
        IVec3::Y => side_ao([
            !chunk.is_void(current_voxel_type, IVec3::new(x, y + 1, z - 1), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x - 1, y + 1, z - 1), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x - 1, y + 1, z), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x - 1, y + 1, z + 1), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x, y + 1, z + 1), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x + 1, y + 1, z + 1), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x + 1, y + 1, z), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x + 1, y + 1, z - 1), world_chunks)
        ]),
        IVec3::NEG_Z => side_ao([
            !chunk.is_void(current_voxel_type, IVec3::new(x - 1, y, z - 1), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x - 1, y - 1, z - 1), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x, y - 1, z - 1), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x + 1, y - 1, z - 1), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x + 1, y, z - 1), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x + 1, y + 1, z - 1), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x, y + 1, z - 1), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x - 1, y + 1, z - 1), world_chunks)
        ]),
        IVec3::Z => side_ao([
            !chunk.is_void(current_voxel_type, IVec3::new(x - 1, y, z + 1), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x - 1, y - 1, z + 1), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x, y - 1, z + 1), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x + 1, y - 1, z + 1), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x + 1, y, z + 1), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x + 1, y + 1, z + 1), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x, y + 1, z + 1), world_chunks),
            !chunk.is_void(current_voxel_type, IVec3::new(x - 1, y + 1, z + 1), world_chunks)
        ]),
        _ => unreachable!()
    }
}

fn side_ao(neighbours: [bool; 8]) -> [u32; 4] {
    [
        //ao_value(neighbours[0], neighbours[1], neighbours[2]),
        //ao_value(neighbours[2], neighbours[3], neighbours[4]),
        //ao_value(neighbours[6], neighbours[7], neighbours[0]),
        //ao_value(neighbours[4], neighbours[5], neighbours[6]),
        ao_value(neighbours[0], neighbours[1], neighbours[2]),
        ao_value(neighbours[6], neighbours[7], neighbours[0]),
        ao_value(neighbours[4], neighbours[5], neighbours[6]),
        ao_value(neighbours[2], neighbours[3], neighbours[4]),
    ]
}
fn ao_value(side1: bool, corner: bool, side2: bool) -> u32 {
    match (side1, corner, side2) {
        (true, _, true) => 0,
        (true, true, false) | (false, true, true) => 1,
        (false, false, false) => 3,
        _ => 2
    }
}