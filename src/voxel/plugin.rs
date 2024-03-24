use bevy::pbr::{NotShadowCaster, NotShadowReceiver};
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use crate::global::Settings;
use crate::voxel::chunk_mesh::ChunkMesh;
use crate::worldgen::world::VoxelWorld;

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
        for y in -2..9 {
            for z in 0..9 {
                let chunk_pos = IVec3::new(x, y, z);
                voxel_world.generate_chunk(chunk_pos);
            }
        }
    }

    let texture: Handle<Image> = asset_server.load("voxel_atlas.png");
    for (_chunk_pos, chunk) in &voxel_world.chunks {
        let mut chunk_mesh = ChunkMesh::default();
        chunk_mesh.build_chunk_mesh(&chunk, &voxel_world.chunks, &settings);

        let mesh_handle = meshes.add(setup_bevy_mesh(chunk_mesh.mesh));

        commands.spawn(PbrBundle {
            mesh: mesh_handle,
            material: materials.add(StandardMaterial {
                base_color_texture: Some(texture.clone()),
                ..default()
            }),
            ..default()
        });

        if !chunk_mesh.liquid_mesh.vertices.is_empty() {
            let mesh_handle = meshes.add(setup_bevy_mesh(chunk_mesh.liquid_mesh));

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

    commands.insert_resource(AmbientLight {
        color: Color::rgb(0.98, 0.95, 0.82),
        brightness: 1000.0,
    });
}

fn setup_bevy_mesh(voxel_mesh: crate::voxel::mesh::Mesh) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, voxel_mesh.vertices.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, voxel_mesh.normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, voxel_mesh.uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, voxel_mesh.colors);

    mesh.insert_indices(Indices::U32(voxel_mesh.indices));

    mesh
}