use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::Inspectable;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<FrogBundle>("Frog")
            .register_ldtk_entity::<BatBundle>("Bat")
            .register_ldtk_entity::<SpiderBundle>("Spider");
    }
}

#[derive(Component, Default, Debug, Inspectable)]
pub struct Spider;

#[derive(Bundle, Component, Default, LdtkEntity)]
struct SpiderBundle {
    spider: Spider,

    #[bundle]
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
}

#[derive(Component, Default, Debug, Inspectable)]
pub struct Frog;

#[derive(Bundle, Component, Default, LdtkEntity)]
struct FrogBundle {
    frog: Frog,

    #[bundle]
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
}

#[derive(Component, Default, Debug, Inspectable)]
pub struct Bat;

#[derive(Bundle, Component, Default, LdtkEntity)]
struct BatBundle {
    bat: Bat,

    #[bundle]
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
}
