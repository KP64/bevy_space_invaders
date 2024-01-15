use bevy::prelude::*;

use crate::window;

pub const DIMENSIONS: Vec2 = Vec2::new(5.0, 25.0);
pub const SPEED: f32 = 10.0;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_projectiles, despawn_projectiles));
    }
}

#[derive(Component)]
pub struct Projectile;

fn move_projectiles(mut query: Query<&mut Transform, With<Projectile>>) {
    for mut projectile in &mut query {
        projectile.translation.y += SPEED;
    }
}

fn despawn_projectiles(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Projectile>>,
) {
    for (projectile, transform) in &query {
        if transform.translation.y >= f32::from(window::HALF_HEIGHT) {
            commands.entity(projectile).despawn();
        }
    }
}
