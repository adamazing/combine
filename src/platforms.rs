use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::statemanagement::GameState;


pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, _app: &mut App) {
        info!("PlatformsPlugin");
    }
}

#[derive(Bundle)]
struct PlatformBundle {
    platform: Platform,
}

#[derive(Component, Default)]
struct Platform;
