use bevy::prelude::Component;

#[derive(Component, Debug, Clone, Copy)]
#[cfg_attr(feature = "inspectable", derive(bevy_inspector_egui::Inspectable))]
pub enum EntityType {
    Player,
    Enemy,
    Hit,
    Wall,
}
