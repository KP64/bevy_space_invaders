use std::ops::RangeInclusive;

use bevy::{app, prelude::*};
use bevy_rapier2d::prelude::*;
use itertools::iproduct;

mod explosion;

use crate::{
    asset_loader::TextureAssets,
    projectile::{self, Projectile},
    score::Score,
    window,
};

const LENGTH: u8 = 24;
const HALF_LENGTH: u8 = LENGTH / 2;
const HEIGHT: u8 = 16;
const HALF_HEIGHT: u8 = HEIGHT / 2;

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(explosion::Plugin)
            .add_systems(PostStartup, setup)
            .add_systems(Update, (tick_spawn_shot, check_hit, shoot));
    }
}

#[derive(Component)]
pub struct Enemy {
    points_worth: u8,
    color: Color,
    timer: Timer,
}

fn setup(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    const SHOT_SPAWN_TIME: f32 = 3.0;

    const ENEMY_COLUMNS: RangeInclusive<u8> = 1..=10;
    const ENEMY_ROWS: RangeInclusive<u8> = 1..=5;
    let x_offset = f32::from(window::WIDTH) / ENEMY_COLUMNS.count() as f32;
    let y_offset = f32::from(window::HEIGHT) / 11.0;

    for (col, row) in iproduct!(ENEMY_COLUMNS, ENEMY_ROWS) {
        let (enemy_type, points_worth, color): (u8, u8, Color) = match row {
            1 => (0, 50, Color::SEA_GREEN),
            2 | 3 => (2, 25, Color::YELLOW_GREEN),
            4 | 5 => (4, 10, Color::ORANGE_RED),
            _ => unreachable!(),
        };

        let texture = texture_assets
            .enemies
            .get(enemy_type as usize)
            .unwrap_or_else(|| panic!("Could not get Enemy Texture at index {enemy_type}"))
            .clone();

        commands.spawn((
            SpriteBundle {
                texture,
                transform: Transform::from_xyz(
                    x_offset.mul_add(f32::from(col), -f32::from(window::HALF_WIDTH))
                        - x_offset / 2.0,
                    y_offset.mul_add(-f32::from(row), f32::from(window::HALF_HEIGHT)),
                    0.0,
                ),
                ..default()
            },
            Enemy {
                points_worth,
                color,
                timer: Timer::from_seconds(
                    rand::random::<f32>().mul_add(10.0, SHOT_SPAWN_TIME),
                    TimerMode::Repeating,
                ),
            },
            Name::new(format!("Enemy {col}:{row}")),
            RigidBody::Fixed,
            Sensor,
            CollisionGroups::new(Group::GROUP_2, Group::GROUP_3),
            ActiveCollisionTypes::KINEMATIC_STATIC,
            ActiveEvents::COLLISION_EVENTS,
            Collider::cuboid(f32::from(HALF_LENGTH), f32::from(HALF_HEIGHT)),
        ));
    }
}

fn tick_spawn_shot(mut query: Query<&mut Enemy>, time: Res<Time>) {
    for mut enemy in &mut query {
        enemy.timer.tick(time.delta());
    }
}

fn shoot(
    mut commands: Commands,
    texture_assets: Res<TextureAssets>,
    query: Query<(&Enemy, &Transform)>,
) {
    const PROJECTILE_VELOCITY: Vec2 = Vec2::new(0.0, -450.0);

    for (enemy, transform) in &query {
        if !(enemy.timer.finished() && rand::random::<bool>()) {
            continue;
        }

        commands.spawn((
            SpriteBundle {
                texture: texture_assets.shots.enemy[0].clone(),
                transform: Transform::from_translation(transform.translation),
                sprite: Sprite {
                    color: enemy.color,
                    ..default()
                },
                ..default()
            },
            Projectile {
                direction: projectile::Direction::Down,
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

fn check_hit(
    mut commands: Commands,
    (mut score, rapier_context): (ResMut<Score>, Res<RapierContext>),
    (enemy_query, projectile_query): (Query<(Entity, &Enemy)>, Query<(Entity, &Projectile)>),
) {
    for (e_entity, enemy) in &enemy_query {
        for (p_entity, _) in projectile_query
            .iter()
            .filter(|(_, p)| p.direction.is_upwards())
        {
            let Some(will_collide) = rapier_context.intersection_pair(e_entity, p_entity) else {
                continue;
            };
            if will_collide {
                score.0 += enemy.points_worth as usize;
                commands.entity(e_entity).despawn();
                commands.entity(p_entity).despawn();
            }
        }
    }
}
