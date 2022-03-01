use bevy::prelude::*;

use crate::component::menu;

pub fn remove_menu(mut commands: Commands, menu_item_q: Query<Entity, With<menu::MenuItem>>) {
    for entity in menu_item_q.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
