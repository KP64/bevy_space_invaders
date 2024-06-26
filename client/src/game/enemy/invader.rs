use super::{Enemy, PointsWorth};
use crate::game::{self, level, player, projectile};
use bevy::{app, prelude::*};
use bevy_rand::prelude::*;
use bevy_rapier2d::prelude::*;
use movement::Delay;
use std::{fmt, time::Duration};

mod movement;
pub mod shooting;

#[derive(Component, Clone, Copy)]
enum Type {
    Squid1,
    Squid2,
    Crab1,
    Crab2,
    Octopus1,
    Octopus2,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let itype = match self {
            Self::Squid1 => "squid_1",
            Self::Squid2 => "squid_2",
            Self::Crab1 => "crab_1",
            Self::Crab2 => "crab_2",
            Self::Octopus1 => "octopus_1",
            Self::Octopus2 => "octopus_2",
        };
        write!(f, "sprites/invaders/{itype}.png")
    }
}

impl Type {
    const fn next(self) -> Self {
        match self {
            Self::Squid1 => Self::Squid2,
            Self::Squid2 => Self::Squid1,
            Self::Crab1 => Self::Crab2,
            Self::Crab2 => Self::Crab1,
            Self::Octopus1 => Self::Octopus2,
            Self::Octopus2 => Self::Octopus1,
        }
    }
}

type Dimensions = Vec2;
type Points = usize;

const TYPES: [(Type, Dimensions, Points, Color); 3] = [
    (Type::Squid1, Vec2::new(16.0, 16.0), 30, Color::GREEN),
    (Type::Crab1, Vec2::new(22.0, 16.0), 20, Color::YELLOW),
    (Type::Octopus1, Vec2::new(24.0, 16.0), 10, Color::RED),
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
            .add_systems(
                Update,
                on_bottom_screen.run_if(in_state(game::State::Playing)),
            )
            .add_systems(OnEnter(game::State::GameOver), cleanup);
    }
}

fn cleanup(mut commands: Commands, invaders: Query<Entity, With<Invader>>) {
    for invader in &invaders {
        if let Some(mut invader) = commands.get_entity(invader) {
            invader.despawn();
        }
    }
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

        let group = (row_idx + 1).div_ceil(2) - 1; // 2 Rows == 1 Group
        let (invader_type, dimensions, points_worth, color) = TYPES[group];

        for (col_idx, column) in row.iter().enumerate() {
            commands.spawn((
                Name::new(format!("Invader {row_idx}:{col_idx}")),
                invader_type,
                Bundle::new(
                    PointsWorth(points_worth),
                    #[allow(clippy::cast_precision_loss)]
                    Delay(Duration::from_secs_f32(
                        0.1 * (row_idx + col_idx / 2) as f32,
                    )),
                    SpriteBundle {
                        texture: loader.load(invader_type.to_string()),
                        transform: Transform::from_xyz(column.x, row_y_offset, 0.0),
                        ..default()
                    },
                    Collider::cuboid(dimensions.x / 2.0, dimensions.y / 2.0),
                ),
                projectile::Color(color),
            ));
        }
    }
}

fn on_bottom_screen(
    (board, mut event): (Res<game::Board>, EventWriter<player::Death>),
    query: Query<&Transform, With<Invader>>,
) {
    let last_y_val = board.get_last_invader_y_cell();
    if query
        .iter()
        .any(|transform| transform.translation.y < last_y_val)
    {
        event.send(player::Death);
    }
}
