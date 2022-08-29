use std::collections::{HashMap, HashSet};

use bevy_inspector_egui::Inspectable;
use leafwing_input_manager::prelude::*;
use heron::prelude::*;

use bevy::prelude::*;
use iyes_loopless::prelude::ConditionSet;
use bevy_ecs_ldtk::prelude::*;
use crate::statemanagement::{GameState, PauseState};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        debug!("Setting up PlayerPlugin");
        app.add_plugin(InputManagerPlugin::<PlayerAction>::default())
            .add_system(pause_physics_during_load)
            .add_system(spawn_wall_collision)
            .add_system(movement)
            .add_system(spawn_ground_sensor)
            .add_system(ground_detection)
            .add_system(restart_level)
            .register_ldtk_entity::<PlayerBundle>("PlayerBlob")
            .register_ldtk_entity::<GoalBundle>("LevelGoal")
            .register_ldtk_entity::<FrogBundle>("Frog")
            .register_ldtk_entity::<BatBundle>("Bat")
            .register_ldtk_entity::<SpiderBundle>("Spider")
            .register_ldtk_entity::<InfoSignBundle>("InfoSign")
            // .register_ldtk_int_cell::<PlatformBundle>(1)
            .register_ldtk_int_cell::<InvisibleWallBundle>(3)
            // .register_ldtk_int_cell::<SpikeBundle>(4)
            .register_ldtk_int_cell::<LadderBundle>(5)
            .register_ldtk_int_cell::<WallBundle>(6)
            .register_ldtk_int_cell::<LadderBundle>(7)
            .add_system_set(
                ConditionSet::new()
                .run_in_state(GameState::GamePlaying)
                .run_not_in_state(PauseState::Paused)
                .into(),
                )
            ;
    }
}

#[derive(Component, Default, Debug, Inspectable)]
pub struct LevelGoal;

#[derive(Bundle, Component, Default, LdtkEntity)]
struct GoalBundle {
    goal: LevelGoal,

    #[bundle]
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
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
pub struct InfoSign;

#[derive(Bundle, Component, Default, LdtkEntity)]
struct InfoSignBundle {
    sign: InfoSign,

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


#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct InvisibleWallBundle {
    wall: Wall
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

#[derive(Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Climber {
    pub climbing: bool,
    pub intersecting_climbables: HashSet<Entity>,
}


#[derive(Component, Default, Debug, Inspectable)]
pub struct Player;

#[derive(Bundle, Default, LdtkEntity)]
struct PlayerBundle {
    player: Player,

    #[bundle]
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,

    #[bundle]
    #[from_entity_instance]
    collider_bundle: ColliderBundle,

    #[bundle]
    input_manager: PlayerInput,
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum PlayerAction {
    Up,
    Down,
    Left,
    Right,
    Jump
}

#[derive(Bundle)]
struct PlayerInput {
    #[bundle]
    input_manager: InputManagerBundle<PlayerAction>,
}

impl Default for PlayerInput {
    fn default() -> Self {
        use PlayerAction::*;

        Self {
            input_manager: InputManagerBundle::<PlayerAction> {
                input_map: InputMap::new([
                    (KeyCode::W, Up),
                    (KeyCode::S, Down),
                    (KeyCode::A, Left),
                    (KeyCode::D, Right),
                    (KeyCode::Up, Up),
                    (KeyCode::Down, Down),
                    (KeyCode::Left, Left),
                    (KeyCode::Right, Right),
                    (KeyCode::Space, Jump),
                ]),
                ..default()
            }
        }
    }
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: CollisionShape,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: RotationConstraints,
    pub physic_material: PhysicMaterial,
}

impl From<EntityInstance> for ColliderBundle {
    fn from(entity_instance: EntityInstance) -> ColliderBundle {
        let rotation_constraints = RotationConstraints::lock();

        match entity_instance.identifier.as_ref() {
            "PlayerBlob" => ColliderBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: Vec3::new(32., 32., 0.),
                    border_radius: Some(1.0),
                },
                rigid_body: RigidBody::Dynamic,
                rotation_constraints,
                ..Default::default()
            },
            "Frog" => ColliderBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: Vec3::new(5., 5., 0.),
                    border_radius: None,
                },
                rigid_body: RigidBody::KinematicVelocityBased,
                rotation_constraints,
                ..Default::default()
            },
            "LevelGoal" => ColliderBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: Vec3::new(8., 8., 0.),
                    border_radius: None,
                },
                rigid_body: RigidBody::Dynamic,
                rotation_constraints,
                physic_material: PhysicMaterial {
                    friction: 0.5,
                    density: 15.0,
                    ..Default::default()
                },
                ..Default::default()
            },
            _ => ColliderBundle::default(),
        }
    }
}

impl From<IntGridCell> for ColliderBundle {
    fn from(int_grid_cell: IntGridCell) -> ColliderBundle {
        let rotation_constraints = RotationConstraints::lock();

        if int_grid_cell.value == 1 || int_grid_cell.value == 7 {
            ColliderBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: Vec3::new(8., 8., 0.),
                    border_radius: None,
                },
                rigid_body: RigidBody::Sensor,
                rotation_constraints,
                ..Default::default()
            }
        } else {
            ColliderBundle::default()
        }
    }
}

