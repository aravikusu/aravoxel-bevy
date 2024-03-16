use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use crate::actors::debug_camera::DebugCamera;

/// Shows a bunch of debug information.
pub struct DebugUIPlugin;

impl Plugin for DebugUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EguiPlugin, FrameTimeDiagnosticsPlugin))
            .add_systems(Update, debug_ui);
    }
}

fn debug_ui(
    mut contexts: EguiContexts,
    diagnostics: Res<DiagnosticsStore>,
    mut camera_query: Query<&mut Transform, With<DebugCamera>>,
) {
    let mut camera_pos: Transform = Transform::default();

    // Get position of camera
    for transform in camera_query.iter_mut() {
        camera_pos = transform.clone();
    }

    let mut fps = 0.0;
    if let Some(fps_diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps_smooth) = fps_diagnostic.smoothed() {
            fps = fps_smooth;
        }
    }
    egui::Window::new("Debug Menu").show(contexts.ctx_mut(), |ui| {
        ui.heading("Player");
        ui.collapsing("Player Info", |ui| {
            ui.collapsing("Position", |ui| {
                ui.horizontal(|ui| {
                    ui.label("X:");
                    ui.label(format!("{}", f32::floor(camera_pos.translation.x)));
                });
                ui.horizontal(|ui| {
                    ui.label("Y:");
                    ui.label(format!("{}", f32::floor(camera_pos.translation.y)));
                });
                ui.horizontal(|ui| {
                    ui.label("Z:");
                    ui.label(format!("{}", f32::floor(camera_pos.translation.z)));
                });
            });
        });
        ui.heading("Voxel World");
        ui.collapsing("Current Chunk", |ui| {
            ui.horizontal(|ui| {
                ui.label("Visible Voxels:");
                ui.label("someday");
            });
            ui.collapsing("Position", |ui| {
                ui.horizontal(|ui| {
                    ui.label("X:");
                    ui.label(format!("{}", f32::floor(camera_pos.translation.x / 32.0)));
                });
                ui.horizontal(|ui| {
                    ui.label("Y:");
                    ui.label(format!("{}", f32::floor(camera_pos.translation.y / 32.0)));
                });
                ui.horizontal(|ui| {
                    ui.label("Z:");
                    ui.label(format!("{}", f32::floor(camera_pos.translation.z / 32.0)));
                });
            });
        });
        ui.heading("Other");
        ui.horizontal(|ui| {
            ui.label("FPS: ");
            ui.label(format!("{fps:.1}"));
        });
    });
}