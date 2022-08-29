use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, _app: &mut App) {
        info!("PlatformsPlugin");
    }
}

