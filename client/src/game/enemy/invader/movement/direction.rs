use crate::game::cell;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub(super) enum Direction {
    Left,
    Right,
    Down,
}

const DIRECTIONS: [Direction; 6] = [
    Direction::Right,
    Direction::Down,
    Direction::Left,
    Direction::Left,
    Direction::Down,
    Direction::Right,
];

impl From<Direction> for Vec2 {
    fn from(value: Direction) -> Self {
        const X_OFFSET: f32 = cell::DIMENSIONS.x / 4.0;
        const Y_OFFSET: f32 = cell::DIMENSIONS.y / 2.0;
        match value {
            Direction::Left => Self::new(-X_OFFSET, 0.0),
            Direction::Right => Self::new(X_OFFSET, 0.0),
            Direction::Down => Self::new(0.0, -Y_OFFSET),
        }
    }
}

#[derive(Resource, Deref, DerefMut)]
pub(super) struct Next {
    idx: usize,

    #[deref]
    pub(super) direction: Direction,
}

impl Default for Next {
    fn default() -> Self {
        Self {
            idx: 0,
            direction: DIRECTIONS[0],
        }
    }
}

impl Next {
    pub(super) fn next(&mut self) {
        let next_idx = (self.idx + 1) % DIRECTIONS.len();
        self.idx = next_idx;
        self.direction = DIRECTIONS[next_idx];
    }
}
