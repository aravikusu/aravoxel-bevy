use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Voxel {
    pub is_solid: bool,
}