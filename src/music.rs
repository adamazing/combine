use bevy_kira_audio::{Audio, AudioControl};
use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{statemanagement::GameState, assets::AudioAssets};

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        debug!("Setting up MusicPlugin");
        app.add_enter_system(GameState::GamePlaying, start_gameplay_background_music)
            .add_exit_system(GameState::GamePlaying, stop_gameplay_background_music);
    }
}

fn start_gameplay_background_music(audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    debug!("Playing music");
    audio.play(audio_assets.music.clone());
    audio.set_volume(0.2);
}

fn stop_gameplay_background_music(audio: Res<Audio>) {
    audio.stop();
}
