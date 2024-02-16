use crate::game::{self, enemy::invader};
use bevy::{app, prelude::*};
use std::fmt;
pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelUp>()
            .init_resource::<Level>()
            .add_systems(Update, lvl_up.run_if(in_state(game::State::Playing)))
            .add_systems(OnEnter(game::State::LvlFinished), new_level);
    }
}

fn new_level(mut game_state: ResMut<NextState<game::State>>) {
    game_state.set(game::State::Setup);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Type {
    Normal,
    Boss,
    Bonus,
}

impl Type {
    pub(super) fn is_normal(lvl: Res<Level>) -> bool {
        Self::from(lvl) == Self::Normal
    }

    pub(super) fn is_boss(lvl: Res<Level>) -> bool {
        Self::from(lvl) == Self::Boss
    }

    pub(super) fn is_bonus(lvl: Res<Level>) -> bool {
        Self::from(lvl) == Self::Bonus
    }
}

impl From<Level> for Type {
    fn from(value: Level) -> Self {
        match value.0 {
            x if x % 10 == 0 => Self::Bonus,
            x if x % 5 == 0 => Self::Boss,
            _ => Self::Normal,
        }
    }
}

impl From<Res<'_, Level>> for Type {
    fn from(value: Res<Level>) -> Self {
        Self::from(*value)
    }
}

#[derive(Resource, Clone, Copy, Deref, DerefMut)]
pub(super) struct Level(usize);

impl Default for Level {
    fn default() -> Self {
        Self(1)
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:^5}", self.0)
    }
}

impl Level {
    fn level_up(&mut self) {
        self.0 += 1;
    }
}

impl From<Level> for Color {
    fn from(value: Level) -> Self {
        Type::from(value).into()
    }
}

impl From<Type> for Color {
    fn from(value: Type) -> Self {
        match value {
            Type::Normal => Self::WHITE,
            Type::Boss => Self::CRIMSON,
            Type::Bonus => Self::GOLD,
        }
    }
}

#[derive(Event, Default)]
pub(super) struct LevelUp;

fn lvl_up(
    (mut lvl_up_event, mut lvl, mut probability, mut game_state): (
        EventReader<LevelUp>,
        ResMut<Level>,
        ResMut<invader::shooting::probability::Probability>,
        ResMut<NextState<game::State>>,
    ),
) {
    for _ in lvl_up_event.read() {
        lvl.level_up();
        probability.increase();
        game_state.set(game::State::LvlFinished);
    }
}
