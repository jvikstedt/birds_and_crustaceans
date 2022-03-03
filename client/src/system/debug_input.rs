use bevy::{
    input::Input,
    prelude::{KeyCode, Query, Res},
};

use crate::component::DebugWindow;

pub fn debug_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut debug_window_q: Query<&mut DebugWindow>,
) {
    if let Ok(mut debug_window) = debug_window_q.get_single_mut() {
        if keyboard_input.just_released(KeyCode::F1) {
            debug_window.visible = !debug_window.visible;
        }
    }
}
