use bevy::{prelude::Component, reflect::Reflect};

#[derive(Default, Component, Reflect, Debug)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

impl Health {
    pub fn new(current: i32, max: i32) -> Self {
        Self { current, max }
    }
}

#[derive(Default, Component, Debug)]
pub struct HealthBar {}
