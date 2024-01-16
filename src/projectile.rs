use bevy::{app, prelude::*};

use crate::window;

pub const DIMENSIONS: Vec2 = Vec2::new(5.0, 25.0);
pub const LENGTH: f32 = DIMENSIONS.x;
pub const HEIGHT: f32 = DIMENSIONS.y;
pub const HALF_DIMENSIONS: Vec2 = Vec2::new(LENGTH / 2.0, HEIGHT / 2.0);

pub const SPEED: f32 = 10.0;

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (move_projectiles, despawn_out_of_window_projectiles),
        );
    }
}

#[derive(Component)]
pub struct Projectile;

fn move_projectiles(mut query: Query<&mut Transform, With<Projectile>>) {
    for mut projectile in &mut query {
        projectile.translation.y += SPEED;
    }
}

fn despawn_out_of_window_projectiles(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Projectile>>,
) {
    for (projectile, transform) in &query {
        if transform.translation.y >= f32::from(window::HALF_HEIGHT) {
            commands.entity(projectile).despawn();
        }
    }
}
