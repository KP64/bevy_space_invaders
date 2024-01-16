use std::ops::RangeInclusive;

use bevy::{app, prelude::*};
use itertools::iproduct;

use crate::{
    projectile::{self, Projectile},
    window,
};

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_enemies)
            .add_systems(Update, get_hit);
    }
}

#[derive(Component)]
struct Enemy;

fn setup_enemies(mut commands: Commands, asset_server: Res<AssetServer>) {
    const X_OFFSET: f32 = window::WIDTH as f32 / 11.0;
    const Y_OFFSET: f32 = window::HEIGHT as f32 / 11.0;
    const ENEMY_COLUMNS: RangeInclusive<u8> = 1..=10;
    const ENEMY_ROWS: RangeInclusive<u8> = 1..=5;

    for (col, row) in iproduct!(ENEMY_COLUMNS, ENEMY_ROWS) {
        let enemy_type = match row {
            0 => "invader_A1.png",
            1 => "invader_A2.png",
            2 => "invader_B1.png",
            3 => "invader_B2.png",
            4 => "invader_C1.png",
            5 => "invader_C2.png",
            _ => unreachable!(),
        };
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load(enemy_type),
                transform: Transform::from_xyz(
                    X_OFFSET.mul_add(f32::from(col), -f32::from(window::HALF_WIDTH)),
                    Y_OFFSET.mul_add(-f32::from(row), f32::from(window::HALF_HEIGHT)),
                    0.0,
                ),
                ..default()
            },
            Enemy,
            Name::new(format!("Enemy {col}:{row}")),
        ));
    }
}

fn get_hit(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    projectile_query: Query<(Entity, &Transform), With<Projectile>>,
) {
    /* TODO: Change this number for each enemy Type */
    const LENGTH: u8 = 24;
    const HALF_LENGTH: f32 = (LENGTH / 2) as f32;
    const HEIGHT: u8 = 16;
    const HALF_HEIGHT: f32 = (HEIGHT / 2) as f32;

    fn get_enemy_translation((ent, trans): (Entity, &Transform)) -> (Entity, Vec2) {
        (ent, trans.translation.xy())
    }

    for (enemy_entity, enemy_translation) in enemy_query.iter().map(get_enemy_translation) {
        for (proj_entity, proj_translation) in projectile_query.iter().map(get_enemy_translation) {
            let enemy_range = (
                (enemy_translation.x - HALF_LENGTH)..=(enemy_translation.x + HALF_LENGTH),
                enemy_translation.y - HALF_HEIGHT..=(enemy_translation.y + HALF_HEIGHT),
            );

            let negative_pos = proj_translation - projectile::HALF_DIMENSIONS;
            let positive_pos = proj_translation + projectile::HALF_DIMENSIONS;

            if (enemy_range.0.contains(&negative_pos.x) || enemy_range.0.contains(&positive_pos.x))
                && (enemy_range.1.contains(&negative_pos.y)
                    || enemy_range.1.contains(&positive_pos.y))
            {
                commands.entity(enemy_entity).despawn();
                commands.entity(proj_entity).despawn();
            }
        }
    }
}
