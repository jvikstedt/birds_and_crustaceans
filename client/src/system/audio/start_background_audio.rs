use bevy::prelude::Res;
use bevy_kira_audio::Audio;

use crate::resource::AudioHandles;

pub fn start_background_audio(audio: Res<Audio>, audio_handles: Res<AudioHandles>) {
    audio.play_looped(audio_handles.get_audio_handle("background").unwrap());
}
