use bevy::prelude::{AssetServer, Res, ResMut};

use crate::resource::AudioHandles;

pub fn setup_audio_handles(
    asset_server: Res<AssetServer>,
    mut audio_handles: ResMut<AudioHandles>,
) {
    let background_handle = asset_server.load("music.ogg");

    audio_handles.add_audio_handle("background".to_string(), background_handle);
}