#[derive(Clone, Default, Component)]
pub struct GroundDetection {
    pub on_ground: bool,
}

#[derive(Component)]
pub struct GroundSensor {
    pub ground_detection_entity: Entity,
    pub intersecting_ground_entities: HashSet<Entity>,
}

fn pause_physics_during_load(mut level_events: EventReader<LevelEvent>, mut physics_time: ResMut<PhysicsTime>){
    for event in level_events.iter() {
        match event {
            LevelEvent::SpawnTriggered(_) => physics_time.set_scale(0.),
            LevelEvent::Transformed(_) => physics_time.set_scale(1.),
            _ => (),
        }
    }
}


pub fn movement(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Climber, &GroundDetection), With<Player>>,
) {
    info!("Movement");
    for (mut velocity, mut climber, ground_detection) in query.iter_mut() {
        let right = if input.pressed(KeyCode::D) { 1. } else { 0. };
        let left = if input.pressed(KeyCode::A) { 1. } else { 0. };

        velocity.linear.x = (right - left) * 200.;

        if climber.intersecting_climbables.is_empty() {
            climber.climbing = false;
        } else if input.just_pressed(KeyCode::W) || input.just_pressed(KeyCode::S) {
            climber.climbing = true;
        }

        if climber.climbing {
            let up = if input.pressed(KeyCode::W) { 1. } else { 0. };
            let down = if input.pressed(KeyCode::S) { 1. } else { 0. };

            velocity.linear.y = (up - down) * 200.;
        }

        if input.just_pressed(KeyCode::Space) && (ground_detection.on_ground || climber.climbing) {
            velocity.linear.y = 450.;
            climber.climbing = false;
        }
    }
}


/// Spawns heron collisions for the walls of a level
///
/// You could just insert a ColliderBundle in to the WallBundle,
/// but this spawns a different collider for EVERY wall tile.
/// This approach leads to bad performance.
///
/// Instead, by flagging the wall tiles and spawning the collisions later,
/// we can minimize the amount of colliding entities.
///
/// The algorithm used here is a nice compromise between simplicity, speed,
/// and a small number of rectangle colliders.
/// In basic terms, it will:
/// 1. consider where the walls are
/// 2. combine wall tiles into flat "plates" in each individual row
/// 3. combine the plates into rectangles across multiple rows wherever possible
/// 4. spawn colliders for each rectangle
pub fn spawn_wall_collision(
    mut commands: Commands,
    wall_query: Query<(&GridCoords, &Parent), Added<Wall>>,
    parent_query: Query<&Parent, Without<Wall>>,
    level_query: Query<(Entity, &Handle<LdtkLevel>)>,
    levels: Res<Assets<LdtkLevel>>,
) {
    /// Represents a wide wall that is 1 tile tall
    /// Used to spawn wall collisions
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Plate {
        left: i32,
        right: i32,
    }

    /// A simple rectangle type representing a wall of any size
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Rect {
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
    }

    // Consider where the walls are
    // storing them as GridCoords in a HashSet for quick, easy lookup
    //
    // The key of this map will be the entity of the level the wall belongs to.
    // This has two consequences in the resulting collision entities:
    // 1. it forces the walls to be split along level boundaries
    // 2. it lets us easily add the collision entities as children of the appropriate level entity
    let mut level_to_wall_locations: HashMap<Entity, HashSet<GridCoords>> = HashMap::new();

    wall_query.for_each(|(&grid_coords, parent)| {
        // An intgrid tile's direct parent will be a layer entity, not the level entity
        // To get the level entity, you need the tile's grandparent.
        // This is where parent_query comes in.
        if let Ok(grandparent) = parent_query.get(parent.get()) {
            level_to_wall_locations
                .entry(grandparent.get())
                .or_insert(HashSet::new())
                .insert(grid_coords);
        }
    });

    if !wall_query.is_empty() {
        level_query.for_each(|(level_entity, level_handle)| {
            if let Some(level_walls) = level_to_wall_locations.get(&level_entity) {
                let level = levels
                    .get(level_handle)
                    .expect("Level should be loaded by this point");

                let LayerInstance {
                    c_wid: width,
                    c_hei: height,
                    grid_size,
                    ..
                } = level
                    .level
                    .layer_instances
                    .clone()
                    .expect("Level asset should have layers")[0];

                // combine wall tiles into flat "plates" in each individual row
                let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

                for y in 0..height {
                    let mut row_plates: Vec<Plate> = Vec::new();
                    let mut plate_start = None;

                    // + 1 to the width so the algorithm "terminates" plates that touch the right
                    // edge
                    for x in 0..width + 1 {
                        match (plate_start, level_walls.contains(&GridCoords { x, y })) {
                            (Some(s), false) => {
                                row_plates.push(Plate {
                                    left: s,
                                    right: x - 1,
                                });
                                plate_start = None;
                            }
                            (None, true) => plate_start = Some(x),
                            _ => (),
                        }
                    }

                    plate_stack.push(row_plates);
                }

                // combine "plates" into rectangles across multiple rows
                let mut wall_rects: Vec<Rect> = Vec::new();
                let mut previous_rects: HashMap<Plate, Rect> = HashMap::new();

                // an extra empty row so the algorithm "terminates" the rects that touch the top
                // edge
                plate_stack.push(Vec::new());

                for (y, row) in plate_stack.iter().enumerate() {
                    let mut current_rects: HashMap<Plate, Rect> = HashMap::new();
                    for plate in row {
                        if let Some(previous_rect) = previous_rects.remove(plate) {
                            current_rects.insert(
                                *plate,
                                Rect {
                                    top: previous_rect.top + 1,
                                    ..previous_rect
                                },
                            );
                        } else {
                            current_rects.insert(
                                *plate,
                                Rect {
                                    bottom: y as i32,
                                    top: y as i32,
                                    left: plate.left,
                                    right: plate.right,
                                },
                            );
                        }
                    }

                    // Any plates that weren't removed above have terminated
                    wall_rects.append(&mut previous_rects.values().copied().collect());
                    previous_rects = current_rects;
                }

                commands.entity(level_entity).with_children(|level| {
                    // Spawn colliders for every rectangle..
                    // Making the collider a child of the level serves two purposes:
                    // 1. Adjusts the transforms to be relative to the level for free
                    // 2. the colliders will be despawned automatically when levels unload
                    for wall_rect in wall_rects {
                        level
                            .spawn()
                            .insert(CollisionShape::Cuboid {
                                half_extends: Vec3::new(
                                    (wall_rect.right as f32 - wall_rect.left as f32 + 1.)
                                        * grid_size as f32
                                        / 2.,
                                    (wall_rect.top as f32 - wall_rect.bottom as f32 + 1.)
                                        * grid_size as f32
                                        / 2.,
                                    0.,
                                ),
                                border_radius: None,
                            })
                            .insert(RigidBody::Static)
                            .insert(PhysicMaterial {
                                friction: 0.1,
                                ..Default::default()
                            })
                            .insert(Transform::from_xyz(
                                (wall_rect.left + wall_rect.right + 1) as f32 * grid_size as f32
                                    / 2.,
                                (wall_rect.bottom + wall_rect.top + 1) as f32 * grid_size as f32
                                    / 2.,
                                0.,
                            ))
                            .insert(GlobalTransform::default());
                    }
                });
            }
        });
    }
}

