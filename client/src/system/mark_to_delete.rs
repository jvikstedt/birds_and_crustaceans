use bevy::prelude::{Commands, DespawnRecursiveExt, Entity, Query, With};

use crate::component::MarkToDelete;

pub fn mark_to_delete(mut commands: Commands, mark_to_delete_q: Query<Entity, With<MarkToDelete>>) {
    for entity in mark_to_delete_q.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
