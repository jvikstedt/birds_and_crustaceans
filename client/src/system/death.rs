use bevy::prelude::{Commands, DespawnRecursiveExt, Entity, Query};

use crate::component::Health;

pub fn death(mut commands: Commands, health_q: Query<(Entity, &Health)>) {
    for (entity, health) in health_q.iter() {
        if health.current <= 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
