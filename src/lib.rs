use bevy::{prelude::*, render::camera::{DepthCalculation, ScalingMode, WindowOrigin}, window::WindowMode};
use bevy_ecs_ldtk::prelude::*;

/** Global game constants */
pub const CLEAR: Color = Color::rgb(0.2,0.1,0.1);
pub const LAUNCHER_TITLE: &str = "Combine";
pub const RATIO: f32 = 0.1;

/** Modules */
mod debug;

use debug::DebugPlugin;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    MainMenu,
    GameLoading,
    GamePlaying,
    GameOver,
    GamePaused,
}

pub fn app() -> App {
    let mut app = App::new();
    app
    .add_state(GameState::MainMenu)
    .add_plugins(DefaultPlugins)
    .add_plugin(LdtkPlugin)
    .add_plugin(DebugPlugin)
    .insert_resource(LevelSelection::Index(0))
    .insert_resource(ClearColor(CLEAR))
    .insert_resource(WindowDescriptor {
        title: LAUNCHER_TITLE.to_string(),
        canvas: Some("#bevy".to_string()),
        fit_canvas_to_parent: true,
        resizable: false,

        mode: WindowMode::Fullscreen,
        ..Default::default()
    })
    // .register_ldtk_entity::<MyBundle>("MyEntityIdentifier")
    .add_startup_system(setup);
    app
}

fn setup(mut commands:Commands, asset_server: Res<AssetServer>) {
    let mut camera_bundle = commands.spawn_bundle(Camera2dBundle{
       projection: OrthographicProjection{
        depth_calculation: DepthCalculation::ZDifference,
        scaling_mode: ScalingMode::WindowSize,
        window_origin: WindowOrigin::BottomLeft,
        ..Default::default()
       },
       ..Default::default()
    });


    camera_bundle.log_components();
    // camera_bundle. = OrthographicProjection {
    //     far,
    //     depth_calculation: DepthCalculation::ZDifference,
    //     scaling_mode: ScalingMode::WindowSize,
    //     window_origin: WindowOrigin::BottomLeft,
    //     ..Default::default()
    // };

    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels/level01.ldtk"),
        ..Default::default()
    });
}

// #[derive(Bundle, LdtkEntity)]
// pub struct MyBundle {
//     a: ComponentA,
//     b: ComponentB,
//     #[sprite_sheet_bundle]
//     #[bundle]
//     sprite_bundle: SpriteSheetBundle,
// }
