#![allow(clippy::type_complexity,clippy::too_many_arguments)]
use bevy::{prelude::*, window::WindowMode};
pub use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_kira_audio::AudioPlugin;
use heron::prelude::*;
pub use iyes_loopless::prelude::*;

/** Global game constants */
pub const CLEAR: Color = Color::rgb(
    (148f32 / 256f32) * 1.0,
    (174f32 / 256f32) * 1.0,
    (214f32 / 256f32) * 1.0,
);
pub const LAUNCHER_TITLE: &str = "Combine";
pub const RATIO: f32 = 0.1;

/** Modules */
mod assets;
mod camera;
mod debug;
mod helpers;
mod level;
mod music;
mod npcs;
mod paused;
mod platforms;
mod player;
mod statemanagement;

use assets::AssetPlugin;
use camera::CameraPlugin;
use debug::DebugPlugin;
use level::LevelManagerPlugin;
use music::MusicPlugin;
use npcs::NpcPlugin;
use paused::PausePlugin;
use platforms::SurfacesPlugin;
use player::PlayerPlugin;
use statemanagement::{GameState, PauseState};

pub fn app() -> App {
    let mut app = App::new();
    app.add_loopless_state(GameState::Loading)
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
        .add_plugin(PhysicsPlugin::default())
        .insert_resource(Gravity::from(Vec3::new(0.0, -400., 0.0)))
        .add_plugin(AssetPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(MusicPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(NpcPlugin)
        .add_plugin(LevelManagerPlugin)
        .add_plugin(SurfacesPlugin) // Adds surfaces (platforms, walls, ladders)
        .add_plugin(PausePlugin)
        // .register_ldtk_entity::<MyBundle>("MyEntityIdentifier")
        .add_startup_system(setup);
    app
}

fn setup(mut _commands: Commands, _asset_server: Res<AssetServer>) {
    debug!("Setting up");
}
