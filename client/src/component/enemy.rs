use bevy::prelude::Component;

#[derive(Debug, Clone, Copy)]
pub enum EnemyType {
    Crustacean,
    Bird,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub x_speed: f32,
    pub y_speed: f32,
}
