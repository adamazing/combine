use iyes_loopless::prelude::*;

use bevy::{prelude::*, render::camera::{DepthCalculation, ScalingMode, WindowOrigin}};
use bevy_ecs_ldtk::{LevelSelection, LevelSpawnBehavior, SetClearColor, LdtkSettings, LdtkLevel};

use crate::{statemanagement::{GameState, PauseState}, player::Player};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelSelection::Uid(0))
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true
                },
                set_clear_color: SetClearColor::FromLevelBackground,
                ..Default::default()
            })
            .add_startup_system(setup_camera)
            .add_system(
                normalise_camera_within_level
                .run_in_state(GameState::GamePlaying)
                .run_not_in_state(PauseState::Paused)
                );
    }
}

fn setup_camera(mut commands: Commands) {
    debug!("Spawning camera");
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            depth_calculation: DepthCalculation::ZDifference,
            scaling_mode: ScalingMode::None,
            window_origin: WindowOrigin::BottomLeft,
            ..Default::default()
        },
        ..Default::default()
    });
}

fn normalise_camera_within_level(
    mut camera_query: Query<
        (
            &mut bevy::render::camera::OrthographicProjection,
            &mut Transform,
        ),
        (Without<Player>, With<Camera2d>),
    >,
    player_query: Query<&Transform, With<Player>>,
    level_query: Query<
        (&Transform, &Handle<LdtkLevel>),
        (Without<OrthographicProjection>, Without<Player>),
    >,
    level_selection: Res<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
    windows: Res<Windows>
) {
    debug!("Normalise camera in level");
    let window = windows.primary();
    let aspect_ratio:f32 = window.width() / window.height();

    if let Ok(Transform {
        translation: player_translation,
        ..
    }) = player_query.get_single()
    {
        let player_translation = *player_translation;

        let (mut orthographic_projection, mut camera_transform) = camera_query.single_mut();

        for (level_transform, level_handle) in &level_query {
            if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
                let level = &ldtk_level.level;
                if level_selection.is_match(&0, level) {
                    let level_ratio = level.px_wid as f32 / ldtk_level.level.px_hei as f32;

                    orthographic_projection.scaling_mode = bevy::render::camera::ScalingMode::None;
                    orthographic_projection.bottom = 0.;
                    orthographic_projection.left = 0.;
                    if level_ratio > aspect_ratio {
                        orthographic_projection.top = (level.px_hei as f32 / 9.).round() * 9.;
                        orthographic_projection.right = orthographic_projection.top * aspect_ratio;
                        camera_transform.translation.x = (player_translation.x
                            - level_transform.translation.x
                            - orthographic_projection.right / 2.)
                            .clamp(0., level.px_wid as f32 - orthographic_projection.right);
                        camera_transform.translation.y = 0.;
                    } else {
                        orthographic_projection.right = (level.px_wid as f32 / 16.).round() * 16.;
                        orthographic_projection.top = orthographic_projection.right / aspect_ratio;
                        camera_transform.translation.y = (player_translation.y
                            - level_transform.translation.y
                            - orthographic_projection.top / 2.)
                            .clamp(0., level.px_hei as f32 - orthographic_projection.top);
                        camera_transform.translation.x = 0.;
                    }

                    camera_transform.translation.x += level_transform.translation.x;
                    camera_transform.translation.y += level_transform.translation.y;
                }
            }
        }
    }

}
