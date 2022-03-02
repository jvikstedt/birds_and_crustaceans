use bevy::{
    prelude::{Commands, DespawnRecursiveExt, Entity, Query},
    sprite::Sprite,
};

use crate::component::{Dynamic, EntityType, Health};

pub fn death(mut commands: Commands, mut health_q: Query<(Entity, &mut Health, &EntityType)>) {
    for (entity, mut health, entity_type) in health_q.iter_mut() {
        if health.current <= 0 {
            if let EntityType::Enemy = entity_type {
                commands
                    .entity(entity)
                    .remove::<EntityType>()
                    .remove::<Dynamic>()
                    .remove::<Health>()
                    .remove::<Sprite>()
                    .despawn_descendants();
            } else if let EntityType::Training = entity_type {
                health.current = health.max;
            } else {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
