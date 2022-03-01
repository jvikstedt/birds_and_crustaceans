use bevy::prelude::{In, ResMut};
use shared::{message::PlayerInput, PlayerHandle};

use crate::resource::MouseInfo;

// Returns the current frame inputs
// This system is called manually by the RollbackStage
#[allow(dead_code)]
pub fn input(_handle: In<PlayerHandle>, mut mouse_info: ResMut<MouseInfo>) -> PlayerInput {
    let mut input = PlayerInput::default();

    if mouse_info.clicked {
        input.mouse_clicked = true;
        mouse_info.clicked = false;
    }

    input.mouse_x = mouse_info.x;
    input.mouse_y = mouse_info.y;

    input
}
