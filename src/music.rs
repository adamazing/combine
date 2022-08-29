use bevy_kira_audio::{Audio, AudioControl};
use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{statemanagement::{GameState, PauseState}, assets::AudioAssets};

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        debug!("Setting up MusicPlugin");
        app.add_enter_system(GameState::GamePlaying, start_gameplay_background_music)
            .add_exit_system(GameState::GamePlaying, stop_gameplay_background_music)
            .add_enter_system(PauseState::UnPaused, start_pause_menu_music)
            .add_exit_system(PauseState::Paused, stop_pause_menu_music);
    }
}

fn start_gameplay_background_music(audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    debug!("Playing music");
    audio.play(audio_assets.music.clone());
    audio.set_volume(0.4);
}

fn stop_gameplay_background_music(audio: Res<Audio>) {
    audio.stop();
}

fn start_pause_menu_music(audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    debug!("Playing pause menu music");
    audio.play(audio_assets.pause_music.clone());
    audio.set_volume(0.3);
}

fn stop_pause_menu_music(audio: Res<Audio>) {
    audio.stop();
}
