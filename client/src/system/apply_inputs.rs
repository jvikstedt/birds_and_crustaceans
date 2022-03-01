use bevy::prelude::{info, Query};
use shared::message::PlayerInput;

use crate::component::Collider;

pub fn apply_inputs(mut player_input_query: Query<(&PlayerInput, &mut Collider)>) {
    for (player_input, mut collider) in player_input_query.iter_mut() {
        collider.pos.x = player_input.mouse_x as f32;
        collider.pos.y = player_input.mouse_y as f32;

        if player_input.mouse_clicked {
            info!("player: clicked");
        }
    }
}
