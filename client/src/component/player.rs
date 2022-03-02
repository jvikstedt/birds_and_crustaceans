use bevy::{prelude::Component, reflect::Reflect};
use shared::PlayerHandle;

#[derive(Component, Default, Reflect, Debug)]
#[cfg_attr(feature = "inspectable", derive(bevy_inspector_egui::Inspectable))]
pub struct Player {
    pub handle: PlayerHandle,
    pub damage: i32,
    pub area: i32,
}
