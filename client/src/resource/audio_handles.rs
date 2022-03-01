use bevy::prelude::Handle;
use bevy_kira_audio::AudioSource;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct AudioHandles {
    handles: HashMap<String, Handle<AudioSource>>,
}

impl AudioHandles {
    pub fn add_audio_handle(&mut self, audio_key: String, handle: Handle<AudioSource>) {
        self.handles.insert(audio_key, handle);
    }

    pub fn get_audio_handle(&self, audio_key: &str) -> Option<Handle<AudioSource>> {
        if let Some(handle) = self.handles.get(audio_key) {
            return Some(handle.clone());
        }
        None
    }
}
