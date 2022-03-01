use bevy::{math::Vec2, prelude::Component, reflect::Reflect};

#[derive(Default, Component, Reflect, Debug)]
#[cfg_attr(feature = "inspectable", derive(bevy_inspector_egui::Inspectable))]
pub struct Collider {
    pub size: Vec2,
    pub pos: Vec2,
    pub vel: Vec2,
}
