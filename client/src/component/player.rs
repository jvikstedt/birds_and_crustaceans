use bevy::prelude::Component;
use shared::PlayerHandle;

#[derive(Component, Debug)]
#[cfg_attr(feature = "inspectable", derive(bevy_inspector_egui::Inspectable))]
pub struct Player {
    pub handle: PlayerHandle,
}
