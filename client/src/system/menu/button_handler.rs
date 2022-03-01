use bevy::prelude::*;
use shared::message::ClientState;

use crate::component::menu;

pub fn button_handler(
    interaction_query: Query<
        (&Interaction, &menu::ButtonType),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_state: ResMut<State<ClientState>>,
) {
    for (interaction, button_type) in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            match button_type {
                menu::ButtonType::StartGame => {
                    app_state.set(ClientState::Connecting).unwrap();
                }
            }
        }
    }
}
