use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::player::ColliderBundle;

// use crate::player::{WallBundle, LadderBundle, InvisibleWallBundle};

pub struct SurfacesPlugin;

impl Plugin for SurfacesPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_int_cell::<WallBundle>(1)
            .register_ldtk_int_cell::<WallBundle>(6)
            .register_ldtk_int_cell::<InvisibleWallBundle>(3)
            // .register_ldtk_int_cell::<SpikeBundle>(4)
            .register_ldtk_int_cell::<LadderBundle>(5)
            .register_ldtk_int_cell::<LadderBundle>(7);
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct InvisibleWallBundle {
    wall: Wall,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Climbable;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct LadderBundle {
    #[from_int_grid_cell]
    #[bundle]
    pub collider_bundle: ColliderBundle,
    pub climbable: Climbable,
}
