use bevy::{app, prelude::*};
use bevy_rapier2d::prelude::*;
use itertools::iproduct;

use crate::{
    asset_loader::TextureAssets,
    enemy::{self, Enemy},
    projectile::{self, Projectile},
    window,
};

const LENGTH: u8 = 24;
const HEIGHT: u8 = 16;
const HALF_LENGTH: u8 = LENGTH / 2;
const HALF_HEIGHT: u8 = HEIGHT / 2;
const SHOT_SPAWN_TIME_DURATION: f32 = 3.0;
const MOVE_TIME_DURATION: f32 = 5.0;

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (tick_spawn_shot, tick_movement, movement, shoot));
    }
}

#[derive(Component)]
pub struct Invader {
    pub color: Color,
}

impl Invader {
    pub const fn new(color: Color) -> Self {
        Self { color }
    }
}

#[derive(Component)]
struct ShotTimer(Timer);

impl Default for ShotTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(
            rand::random::<f32>().mul_add(10.0, SHOT_SPAWN_TIME_DURATION),
            TimerMode::Repeating,
        ))
    }
}

#[derive(Component)]
struct MoveTimer(Timer);

impl Default for MoveTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(
            MOVE_TIME_DURATION,
            TimerMode::Repeating,
        ))
    }
}

fn tick_spawn_shot(mut query: Query<&mut ShotTimer>, time: Res<Time>) {
    for mut invader in &mut query {
        invader.0.tick(time.delta());
    }
}

fn tick_movement(mut query: Query<&mut MoveTimer>, time: Res<Time>) {
    for mut invader in &mut query {
        invader.0.tick(time.delta());
    }
}

enum Direction {
    Left,
    Right,
    Down,
}

#[derive(Component, Default)]
struct MovementPointer(usize);

const MOVEMENT: [Direction; 6] = [
    Direction::Left,
    Direction::Down,
    Direction::Right,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

fn movement(
    mut query: Query<
        (
            &mut MovementPointer,
            &mut KinematicCharacterController,
            &MoveTimer,
        ),
        With<Invader>,
    >,
) {
    for (mut move_pointer, mut controller, _) in
        query.iter_mut().filter(|(_, _, timer)| timer.0.finished())
    {
        #[allow(clippy::cast_precision_loss)]
        let x_offset = f32::from(window::WIDTH) / enemy::COLUMNS.count() as f32;
        let x_offset = x_offset / 4.0;

        controller.translation = Some(match MOVEMENT[move_pointer.0] {
            Direction::Left => Vec2::new(-x_offset, 0.0),
            Direction::Right => Vec2::new(x_offset, 0.0),
            Direction::Down => Vec2::new(0.0, -Y_OFFSET / 4.0),
        });
        move_pointer.0 = (move_pointer.0 + 1) % MOVEMENT.len();
    }
}

pub const Y_OFFSET: f32 = window::HEIGHT as f32 / 11.0;
fn setup(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    #[allow(clippy::cast_precision_loss)]
    let x_offset = f32::from(window::WIDTH) / enemy::COLUMNS.count() as f32;

    for (col, row) in iproduct!(enemy::COLUMNS, enemy::ROWS.skip(1)) {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let grouping = (f32::from(row) / 3.0).floor() as u8;

        let (invader_type, points_worth, color): (usize, u8, Color) = match grouping {
            0 => (0, 30, Color::SEA_GREEN),
            1 => (2, 20, Color::YELLOW_GREEN),
            2 => (4, 10, Color::ORANGE_RED),
            _ => {
                warn!("Not Suitable Row Count for invader spawning");
                continue;
            }
        };

        let texture = texture_assets
            .invaders
            .get(invader_type)
            .unwrap_or_else(|| panic!("Could not get Enemy Texture at index {invader_type}"))
            .clone();

        commands.spawn((
            Name::new(format!("Enemy {col}:{row}")),
            Invader::new(color),
            Enemy,
            enemy::Points(points_worth.into()),
            ShotTimer::default(),
            MoveTimer::default(),
            MovementPointer::default(),
            SpriteBundle {
                texture,
                transform: Transform::from_xyz(
                    x_offset.mul_add(f32::from(col), -f32::from(window::HALF_WIDTH))
                        - x_offset / 2.0,
                    Y_OFFSET.mul_add(-f32::from(row), f32::from(window::HALF_HEIGHT)),
                    0.0,
                ),
                ..default()
            },
            RigidBody::KinematicVelocityBased,
            KinematicCharacterController {
                filter_groups: Some(CollisionGroups::new(Group::GROUP_2, Group::GROUP_3)),
                ..default()
            },
            Sensor,
            ActiveCollisionTypes::KINEMATIC_STATIC,
            ActiveEvents::COLLISION_EVENTS,
            Collider::cuboid(f32::from(HALF_LENGTH), f32::from(HALF_HEIGHT)),
        ));
    }
}

fn shoot(
    mut commands: Commands,
    texture_assets: Res<TextureAssets>,
    query: Query<(&Invader, &ShotTimer, &Transform)>,
) {
    const PROJECTILE_VELOCITY: Vec2 = Vec2::new(0.0, -450.0);

    for (invader, i_timer, transform) in &query {
        if !(i_timer.0.finished() && rand::random::<bool>()) {
            continue;
        }

        commands.spawn((
            Projectile::new(projectile::Direction::Down),
            SpriteBundle {
                texture: texture_assets.shots.enemy[0].clone(),
                transform: Transform::from_translation(transform.translation),
                sprite: Sprite {
                    color: invader.color,
                    ..default()
                },
                ..default()
            },
            RigidBody::KinematicVelocityBased,
            Sensor,
            ActiveCollisionTypes::KINEMATIC_KINEMATIC | ActiveCollisionTypes::KINEMATIC_STATIC,
            ActiveEvents::COLLISION_EVENTS,
            Collider::cuboid(projectile::HALF_LENGTH, projectile::HALF_HEIGHT),
            CollisionGroups::new(Group::GROUP_4, Group::GROUP_1),
            Velocity::linear(PROJECTILE_VELOCITY),
        ));
        break;
    }
}
