use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_atmosphere::plugin::AtmospherePlugin;
use crate::actors::debug_camera::DebugCameraPlugin;
use crate::global::{GlobalPlugin, KeyBinds, Settings};
use crate::ui::debug::DebugUIPlugin;
use crate::voxel::plugin::VoxelWorldPlugin;

mod actors;
mod global;
mod voxel;
mod ui;
mod worldgen;

fn main() {
    App::new()
        .insert_resource(KeyBinds::default())
        .insert_resource(Settings::default())
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "aravoxel".into(),
                    name: Some("aravoxel.app".into()),
                    resolution: (1280., 720.).into(),
                    present_mode: PresentMode::AutoVsync,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    resizable: false,
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    visible: true,
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins((
            GlobalPlugin,
            DebugCameraPlugin,
            VoxelWorldPlugin,
            DebugUIPlugin,
            AtmospherePlugin
        ))
        .run();
}
