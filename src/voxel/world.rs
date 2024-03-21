use std::collections::HashMap;
use bevy::pbr::{NotShadowCaster, NotShadowReceiver};
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy_atmosphere::prelude::*;
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
    asset_server: Res<AssetServer>,
    mut voxel_world: ResMut<VoxelWorld>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    settings: Res<Settings>
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

    let texture: Handle<Image> = asset_server.load("voxel_atlas.png");
    for (_chunk_pos, chunk) in &voxel_world.chunks {
        let mut chunk_mesh = ChunkMesh::default();
        chunk_mesh.build_chunk_mesh(&chunk, &voxel_world.chunks, &settings);

        let mesh_handle = meshes.add(setup_bevy_mesh(chunk_mesh.mesh, false));

        commands.spawn(PbrBundle {
            mesh: mesh_handle,
            material: materials.add(StandardMaterial {
                base_color_texture: Some(texture.clone()),
                ..default()
            }),
            ..default()
        });

        if !chunk_mesh.liquid_mesh.vertices.is_empty() {
            let mesh_handle = meshes.add(setup_bevy_mesh(chunk_mesh.liquid_mesh, true));

            commands.spawn((PbrBundle {
                mesh: mesh_handle,
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(texture.clone()),
                    alpha_mode: AlphaMode::Add,
                    cull_mode: None,
                    ..default()
                }),
                ..default()
            }, NotShadowReceiver, NotShadowCaster));
        }
    }

    commands.spawn(DirectionalLightBundle {
       directional_light: DirectionalLight {
           color: Color::rgb(0.98, 0.95, 0.82),
           shadows_enabled: true,
           ..default()
       },
       transform: Transform::from_xyz(0.0, 0.0, 0.0)
           .looking_at(Vec3::new(-0.15, -5.05, 0.25), Vec3::Y),
       ..default()
    });


}

fn setup_bevy_mesh(voxel_mesh: crate::voxel::mesh::Mesh, liquid: bool) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, voxel_mesh.vertices.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, voxel_mesh.normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, voxel_mesh.uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, voxel_mesh.colors);

    mesh.insert_indices(Indices::U32(voxel_mesh.indices));

    mesh
}