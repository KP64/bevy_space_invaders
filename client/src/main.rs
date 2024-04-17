#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{
    audio,
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
    /// State when the app is Started and is not `InGame`
    #[default]
    MainMenu,

    /// State when the app is in the Settings Menu
    Settings,

    /// State when in a Game
    InGame,
}

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(LogPlugin {
                level: log::Level::DEBUG,
                filter: "info,wgpu_core=warn,wgpu_hal=warn,bevy_space_invaders=debug".into(),
                ..default()
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
            .add_systems(Update, (toggle_debug_renderer, volume));
        bevy_mod_debugdump::print_schedule_graph(&mut app, Update);
    }

    app.init_state::<AppState>().add_plugins((
        window::Plugin,
        camera::Plugin,
        menu::Plugin,
        game::Plugin,
    ));

    app.run();
}

#[cfg(debug_assertions)]
fn toggle_debug_renderer(
    (mut ctx, input): (ResMut<DebugRenderContext>, Res<ButtonInput<KeyCode>>),
) {
    if input.just_pressed(KeyCode::KeyR) {
        ctx.enabled = !ctx.enabled;
    }
}

fn volume((mut glob_vol, keyboard_input): (ResMut<GlobalVolume>, Res<ButtonInput<KeyCode>>)) {
    let diff = if keyboard_input.just_pressed(KeyCode::KeyL) {
        0.1
    } else if keyboard_input.just_pressed(KeyCode::KeyK) {
        -0.1
    } else {
        return;
    };

    let clamped = (glob_vol.volume.get() + diff).clamp(0.0, 1.0);
    glob_vol.volume = audio::Volume::new(clamped);
}
