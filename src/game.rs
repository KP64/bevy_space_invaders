use bevy::{app, ecs::schedule, prelude::*, time::Stopwatch};
use bevy_rand::{plugin::EntropyPlugin, prelude::*};
use std::fmt;

use crate::AppState;

mod cell;
mod enemy;
mod level;
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
            .init_resource::<Board>()
            .init_resource::<Score>()
            .add_plugins(EntropyPlugin::<ChaCha8Rng>::default())
            .add_plugins((
                level::Plugin,
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
            );
    }
}

fn start_new(mut next_state: ResMut<NextState<State>>) {
    next_state.set(State::Setup);
}

fn change_to_play_state(mut next_state: ResMut<NextState<State>>) {
    next_state.set(State::Playing);
}

#[derive(Resource, Default, Clone, Copy, Deref, DerefMut)]
struct Score(usize);

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:^05}", self.0)
    }
}

impl From<Score> for Color {
    fn from(value: Score) -> Self {
        match value.0 {
            0..=99 => Self::WHITE,
            100..=499 => Self::rgb(0.7, 0.7, 0.7),
            500..=999 => Self::rgb(0.4, 0.4, 0.4),
            _ => Self::AQUAMARINE,
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
        Res<schedule::State<State>>,
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