pub fn spawn_ground_sensor(
    mut commands: Commands,
    detect_ground_for: Query<(Entity, &CollisionShape, &Transform), Added<GroundDetection>>,
) {
    for (entity, shape, transform) in detect_ground_for.iter() {
        if let CollisionShape::Cuboid { half_extends, .. } = shape {
            let detector_shape = CollisionShape::Cuboid {
                half_extends: Vec3::new(half_extends.x / 2., 2., 0.),
                border_radius: None,
            };

            let sensor_translation = Vec3::new(0., -half_extends.y, 0.) / transform.scale;

            commands.entity(entity).with_children(|builder| {
                builder
                    .spawn()
                    .insert(RigidBody::Sensor)
                    .insert(detector_shape)
                    .insert(Transform::from_translation(sensor_translation))
                    .insert(GlobalTransform::default())
                    .insert(GroundSensor {
                        ground_detection_entity: entity,
                        intersecting_ground_entities: HashSet::new(),
                    });
            });
        }
    }
}

pub fn ground_detection(
    mut ground_detectors: Query<&mut GroundDetection>,
    mut ground_sensors: Query<(Entity, &mut GroundSensor)>,
    mut collisions: EventReader<CollisionEvent>,
    rigid_bodies: Query<&RigidBody>,
) {
    for (entity, mut ground_sensor) in ground_sensors.iter_mut() {
        for collision in collisions.iter() {
            match collision {
                CollisionEvent::Started(a, b) => match rigid_bodies.get(b.rigid_body_entity()) {
                    Ok(RigidBody::Sensor) => {
                        // don't consider sensors to be "the ground"
                    }
                    Ok(_) => {
                        if a.rigid_body_entity() == entity {
                            ground_sensor
                                .intersecting_ground_entities
                                .insert(b.rigid_body_entity());
                        }
                    }
                    Err(_) => {
                        panic!("If there's a collision, there should be an entity")
                    }
                },
                CollisionEvent::Stopped(a, b) => {
                    if a.rigid_body_entity() == entity {
                        ground_sensor
                            .intersecting_ground_entities
                            .remove(&b.rigid_body_entity());
                    }
                }
            }
        }

        if let Ok(mut ground_detection) =
            ground_detectors.get_mut(ground_sensor.ground_detection_entity)
        {
            ground_detection.on_ground = ground_sensor.intersecting_ground_entities.len() > 0;
        }
    }
}

pub fn restart_level(
    mut commands: Commands,
    level_query: Query<Entity, With<Handle<LdtkLevel>>>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::R) {
        for level_entity in level_query.iter() {
            commands.entity(level_entity).insert(Respawn);
        }
    }
}

