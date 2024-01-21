use std::ops::RangeInclusive;

use crate::window;
use bevy::{app, prelude::*};

pub const DIMENSIONS: Vec2 = Vec2::new(5.0, 25.0);
pub const LENGTH: f32 = DIMENSIONS.x;
pub const HEIGHT: f32 = DIMENSIONS.y;
pub const HALF_LENGTH: f32 = LENGTH / 2.0;
pub const HALF_HEIGHT: f32 = HEIGHT / 2.0;

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_out_of_window);
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
}

impl Direction {
    pub const fn is_downwards(self) -> bool {
        matches!(self, Self::Down)
    }

    pub const fn is_upwards(self) -> bool {
        matches!(self, Self::Up)
    }
}

#[derive(Component)]
pub struct Projectile {
    pub direction: Direction,
}

fn despawn_out_of_window(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Projectile>>,
) {
    const WINDOW_HEIGHT_RANGE: RangeInclusive<f32> =
        -(window::HALF_HEIGHT as f32)..=(window::HALF_HEIGHT as f32);
    for (projectile, _) in query
        .iter()
        .filter(|(_, transform)| !WINDOW_HEIGHT_RANGE.contains(&transform.translation.y))
    {
        commands.entity(projectile).despawn();
    }
}
