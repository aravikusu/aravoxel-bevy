use std::collections::HashMap;
use std::f32::consts::PI;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology, VertexAttributeValues};
use bevy::render::render_asset::RenderAssetUsages;
use crate::global::Settings;
use crate::voxel::chunk::Chunk;
use crate::voxel::chunk_mesh::ChunkMesh;

#[derive(Resource)]
pub struct VoxelWorld {
    chunks: HashMap<IVec3, Chunk>,
    //meshes: HashMap<IVec3, ChunkMesh>
}

impl Default for VoxelWorld {
    fn default() -> Self {
        Self {
            chunks: HashMap::new(),
            //meshes: HashMap::new(),
        }
    }
}

impl VoxelWorld {
    pub fn insert_chunk(&mut self, position: IVec3, chunk: Chunk) -> &Chunk {
        self.chunks.insert(position, chunk);

        self.chunks.get(&position).unwrap()
    }
}

/// Handles the logic and all the fun things relating
/// to our voxel world.
pub struct VoxelWorldPlugin;

impl Plugin for VoxelWorldPlugin {
    fn build(&self,  app: &mut App) {
        app.insert_resource(VoxelWorld::default())
            .add_systems(Startup, setup_world);
    }
}

fn setup_world(
    mut commands: Commands,
    mut voxel_world: ResMut<VoxelWorld>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut settings: ResMut<Settings>
) {
    for x in 0..9 {
        for y in 0..9 {
            for z in 0..9 {
                let chunk_pos = IVec3::new(x, y, z);
                let mut chunk = Chunk::new(chunk_pos);
                chunk.generate();

                voxel_world.chunks.insert(chunk_pos, chunk.clone());
            }
        }
    }

    for (_chunk_pos, chunk) in &voxel_world.chunks {
        let mut chunk_mesh = ChunkMesh::default();
        chunk_mesh.build_chunk_mesh(&chunk, &voxel_world.chunks, &settings);

        let mesh_handle = meshes.add(setup_bevy_mesh(chunk_mesh));

        commands.spawn(PbrBundle {
            mesh: mesh_handle,
            material: materials.add(StandardMaterial { ..default() }),
            ..default()
        });
    }

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });

}

fn setup_bevy_mesh(chunk_mesh: ChunkMesh) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, chunk_mesh.vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, chunk_mesh.normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, chunk_mesh.colors);

    mesh.insert_indices(Indices::U32(chunk_mesh.indices));

    mesh
}