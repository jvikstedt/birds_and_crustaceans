use bevy::prelude::Component;

#[derive(Default, Component, Debug)]
pub struct Training {
    pub damage_increase: i32,
    pub area_increase: i32,
}
