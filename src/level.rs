use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use iyes_loopless::prelude::*;

use crate::assets::*;
use crate::player::Player;
use crate::{GameState, PauseState};

pub struct LevelManagerPlugin;

impl Plugin for LevelManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::GamePlaying, spawn_level)
            .add_system(
                update_level_selection
                    .run_in_state(GameState::GamePlaying)
                    .run_not_in_state(PauseState::Paused),
            );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn spawn_level(mut commands: Commands, level: Res<LevelsAsset>) {
    debug!("Spawning level");

    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: level.map.clone(),
        ..default()
    });
}

fn update_level_selection(
    level_query: Query<(&Handle<LdtkLevel>, &Transform), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    mut level_selection: ResMut<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    for (level_handle, level_transform) in &level_query {
        debug!("{:?}", level_handle);
        if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
            debug!("Got a level");
            for player_transform in &player_query {
                if player_transform.translation.x
                    < level_transform.translation.x
                        + ldtk_level.level.px_wid as f32
                    && player_transform.translation.x
                        > level_transform.translation.x
                    && player_transform.translation.y
                        < level_transform.translation.y
                            + ldtk_level.level.px_hei as f32
                    && player_transform.translation.y
                        > level_transform.translation.y
                    && !level_selection.is_match(&0, &ldtk_level.level)
                {
                    *level_selection =
                        LevelSelection::Iid(ldtk_level.level.iid.clone());
                }
            }
        }
    }
}
