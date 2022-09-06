use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::LdtkAsset;
use bevy_kira_audio::AudioSource;

use crate::statemanagement::GameState;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        info!("Loading assets");
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .with_collection::<LevelsAsset>()
                .with_collection::<AudioAssets>()
                .with_collection::<FontAssets>()
                .continue_to_state(GameState::GamePlaying),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/Baloo-Regular.ttf")]
    pub baloo: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/music/joshua-mclean_50s-bit.ogg")]
    pub game_music: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct LevelsAsset {
    #[asset(path = "levels/level01.ldtk")]
    pub map: Handle<LdtkAsset>,
}
