use bevy::{prelude::Component, reflect::Reflect};

#[derive(Component, Default, Reflect)]
pub struct Checksum {
    pub value: f64,
}
