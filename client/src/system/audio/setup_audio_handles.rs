use bevy::prelude::{AssetServer, Res, ResMut};

use crate::resource::AudioHandles;

pub fn setup_audio_handles(
    asset_server: Res<AssetServer>,
    mut audio_handles: ResMut<AudioHandles>,
) {
    let background_handle = asset_server.load("music.ogg");
    let click_handle = asset_server.load("click.ogg");
    let powerup_handle = asset_server.load("powerup.ogg");
    let bird_handle = asset_server.load("bird.ogg");
    let frog_handle = asset_server.load("frog.ogg");

    audio_handles.add_audio_handle("background".to_string(), background_handle);
    audio_handles.add_audio_handle("click".to_string(), click_handle);
    audio_handles.add_audio_handle("powerup".to_string(), powerup_handle);
    audio_handles.add_audio_handle("bird".to_string(), bird_handle);
    audio_handles.add_audio_handle("frog".to_string(), frog_handle);
}
