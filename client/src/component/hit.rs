use bevy::prelude::{Component, Entity};

#[derive(Component, Debug)]
pub struct Hit {
    pub parent: Entity,
    pub damage: i32,
}

impl Hit {
    pub fn new(parent: Entity, damage: i32) -> Self {
        Self { parent, damage }
    }
}
