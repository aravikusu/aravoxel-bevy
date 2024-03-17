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

    pub fn type_to_color(&self) -> [f32; 4] {
        match *self {
            VoxelType::STONE => [0.717, 0.710, 0.717, 1.0],
            VoxelType::GRASS => [0.194, 0.840, 0.160, 1.0],
            VoxelType::WATER => [0.385, 0.610, 0.770, 0.5],
            _ => [0.0, 0.0, 0.0, 0.0]
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