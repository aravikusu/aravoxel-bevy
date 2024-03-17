use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

#[derive(Resource)]
pub struct KeyBinds {
    pub move_forward: KeyCode,
    pub move_backward: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub move_ascend: KeyCode,
    pub move_descend: KeyCode,
    pub sprint: KeyCode,
    pub toggle_cursor: KeyCode,
}

impl Default for KeyBinds {
    fn default() -> Self {
        Self {
            move_forward: KeyCode::KeyW,
            move_backward: KeyCode::KeyS,
            move_left: KeyCode::KeyA,
            move_right: KeyCode::KeyD,
            move_ascend: KeyCode::Space,
            move_descend: KeyCode::ControlLeft,
            sprint: KeyCode::ShiftLeft,
            toggle_cursor: KeyCode::Escape,
        }
    }
}

#[derive(Resource)]
pub struct Settings {
    pub clown_vomit: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            clown_vomit: false,
        }
    }
}


pub struct GlobalPlugin;

impl Plugin for GlobalPlugin {
    fn build(&self,  app: &mut App) {
        app.insert_resource(KeyBinds::default())
            .add_systems(Update, grab_cursor);
    }
}

fn grab_cursor(
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
    key_binds: Res<KeyBinds>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        if keys.just_pressed(key_binds.toggle_cursor) {
            toggle_cursor(&mut window);
        }
    }
}

fn toggle_cursor(window: &mut Window) {
    match window.cursor.grab_mode {
        CursorGrabMode::None => {
            window.cursor.grab_mode = CursorGrabMode::Confined;
            window.cursor.visible = false;
        }
        _ => {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    }
}