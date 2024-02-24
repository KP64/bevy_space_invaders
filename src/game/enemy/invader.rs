use super::{Enemy, PointsWorth};
use crate::game::{self, level};
use bevy::{app, prelude::*};
use bevy_rand::prelude::*;
use bevy_rapier2d::prelude::*;

mod movement;
pub mod shooting;

type Type = char;
type Dimensions = Vec2;
type Points = usize;

const TYPES: [(Type, Dimensions, Points); 3] = [
    ('A', Vec2::new(16.0, 16.0), 30),
    ('B', Vec2::new(22.0, 16.0), 20),
    ('C', Vec2::new(24.0, 16.0), 10),
];
const ROWS_TO_POPULATE: usize = 5;
const ROWS_TO_SKIP: usize = 2;

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((movement::Plugin, shooting::Plugin))
            .add_systems(
                OnEnter(game::State::Setup),
                setup.run_if(level::Type::is_normal),
            )
            .add_systems(OnEnter(game::State::LvlFinished), cleanup);
    }
}

fn get_type(grouping: usize) -> (char, Vec2, usize) {
    *TYPES
        .get(grouping)
        .unwrap_or_else(|| panic!("There is no Enemy Type No°{grouping}"))
}

#[derive(Component)]
struct Row;

#[derive(Bundle)]
struct Bundle {
    enemy: Enemy,
    points: PointsWorth,
    shooting_timer: shooting::Timer,
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
    fn new(points: PointsWorth, sprite: SpriteBundle, collider: Collider) -> Self {
        Self {
            enemy: Enemy,
            points,
            shooting_timer: shooting::Timer::default(),
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

fn setup(mut commands: Commands, (game_board, loader): (Res<game::Board>, Res<AssetServer>)) {
    for (row_idx, row) in game_board
        .iter()
        .enumerate()
        .skip(ROWS_TO_SKIP)
        .take(ROWS_TO_POPULATE)
    {
        let row_y_offset = row
            .first()
            .unwrap_or_else(|| panic!("Could not get the first Cellposition of row {row_idx}"))
            .y;
        let mut entity = commands.spawn((
            Name::new(format!("InvaderRow {row_idx}")),
            Row,
            SpatialBundle::from_transform(Transform::from_xyz(0.0, row_y_offset, 0.0)),
        ));

        // TODO: Formula Not Accurate. Change to one that could never fail.
        let group = row_idx / TYPES.len();
        let (invader_type, dimensions, points_worth) = get_type(group);

        for column in row {
            entity.with_children(|parent| {
                parent.spawn(Bundle::new(
                    PointsWorth(points_worth),
                    SpriteBundle {
                        texture: loader.load(format!("sprites/invaders/{invader_type}1.png")),
                        transform: Transform::from_xyz(column.x, 0.0, 0.0),
                        ..default()
                    },
                    Collider::cuboid(dimensions.x / 2.0, dimensions.y / 2.0),
                ));
            });
        }
    }
}

fn cleanup(mut commands: Commands, rows: Query<Entity, With<Row>>) {
    for row in &rows {
        commands.entity(row).despawn();
    }
}
