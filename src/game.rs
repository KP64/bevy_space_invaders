use crate::AppState;
use bevy::{app, ecs::schedule, prelude::*, time::Stopwatch};
use bevy_rand::prelude::*;
use leafwing_input_manager::prelude::*;
use player::actions::Action;
use std::fmt;

mod cell;
mod enemy;
mod level;
mod menu;
mod player;
mod projectile;
mod ui;

const ROWS: u8 = 11;
const COLUMNS: u8 = 11;

#[derive(States, Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum State {
    /// Empty State without any Logic
    #[default]
    Empty,

    /// State when a new Game is started
    Setup,

    /// State when Level should be Played
    LvlStartup,

    /// State when Player is in the middle of a game
    Playing,

    /// State when a Level has been finished
    LvlFinished,

    /// State when Player ends a game or Dies
    GameOver,

    /// State when Player enters Leaderboard Data
    Leaderboard,

    /// State when the game is Paused
    /// Can only be triggered when Playing
    Paused,

    /// State when Player exits a Game
    Exit,
}

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_state::<State>()
            .init_resource::<Board>()
            .add_plugins(EntropyPlugin::<ChaCha8Rng>::default())
            .add_plugins((
                level::Plugin,
                ui::Plugin,
                menu::Plugin,
                projectile::Plugin,
                player::Plugin,
                enemy::Plugin,
            ))
            .add_systems(
                OnTransition {
                    from: AppState::MainMenu,
                    to: AppState::InGame,
                },
                start_new,
            )
            .add_systems(OnEnter(State::Setup), setup)
            .add_systems(OnEnter(State::LvlStartup), to_play_state)
            .add_systems(
                Update,
                toggle_pause.run_if(in_state(State::Playing).or_else(in_state(State::Paused))),
            );
    }
}

fn start_new(mut next_state: ResMut<NextState<State>>) {
    next_state.set(State::Setup);
}

fn setup(mut commands: Commands) {
    commands.insert_resource(Score::default());
}

fn to_play_state(mut next_state: ResMut<NextState<State>>) {
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
        let time = self.elapsed().as_secs();
        let (minutes, seconds) = (time / 60, time % 60);

        write!(f, "{minutes:02}:{seconds:02}")
    }
}

#[derive(Resource, Deref, DerefMut)]
struct Board(Vec<Vec<cell::Position>>);

impl Board {
    fn get_last_invader_y_cell(&self) -> f32 {
        let x = &self[self.len() - 2];
        x[0].y
    }
}

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

fn toggle_pause(
    (state, mut next_state): (Res<schedule::State<State>>, ResMut<NextState<State>>),
    input: Query<&ActionState<Action>>,
) {
    for input in &input {
        if !input.just_pressed(&Action::TogglePause) {
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
}
