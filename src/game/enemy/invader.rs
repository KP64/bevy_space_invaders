use super::{Enemy, PointsWorth};
use crate::game::{self, level};
use bevy::{app, prelude::*};
use bevy_rand::prelude::*;
use bevy_rapier2d::prelude::*;
use movement::Delay;

mod movement;
pub mod shooting;

type Type = &'static str;
type Dimensions = Vec2;
type Points = usize;

const TYPES: [(Type, Dimensions, Points); 3] = [
    ("squid", Vec2::new(16.0, 16.0), 30),
    ("crab", Vec2::new(22.0, 16.0), 20),
    ("octopus", Vec2::new(24.0, 16.0), 10),
];
const ROWS_TO_POPULATE: usize = 5;
const ROWS_TO_SKIP: usize = 2;

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((movement::Plugin, shooting::Plugin))
            .add_systems(
                OnEnter(game::State::LvlStartup),
                setup.run_if(level::Type::is_normal),
            )
            .add_systems(OnEnter(game::State::GameOver), cleanup);
    }
}

fn cleanup(mut commands: Commands, invaders: Query<Entity, With<Invader>>) {
    for invader in &invaders {
        commands.entity(invader).despawn_recursive();
    }
}

fn get_type(grouping: usize) -> (&'static str, Vec2, usize) {
    *TYPES
        .get(grouping)
        .unwrap_or_else(|| panic!("There is no Enemy Type No°{grouping}"))
}

#[derive(Bundle)]
struct Bundle {
    enemy: Enemy,
    invader: Invader,
    points: PointsWorth,
    delay: Delay,
    shooting_cooldown: shooting::Cooldown,
    shooting_entropy: EntropyComponent<ChaCha8Rng>,
    sprite: SpriteBundle,
    rigidbody: RigidBody,
    velocity: Velocity,
    sensor: Sensor,
    active_collision_types: ActiveCollisionTypes,
    active_events: ActiveEvents,
    collider: Collider,
    collision_groups: CollisionGroups,
}

impl Bundle {
    fn new(points: PointsWorth, delay: Delay, sprite: SpriteBundle, collider: Collider) -> Self {
        Self {
            enemy: Enemy,
            invader: Invader,
            points,
            delay,
            shooting_cooldown: shooting::Cooldown::default(),
            shooting_entropy: EntropyComponent::<ChaCha8Rng>::default(),
            sprite,
            rigidbody: RigidBody::KinematicPositionBased,
            velocity: Velocity::zero(),
            sensor: Sensor,
            active_collision_types: ActiveCollisionTypes::KINEMATIC_KINEMATIC
                | ActiveCollisionTypes::KINEMATIC_STATIC,
            active_events: ActiveEvents::COLLISION_EVENTS,
            collider,
            collision_groups: CollisionGroups::new(Group::GROUP_2, Group::GROUP_3),
        }
    }
}

#[derive(Component)]
struct Invader;

fn setup(mut commands: Commands, (game_board, loader): (Res<game::Board>, Res<AssetServer>)) {
    for (row_idx, row) in game_board
        .iter()
        .skip(ROWS_TO_SKIP)
        .take(ROWS_TO_POPULATE)
        .enumerate()
    {
        let row_y_offset = row
            .first()
            .unwrap_or_else(|| panic!("Could not get the first Cellposition of row {row_idx}"))
            .y;

        // TODO: Formula Not Accurate. Change to one that could never fail.
        let group = row_idx / TYPES.len() % 3;
        let (invader_type, dimensions, points_worth) = get_type(group);

        for (col_idx, column) in row.iter().enumerate() {
            commands.spawn((
                Name::new(format!("Invader {row_idx}:{col_idx}")),
                Bundle::new(
                    PointsWorth(points_worth),
                    // TODO: Find Formula or alternative Solution for chaotic movement.
                    // ? Top Left => Invader 0:0
                    // ? Bottom Right => Invader 4:10
                    Delay(0.05 * (row_idx + col_idx / 2) as f32),
                    SpriteBundle {
                        texture: loader.load(format!("sprites/invaders/{invader_type}_1.png")),
                        transform: Transform::from_xyz(column.x, row_y_offset, 0.0),
                        ..default()
                    },
                    Collider::cuboid(dimensions.x / 2.0, dimensions.y / 2.0),
                ),
            ));
        }
    }
}
