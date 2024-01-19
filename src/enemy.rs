use std::ops::RangeInclusive;

use bevy::{app, prelude::*};
use itertools::iproduct;

use crate::{
    asset_loader::TextureAssets,
    projectile::{self, Direction, Projectile},
    score::Score,
    window,
};

const LENGTH: u8 = 24;
const HALF_LENGTH: f32 = (LENGTH / 2) as f32;
const HEIGHT: u8 = 16;
const HALF_HEIGHT: f32 = (HEIGHT / 2) as f32;

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup)
            .add_systems(Update, (check_hit, tick_shot_spawn_timer, shoot_projectile))
            .add_systems(Update, (tick_explosion_timer, despawn_explosion));
    }
}

#[derive(Component)]
struct Enemy {
    points_worth: u8,
    color: Color,
    timer: Timer,
}

fn setup(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    const SHOT_SPAWN_TIME: f32 = 3.0;

    const X_OFFSET: f32 = window::WIDTH as f32 / 11.0;
    const Y_OFFSET: f32 = window::HEIGHT as f32 / 11.0;
    const ENEMY_COLUMNS: RangeInclusive<u8> = 1..=10;
    const ENEMY_ROWS: RangeInclusive<u8> = 1..=5;

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
                    X_OFFSET.mul_add(f32::from(col), -f32::from(window::HALF_WIDTH)),
                    Y_OFFSET.mul_add(-f32::from(row), f32::from(window::HALF_HEIGHT)),
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
        ));
    }
}

fn tick_shot_spawn_timer(mut query: Query<&mut Enemy>, time: Res<Time>) {
    for mut enemy in &mut query {
        enemy.timer.tick(time.delta());
    }
}

fn shoot_projectile(
    mut commands: Commands,
    texture_assets: Res<TextureAssets>,
    query: Query<(&Enemy, &Transform)>,
) {
    const PROJECTILE_SPEED: f32 = 450.0;
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
            Projectile::new(Direction::Down, PROJECTILE_SPEED),
        ));
        break;
    }
}

fn check_hit(
    mut commands: Commands,
    (mut score, texture_assets): (ResMut<Score>, Res<TextureAssets>),
    (enemy_query, projectile_query): (
        Query<(Entity, &Transform, &Enemy)>,
        Query<(Entity, &Transform, &Projectile)>,
    ),
) {
    /* TODO: Change this number for each enemy Type */
    fn get_xy_translation<T>((ent, trans, t): (Entity, &Transform, T)) -> (Entity, Vec3, T) {
        (ent, trans.translation, t)
    }

    for (enemy_entity, enemy_translation, enemy) in enemy_query.iter().map(get_xy_translation) {
        for (proj_entity, proj_translation, _) in projectile_query
            .iter()
            .map(get_xy_translation)
            .filter(|(_, _, proj)| proj.direction != Direction::Down)
        {
            let enemy_range = (
                (enemy_translation.x - HALF_LENGTH)..=(enemy_translation.x + HALF_LENGTH),
                enemy_translation.y - HALF_HEIGHT..=(enemy_translation.y + HALF_HEIGHT),
            );

            let negative_pos = proj_translation.xy() - projectile::HALF_DIMENSIONS;
            let positive_pos = proj_translation.xy() + projectile::HALF_DIMENSIONS;

            if (enemy_range.0.contains(&negative_pos.x) || enemy_range.0.contains(&positive_pos.x))
                && (enemy_range.1.contains(&negative_pos.y)
                    || enemy_range.1.contains(&positive_pos.y))
            {
                /* TODO: Spawn Death Animation */
                score.0 += enemy.points_worth as usize;
                commands.entity(enemy_entity).despawn();
                commands.entity(proj_entity).despawn();

                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_translation(enemy_translation),
                        texture: texture_assets.explosions.enemy.clone(),
                        ..default()
                    },
                    Explosion::default(),
                ));
            }
        }
    }
}

#[derive(Component)]
struct Explosion {
    timer: Timer,
}

impl Default for Explosion {
    fn default() -> Self {
        const TIME_TILL_DESPAWN: f32 = 0.5;
        Self {
            timer: Timer::from_seconds(TIME_TILL_DESPAWN, TimerMode::Once),
        }
    }
}

fn tick_explosion_timer(mut query: Query<&mut Explosion>, time: Res<Time>) {
    for mut ex in &mut query {
        ex.timer.tick(time.delta());
    }
}

fn despawn_explosion(mut commands: Commands, query: Query<(Entity, &Explosion)>) {
    for (entity, _) in query
        .iter()
        .filter(|(_, explosion)| explosion.timer.finished())
    {
        commands.entity(entity).despawn();
    }
}
