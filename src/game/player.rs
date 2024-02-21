use super::cell;
use crate::{game, window};
use actions::Action;
use bevy::{app, prelude::*};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::InputManagerBundle;

pub mod actions;

const DIMENSIONS: Vec2 = Vec2::new(26.0, 16.0);

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Death>()
            .add_plugins(actions::Plugin)
            .add_systems(OnEnter(game::State::Setup), setup)
            .add_systems(Update, when_hit.run_if(in_state(game::State::Playing)))
            .add_systems(OnEnter(game::State::LvlFinished), reset_player);
    }
}

fn reset_player(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for player in &query {
        commands.entity(player).despawn();
    }
}

#[derive(Component)]
pub(super) struct Player;

#[derive(Bundle)]
struct Bundle {
    player: Player,
    sprite: SpriteBundle,
    input_manager: InputManagerBundle<Action>,
    rigidbody: RigidBody,
    velocity: Velocity,
    sensor: Sensor,
    collision_groups: CollisionGroups,
    active_collision_types: ActiveCollisionTypes,
    active_events: ActiveEvents,
    collider: Collider,
}

impl Bundle {
    fn new(sprite: SpriteBundle) -> Self {
        Self {
            player: Player,
            sprite,
            input_manager: InputManagerBundle {
                input_map: Action::default_input_map(),
                ..default()
            },
            rigidbody: RigidBody::KinematicVelocityBased,
            velocity: Velocity::zero(),
            sensor: Sensor,
            collision_groups: CollisionGroups::new(Group::GROUP_1, Group::GROUP_4),
            active_collision_types: ActiveCollisionTypes::KINEMATIC_KINEMATIC
                | ActiveCollisionTypes::KINEMATIC_STATIC,
            active_events: ActiveEvents::COLLISION_EVENTS,
            collider: Collider::cuboid(DIMENSIONS.x / 2.0, DIMENSIONS.y / 2.0),
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Name::new("Player"),
        Bundle::new(SpriteBundle {
            texture: asset_server.load("sprites/Player.png"),
            transform: Transform::from_xyz(
                0.0,
                -window::DIMENSIONS.y / 2.0 + cell::DIMENSIONS.y,
                0.0,
            ),
            ..default()
        }),
    ));
}

#[derive(Event)]
pub(super) struct Death;

fn when_hit(
    (mut death_event, mut game_state): (EventReader<Death>, ResMut<NextState<game::State>>),
) {
    for _ in death_event.read() {
        game_state.set(game::State::GameOver);
    }
}
