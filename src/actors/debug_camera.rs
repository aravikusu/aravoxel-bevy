use bevy::ecs::event::ManualEventReader;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use crate::global::KeyBinds;

/// A plugin for the Debug Camera.
/// Fly around freely with no restrictions.
pub struct DebugCameraPlugin;

#[derive(Component)]
pub struct DebugCamera;

#[derive(Resource)]
pub struct DebugCameraSettings {
    speed: f32,
    sensitivity: f32,
}

#[derive(Resource, Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
}

impl Default for DebugCameraSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00012,
            speed: 12.0,
        }
    }
}

impl Plugin for DebugCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
            .init_resource::<DebugCameraSettings>()
            .add_systems(Startup, spawn_debug_camera)
            .add_systems(Update, (move_debug_camera, turn_camera));
    }
}

fn spawn_debug_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        DebugCamera,
    ));
}

/// Moving the camera. You know... so you can go somewhere.
fn move_debug_camera(
    mut query: Query<&mut Transform, With<DebugCamera>>,
    key_binds: Res<KeyBinds>,
    settings: Res<DebugCameraSettings>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    // Iterate through the query, even if there should only be one
    for mut transform in query.iter_mut() {
        let mut velocity = Vec3::ZERO;
        let local_z = transform.local_z();
        let forward = -Vec3::new(local_z.x, 0.0, local_z.z);
        let right = Vec3::new(local_z.z, 0.0, -local_z.x);

        if keys.any_pressed([key_binds.move_forward]) {
            velocity += forward;
        }

        if keys.any_pressed([key_binds.move_backward]) {
            velocity -= forward;
        }

        if keys.any_pressed([key_binds.move_left]) {
            velocity -= right;
        }

        if keys.any_pressed([key_binds.move_right]) {
            velocity += right;
        }

        if keys.any_pressed([key_binds.move_ascend]) {
            velocity += Vec3::Y;
        }

        if keys.any_pressed([key_binds.move_descend]) {
            velocity -= Vec3::Y;
        }

        velocity = velocity.normalize_or_zero();
        transform.translation += velocity * time.delta_seconds() * settings.speed;
    }
}

/// Turns the camera around when you move your mouse.
/// If the mouse is captured, that is.
fn turn_camera(
    settings: Res<DebugCameraSettings>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<&mut Transform, With<DebugCamera>>
) {
    // We get the window this time specifically to check if the mouse is grabbed
    if let Ok(window) = primary_window.get_single() {
        for mut transform in query.iter_mut() {
            for ev in state.reader_motion.read(&motion) {
                let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);

                match window.cursor.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        // Ensuring vertical/horizontal sensitivity is equal
                        let window_scale = window.height().min(window.width());
                        pitch -= (settings.sensitivity * ev.delta.y * window_scale).to_radians();
                        yaw -= (settings.sensitivity * ev.delta.x * window_scale).to_radians();
                    }
                }

                pitch = pitch.clamp(-1.54, 1.54);

                // Order is important since we don't want unintended rolling
                transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
            }
        }
    }
}