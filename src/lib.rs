use bevy::{prelude::*, render::camera::{DepthCalculation, ScalingMode, WindowOrigin}, window::WindowMode};
use bevy_ecs_ldtk::prelude::*;
use bevy_kira_audio::AudioPlugin;
pub use iyes_loopless::prelude::*;
// use bevy_rapier2d::prelude::*;
// use bevy_kira_audio::*;
// use leafwing_input_manager::prelude::*;
pub use bevy_asset_loader::prelude::*;

/** Global game constants */
pub const CLEAR: Color = Color::rgb((148f32/256f32) *1.0, (174f32/256f32) * 1.0, (214f32/256f32) * 1.0);
pub const LAUNCHER_TITLE: &str = "Combine";
pub const RATIO: f32 = 0.1;

/** Modules */
mod assets;
mod debug;
mod statemanagement;
mod level;
mod player;
mod paused;
mod helpers;
mod music;
mod camera;

use assets::AssetPlugin;
use debug::DebugPlugin;
use music::MusicPlugin;
use paused::PausePlugin;
use camera::CameraPlugin;
use statemanagement::{GameState,PauseState};
use player::PlayerPlugin;

pub fn app() -> App {
    let mut app = App::new();
    app
    .add_loopless_state(GameState::Loading)
    .add_loopless_state(PauseState::UnPaused)
    .insert_resource(WindowDescriptor {
        title: LAUNCHER_TITLE.to_string(),
        canvas: Some("#bevy".to_string()),
        fit_canvas_to_parent: true,
        resizable: false,
        mode: WindowMode::BorderlessFullscreen,
        ..Default::default()
    })
    .insert_resource(ClearColor(CLEAR))
    .add_plugins(DefaultPlugins)
    .add_plugin(LdtkPlugin)
    .add_plugin(AssetPlugin)
    .add_plugin(AudioPlugin)
    .add_plugin(CameraPlugin)
    .add_plugin(DebugPlugin)
    .add_plugin(MusicPlugin)
    .add_plugin(PlayerPlugin)
    .add_plugin(PausePlugin)
    // .register_ldtk_entity::<MyBundle>("MyEntityIdentifier")
    .add_startup_system(setup)
    ;
    app
}

fn setup(mut _commands:Commands, _asset_server: Res<AssetServer> ) {
    debug!("Setting up");
}

