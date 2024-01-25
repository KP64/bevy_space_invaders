#![allow(clippy::type_complexity, clippy::multiple_crate_versions)]

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
    window::{PresentMode, WindowMode},
};
use bevy_rapier2d::prelude::*;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use enemy::invader::Invader;
use player::Player;

mod asset_loader;
mod camera;
mod enemy;
mod game_time;
mod player;
mod projectile;
mod score;
mod window;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    InGame,
    GameOver,
}

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(LogPlugin {
                level: Level::DEBUG,
                filter: "info,wgpu_core=warn,wgpu_hal=warn,bevy_space_invaders=debug".into(),
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Space Invaders".into(),
                    present_mode: PresentMode::AutoNoVsync,
                    resolution: window::DIMENSIONS.into(),
                    mode: WindowMode::Windowed,
                    ..default()
                }),
                ..default()
            }),
    )
    .add_plugins((
        ScreenDiagnosticsPlugin::default(),
        ScreenFrameDiagnosticsPlugin,
    ))
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugins((window::Plugin, camera::Plugin))
    .add_plugins((
        asset_loader::Plugin,
        score::Plugin,
        game_time::Plugin,
        player::Plugin,
        projectile::Plugin,
        enemy::Plugin,
    ));

    app.add_state::<AppState>();

    app.add_systems(Update, is_game_over);

    #[cfg(debug_assertions)]
    {
        use bevy_inspector_egui::quick::WorldInspectorPlugin;
        app.add_plugins(RapierDebugRenderPlugin::default())
            .add_plugins(WorldInspectorPlugin::default());
        bevy_mod_debugdump::print_schedule_graph(&mut app, Update);
    }

    app.run();
}

fn is_game_over(
    mut game_state: ResMut<NextState<AppState>>,
    (player_query, invader_query): (Query<&Player>, Query<&Invader>),
) {
    if invader_query.is_empty() || player_query.is_empty() {
        game_state.set(AppState::GameOver);
    }
}

#[macro_export]
macro_rules! get_single {
    ($q:expr) => {
        match $q.get_single() {
            Ok(m) => m,
            _ => return,
        }
    };
}

#[macro_export]
macro_rules! get_single_mut {
    ($q:expr) => {
        match $q.get_single_mut() {
            Ok(m) => m,
            _ => return,
        }
    };
}
