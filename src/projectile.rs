use std::ops::RangeInclusive;

use bevy::{app, prelude::*};

use crate::window;

pub const DIMENSIONS: Vec2 = Vec2::new(5.0, 25.0);
pub const LENGTH: f32 = DIMENSIONS.x;
pub const HEIGHT: f32 = DIMENSIONS.y;
pub const HALF_DIMENSIONS: Vec2 = Vec2::new(LENGTH / 2.0, HEIGHT / 2.0);

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (move_projectiles, despawn_out_of_window_projectiles),
        );
    }
}

#[derive(PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
}

#[derive(Component)]
pub struct Projectile {
    pub direction: Direction,
    pub speed: f32,
}

impl Projectile {
    pub const fn new(direction: Direction, speed: f32) -> Self {
        Self { direction, speed }
    }
}

fn move_projectiles(mut query: Query<(&mut Transform, &Projectile)>) {
    for (mut projectile, proj) in &mut query {
        projectile.translation.y += if proj.direction == Direction::Up {
            proj.speed
        } else {
            -proj.speed
        };
    }
}

fn despawn_out_of_window_projectiles(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Projectile>>,
) {
    const WINDOW_HEIGHT_RANGE: RangeInclusive<f32> =
        -(window::HALF_HEIGHT as f32)..=(window::HALF_HEIGHT as f32);
    for (projectile, transform) in &query {
        if !WINDOW_HEIGHT_RANGE.contains(&transform.translation.y) {
            commands.entity(projectile).despawn();
        }
    }
}
