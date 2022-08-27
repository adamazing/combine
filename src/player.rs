use leafwing_input_manager::prelude::*;
use bevy_rapier2d::prelude::{Damping, Friction, LockedAxes, GravityScale, Velocity, RigidBody, Collider};

use bevy::prelude::*;
use iyes_loopless::prelude::ConditionSet;
use bevy_ecs_ldtk::prelude::{LdtkEntity,RegisterLdtkObjects};
use crate::statemanagement::{GameState, PauseState};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        debug!("Setting up PlayerPlugin");
        app.add_plugin(InputManagerPlugin::<PlayerAction>::default())
            .register_ldtk_entity::<PlayerBundle>("PlayerBlob")
            .add_system_set(
                ConditionSet::new()
                .run_in_state(GameState::GamePlaying)
                .run_not_in_state(PauseState::Paused)
                .with_system(player_movement)
                .with_system(player_animation)
                .into(),
                );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

#[derive(Component, Default, Debug)]
pub struct Player;

#[derive(Bundle, Default, LdtkEntity)]
struct PlayerBundle {
    player: Player,

    #[bundle]
    collider: PlayerColliderBundle,

    #[bundle]
    input_manager: PlayerInput,
}

#[derive(Bundle)]
struct PlayerColliderBundle {
    collider: Collider,
    rigid_body: RigidBody,
    velocity: Velocity,
    gravity_scale: GravityScale,
    locked_axis: LockedAxes,
    friction: Friction,
    dampening: Damping,
}

impl Default for PlayerColliderBundle {
    fn default() -> Self {
        Self {
            collider: Collider::cuboid(10.0, 10.0),
            rigid_body: RigidBody::Dynamic,
            velocity: Velocity::zero(),
            gravity_scale: GravityScale(0.0),
            locked_axis: LockedAxes::ROTATION_LOCKED,
            friction: Friction::coefficient(0.0),
            dampening: Damping {
                linear_damping: 40.0,
                angular_damping: 0.0,
            },
        }
    }
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

fn player_movement(mut player_query: Query<(&mut Velocity, &ActionState<PlayerAction>),
                   (With<Player>, Changed<ActionState<PlayerAction>>),>) {
    debug!("Player movement");
    for (mut vel, action_state) in &mut player_query {
        if action_state.pressed(PlayerAction::Left) {
            vel.linvel.x = -100.0;
        }
        if action_state.pressed(PlayerAction::Right) {
            vel.linvel.x = 100.0;
        }
    }
}

struct AnimationTimer(Timer);

impl Default for AnimationTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.2, false))
    }
}

fn player_animation(mut _player_query: Query<
                    (&mut TextureAtlasSprite, &ActionState<PlayerAction>),
                    With<Player>>,
                    mut _timer: Local<AnimationTimer>,
                    _time: Res<Time>) {
    info!("Animation not implemented");
}



