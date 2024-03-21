use bevy::prelude::*;

#[derive(Debug, Clone)]
#[derive(PartialEq)]
pub enum VoxelType {
    AIR,
    GRASS,
    STONE,
    WATER,
}

impl VoxelType {
    /// If something is AIR, it's "invisible".
    pub fn is_visible(&self) -> bool {
        *self != VoxelType::AIR
    }

    pub fn is_liquid(&self) -> bool {
        *self == VoxelType::WATER
    }

    /// Some blocks are technically "visible"
    /// but should still have things render behind them.
    /// Things like WATER, etc.
    pub fn should_render(&self) -> bool {
        match *self {
            VoxelType::AIR => true,
            VoxelType::WATER => true,
            _ => false
        }
    }

    pub fn type_to_uvs(&self) -> Vec<[f32; 2]> {
        match *self {
            VoxelType::GRASS => vec![[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]],
            VoxelType::STONE => vec![[0.1, 0.0], [0.1, 0.0], [0.1, 0.0], [0.1, 0.0]],
            VoxelType::WATER => vec![[0.2, 0.0], [0.2, 0.0], [0.2, 0.0], [0.2, 0.0]],
            _ => unreachable!()
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct Voxel {
    pub local_position: IVec3,
    pub world_position: IVec3,
    pub voxel_type: VoxelType,
}

impl Default for Voxel {
    fn default() -> Self {
        Self {
            local_position: IVec3::ZERO,
            world_position: IVec3::ZERO,
            voxel_type: VoxelType::AIR,
        }
    }
}