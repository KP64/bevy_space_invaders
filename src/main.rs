#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::type_complexity, clippy::multiple_crate_versions)]

use bevy::{
    log::{self, LogPlugin},
    prelude::*,
    window::PresentMode,
};
use bevy_rapier2d::prelude::*;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};

mod camera;
mod game;
mod menu;
mod window;

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
enum AppState {
    /// State when the app is Started & is not InGame
    #[default]
    MainMenu,

    /// State when the app is in the Settings Menu
    Settings,

    /// State when in a Game
    Game,
}

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(LogPlugin {
                level: log::Level::DEBUG,
                filter: "info,wgpu_core=warn,wgpu_hal=warn,bevy_space_invaders=debug".into(),
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Space Invaders".into(),
                    resolution: window::DIMENSIONS.into(),
                    present_mode: PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    )
    .add_plugins((
        ScreenDiagnosticsPlugin::default(),
        ScreenFrameDiagnosticsPlugin,
    ))
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::default());

    #[cfg(debug_assertions)]
    {
        app.add_plugins(RapierDebugRenderPlugin::default())
            .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::default())
            .add_systems(Update, toggle_debug_renderer);
        bevy_mod_debugdump::print_schedule_graph(&mut app, Update);
    }

    app.add_state::<AppState>().add_plugins((
        window::Plugin,
        camera::Plugin,
        menu::Plugin,
        game::Plugin,
    ));

    app.run();
}

#[cfg(debug_assertions)]
fn toggle_debug_renderer((mut ctx, input): (ResMut<DebugRenderContext>, Res<Input<KeyCode>>)) {
    if input.just_pressed(KeyCode::R) {
        ctx.enabled = !ctx.enabled;
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
