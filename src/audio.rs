use crate::source::AudioSource;
use bevy::prelude::Handle;
use parking_lot::RwLock;
use std::collections::VecDeque;

pub enum AudioCommands {
    Play(PlayAudioSettings),
    Stop,
}

pub struct PlayAudioSettings {
    pub source: Handle<AudioSource>,
    pub looped: bool,
}

#[derive(Default)]
pub struct Audio {
    pub commands: RwLock<VecDeque<AudioCommands>>,
}

impl Audio {
    pub fn play(&self, audio_source: Handle<AudioSource>) {
        self.commands
            .write()
            .push_front(AudioCommands::Play(PlayAudioSettings {
                source: audio_source,
                looped: false,
            }));
    }

    pub fn play_looped(&self, audio_source: Handle<AudioSource>) {
        self.commands
            .write()
            .push_front(AudioCommands::Play(PlayAudioSettings {
                source: audio_source,
                looped: true,
            }));
    }

    pub fn stop(&self) {
        self.commands.write().push_front(AudioCommands::Stop);
    }
}
