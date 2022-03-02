use bevy::{
    prelude::{Commands, DespawnRecursiveExt, Entity, Query},
    sprite::Sprite,
};

use crate::component::{Dynamic, Enemy, EntityType, Health};

pub fn death(mut commands: Commands, health_q: Query<(Entity, &Health, Option<&Enemy>)>) {
    for (entity, health, enemy) in health_q.iter() {
        if health.current <= 0 {
            if enemy.is_some() {
                commands
                    .entity(entity)
                    .remove::<EntityType>()
                    .remove::<Dynamic>()
                    .remove::<Health>()
                    .remove::<Sprite>()
                    .despawn_descendants();
            } else {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
