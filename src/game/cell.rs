use crate::window;

use super::{COLUMNS, ROWS};
use bevy::prelude::*;

pub(super) const DIMENSIONS: Vec2 = Vec2::new(
    window::DIMENSIONS.x / (COLUMNS as f32),
    window::DIMENSIONS.y / (ROWS as f32),
);

#[derive(Default, Clone, Copy, Deref, DerefMut)]
pub(super) struct Position(pub(super) Vec2);

impl Position {
    pub(super) fn new(row: u8, column: u8) -> Self {
        Self(Vec2::new(
            DIMENSIONS
                .x
                .mul_add(f32::from(column), -window::DIMENSIONS.x / 2.0)
                + DIMENSIONS.x / 2.0,
            DIMENSIONS
                .y
                .mul_add(-f32::from(row), window::DIMENSIONS.y / 2.0)
                - DIMENSIONS.y / 2.0,
        ))
    }
}
