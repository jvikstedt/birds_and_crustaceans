use bevy::prelude::*;
use shared::message::ClientState;

use crate::system::menu;

pub struct MenuPlugin {}

impl MenuPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(ClientState::MainMenu).with_system(menu::setup_menu),
        );
        app.add_system_set(
            SystemSet::on_exit(ClientState::MainMenu).with_system(menu::remove_menu),
        );
        app.add_system_set(
            SystemSet::on_update(ClientState::MainMenu).with_system(menu::button_handler),
        );
    }
}
