use bevy::prelude::{Query, Res};
use shared::message::{Frame, PlayerInput};

use crate::component::Player;

// Takes all the input for the current frame and updates player entities inputs to match them
// This needs to be executed before inputs for the current frame are being executed
pub fn update_inputs(frame: Res<Frame>, mut player_query: Query<(&Player, &mut PlayerInput)>) {
    for (player, mut player_input) in player_query.iter_mut() {
        if let Some(input) = frame
            .inputs
            .iter()
            .find(|i| i.player_handle == player.handle)
        {
            *player_input = input.input;
        } else {
            player_input.mouse_clicked = false;
        }
    }
}
