use bevy::prelude::{Commands, Entity, Query, Res, ResMut, Without};
use bevy_kira_audio::Audio;

use crate::{
    component::Keep,
    resource::{NetworkIdProvider, RemoteFrames, Scores},
    rollback::{rollback_stage::StopRollbackStage, RollbackIdProvider},
};

pub fn reset_session(
    mut commands: Commands,
    mut remote_frames: ResMut<RemoteFrames>,
    audio: Res<Audio>,
    mut nip: ResMut<NetworkIdProvider>,
    mut rip: ResMut<RollbackIdProvider>,
    mut scores: ResMut<Scores>,
    query_all: Query<Entity, Without<Keep>>,
) {
    *remote_frames = RemoteFrames::new(1);
    commands.insert_resource(StopRollbackStage {});
    audio.stop();
    nip.reset();
    rip.reset();
    scores.reset();
    audio.set_volume(0.);

    for e in query_all.iter() {
        commands.entity(e).despawn();
    }
}
