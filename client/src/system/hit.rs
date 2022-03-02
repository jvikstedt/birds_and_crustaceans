use bevy::prelude::{Commands, Entity, Query, ResMut};
use glam::Vec2;
use shared::message::PlayerInput;

use crate::{
    component::{
        Checksum, Collider, Dynamic, EntityType, Hit, MarkToDelete, NetworkEntity, Player,
    },
    resource::NetworkIdProvider,
    rollback::{Rollback, RollbackIdProvider},
};

pub fn hit(
    mut commands: Commands,
    players_q: Query<(Entity, &NetworkEntity, &PlayerInput, &Collider, &Player)>,
    mut rip: ResMut<RollbackIdProvider>,
    mut nip: ResMut<NetworkIdProvider>,
) {
    let mut players = players_q.iter().collect::<Vec<_>>();
    players.sort_by_key(|e| e.1.id);

    for (entity, _, player_input, collider, player) in players {
        if player_input.mouse_clicked {
            commands
                .spawn()
                .insert(Dynamic::default())
                .insert(Collider {
                    size: Vec2::new(player.area as f32, player.area as f32),
                    pos: collider.pos,
                    vel: Vec2::new(1., 1.), // Move abit so collision system detects it collisions
                })
                .insert(EntityType::Hit)
                .insert(Hit::new(entity, player.damage))
                .insert(MarkToDelete::default())
                .insert(Checksum::default())
                .insert(NetworkEntity { id: nip.next_id() })
                .insert(Rollback::new(rip.next_id()));
        }
    }
}
