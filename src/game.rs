use bevy::{app, ecs, prelude::*, time::Stopwatch};
use bevy_rand::{plugin::EntropyPlugin, prelude::*};
use std::fmt;

use crate::AppState;

mod cell;
mod enemy;
mod player;
mod projectile;
mod ui;

const ROWS: u8 = 11;
const COLUMNS: u8 = 11;

const ON_STARTUP: OnTransition<AppState> = OnTransition {
    from: AppState::MainMenu,
    to: AppState::Game,
};

#[derive(States, Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum State {
    /// State when Player starts a new Game
    #[default]
    Started,

    /// State when a Level Should be Constructed
    Setup,

    /// State when Player is in the middle of a game
    Playing,

    /// State when the game is Paused
    /// Can only be triggered when `InGame`
    Paused,

    /// State when a Level has been finished
    LvlFinished,

    /// State when Player ends a game or Dies
    GameOver,

    /// State when Player exits a Game
    Exit,
}

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_state::<State>()
            .add_event::<LevelUp>()
            .init_resource::<Board>()
            .init_resource::<Level>()
            .init_resource::<Score>()
            .add_plugins(EntropyPlugin::<ChaCha8Rng>::default())
            .add_plugins((
                ui::Plugin,
                projectile::Plugin,
                player::Plugin,
                enemy::Plugin,
            ))
            .add_systems(ON_STARTUP, start_new)
            .add_systems(OnEnter(State::Setup), change_to_play_state)
            .add_systems(
                Update,
                pause.run_if(in_state(State::Playing).or_else(in_state(State::Paused))),
            )
            .add_systems(Update, lvl_up.run_if(in_state(State::Playing)))
            .add_systems(OnEnter(State::LvlFinished), new_level);
    }
}

fn start_new(mut next_state: ResMut<NextState<State>>) {
    next_state.set(State::Setup);
}

fn new_level(mut game_state: ResMut<NextState<State>>) {
    game_state.set(State::Setup);
}

fn change_to_play_state(mut next_state: ResMut<NextState<State>>) {
    next_state.set(State::Playing);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LevelType {
    Normal,
    Boss,
    Bonus,
}

impl From<Level> for LevelType {
    fn from(value: Level) -> Self {
        if value.0 % 10 == 0 {
            Self::Bonus
        } else if value.0 % 5 == 0 {
            Self::Boss
        } else {
            Self::Normal
        }
    }
}

#[derive(Resource, Clone, Copy, Deref, DerefMut)]
struct Level(usize);

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

    fn get_color(self) -> Color {
        let lvl_type = LevelType::from(self);
        match lvl_type {
            LevelType::Normal => Color::WHITE,
            LevelType::Boss => Color::CRIMSON,
            LevelType::Bonus => Color::GOLD,
        }
    }
}

#[derive(Event, Default)]
struct LevelUp;

fn lvl_up(
    (mut lvl, mut lvl_up_event, mut game_state): (
        ResMut<Level>,
        EventReader<LevelUp>,
        ResMut<NextState<State>>,
    ),
) {
    for _ in lvl_up_event.read() {
        lvl.level_up();
        LevelType::from(*lvl);
        game_state.set(State::LvlFinished);
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
struct Score(usize);

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:^05}", self.0)
    }
}

impl Score {
    const fn get_color(&self) -> Color {
        match self.0 {
            0..=99 => Color::WHITE,
            100..=499 => Color::rgb(0.7, 0.7, 0.7),
            500..=999 => Color::rgb(0.4, 0.4, 0.4),
            _ => Color::AQUAMARINE,
        }
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
struct Time(Stopwatch);

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let time = self.0.elapsed().as_secs();
        let (minutes, seconds) = (time / 60, time % 60);

        write!(f, "{minutes:02}:{seconds:02}")
    }
}

#[derive(Resource, Deref, DerefMut)]
struct Board(Vec<Vec<cell::Position>>);

impl Default for Board {
    fn default() -> Self {
        Self(
            (0..ROWS)
                .map(|row| {
                    (0..COLUMNS)
                        .map(|column| cell::Position::new(row, column))
                        .collect()
                })
                .collect(),
        )
    }
}

fn pause(
    (keys, state, mut next_state): (
        Res<Input<KeyCode>>,
        Res<ecs::schedule::State<State>>,
        ResMut<NextState<State>>,
    ),
) {
    if !keys.just_pressed(KeyCode::P) {
        return;
    }

    next_state.set(match state.get() {
        State::Playing => State::Paused,
        State::Paused => State::Playing,
        _ => unreachable!(
            "The `pause` System should not be run unless when the User is in `Playing` or `Paused` GameState"
        ),
    });
}
