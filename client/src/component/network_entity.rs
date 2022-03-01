use bevy::prelude::Component;

#[derive(Component, Debug)]
#[cfg_attr(feature = "inspectable", derive(bevy_inspector_egui::Inspectable))]
pub struct NetworkEntity {
    pub id: u32,
}
