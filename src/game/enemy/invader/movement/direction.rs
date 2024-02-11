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

impl Direction {
    pub(super) fn to_vec2(self) -> Vec2 {
        const X_OFFSET: f32 = cell::DIMENSIONS.x / 4.0;
        const Y_OFFSET: f32 = cell::DIMENSIONS.y / 2.0;
        match self {
            Self::Left => Vec2::new(-X_OFFSET, 0.0),
            Self::Right => Vec2::new(X_OFFSET, 0.0),
            Self::Down => Vec2::new(0.0, -Y_OFFSET),
        }
    }
}

#[derive(Resource, Deref, DerefMut)]
pub(super) struct Next {
    pub(super) idx: usize,

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
