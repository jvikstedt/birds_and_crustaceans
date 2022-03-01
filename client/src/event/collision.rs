use bevy::{math::Vec2, prelude::Entity};

use crate::component::EntityType;

#[derive(Debug)]
pub struct CollisionEvent {
    pub source: Entity,
    pub source_type: EntityType,
    pub target: Entity,
    pub target_type: EntityType,
    pub contact_normal: Vec2,
    pub is_solid: bool,
}
