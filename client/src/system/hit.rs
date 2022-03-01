use bevy::prelude::{Commands, Entity, Query, ResMut};
use glam::Vec2;
use shared::message::PlayerInput;

use crate::{
    component::{Checksum, Collider, Dynamic, EntityType, Hit, MarkToDelete, NetworkEntity},
    resource::NetworkIdProvider,
    rollback::{Rollback, RollbackIdProvider},
};

pub fn hit(
    mut commands: Commands,
    players_q: Query<(Entity, &NetworkEntity, &PlayerInput, &Collider)>,
    mut rip: ResMut<RollbackIdProvider>,
    mut nip: ResMut<NetworkIdProvider>,
) {
    let mut players = players_q.iter().collect::<Vec<_>>();
    players.sort_by_key(|e| e.1.id);

    for (entity, _, player_input, collider) in players {
        if player_input.mouse_clicked {
            commands
                .spawn()
                .insert(Dynamic::default())
                .insert(Collider {
                    size: collider.size,
                    pos: collider.pos,
                    vel: Vec2::new(0.01, 0.01), // Move abit so collision system detects it collisions
                })
                .insert(EntityType::Hit)
                .insert(Hit::new(entity, 10))
                .insert(MarkToDelete::default())
                .insert(Checksum::default())
                .insert(NetworkEntity { id: nip.next_id() })
                .insert(Rollback::new(rip.next_id()));
        }
    }
}
