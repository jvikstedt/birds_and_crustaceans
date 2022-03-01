use bevy::prelude::{info, Commands, DespawnRecursiveExt, Entity, Query, Res};
use shared::message::Frame;

use crate::component::Player;

// Remove all the players for the current frame
pub fn despawn_players(
    mut commands: Commands,
    frame: Res<Frame>,
    player_query: Query<(Entity, &Player)>,
) {
    for player_handle in frame.leaved_players.iter() {
        for (entity, player) in player_query.iter() {
            if *player_handle == player.handle {
                info!("removing player: {:?}", player_handle);
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
