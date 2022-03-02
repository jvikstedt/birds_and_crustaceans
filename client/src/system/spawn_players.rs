use bevy::{
    math::Vec2,
    prelude::{info, AssetServer, Commands, Res, ResMut, SpriteBundle, Transform, Visibility},
    sprite::Sprite,
};
use glam::Vec3;
use shared::message::{Frame, Information, PlayerInput};

use crate::{
    component::{
        Checksum, Collider, Dynamic, EntityType, Myself, NetworkEntity, Player, PositionHistory,
    },
    resource::{NetworkIdProvider, Opt},
    rollback::{Rollback, RollbackIdProvider},
};

// Spaws players for the current frame
// This needs to be executed deterministically on all clients
// Though, timing when system is executed should not matter, because spawn is pushed to next frame
// anyway
pub fn spawn_players(
    mut commands: Commands,
    frame: Res<Frame>,
    mut rip: ResMut<RollbackIdProvider>,
    mut nip: ResMut<NetworkIdProvider>,
    information: Res<Information>,
    opt: Res<Opt>,
    asset_server: Res<AssetServer>,
) {
    for player_handle in frame.joined_players.iter() {
        info!("creating player: {:?}", player_handle);

        let myself = information.player_handle == *player_handle;

        let width = 29.;
        let height = 29.;

        let next_id = rip.next_id();

        let pos = Vec2::new(100., 50.);

        let mut entity_commands = commands.spawn();

        entity_commands
            .insert_bundle(SpriteBundle {
                texture: asset_server.load("pointer.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(width, height)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(pos.x, pos.y, 999.),
                    ..Default::default()
                },
                visibility: Visibility {
                    is_visible: !myself,
                },
                ..Default::default()
            })
            .insert(Player {
                handle: *player_handle,
            })
            .insert(Dynamic::default())
            .insert(PlayerInput::default())
            .insert(Collider {
                size: Vec2::new(width, height),
                pos,
                vel: Vec2::default(),
            })
            // Rollback component needs to be added for each entity that we wan't to track for
            // rollbacks
            .insert(NetworkEntity { id: nip.next_id() })
            .insert(Rollback::new(next_id))
            .insert(EntityType::Player)
            .insert(Checksum::default());

        if myself {
            entity_commands.insert(Myself);
            entity_commands.insert(PositionHistory::default());
        } else {
            entity_commands.insert(PositionHistory::new(opt.remote_player_delay));
        }
    }
}
