#![allow(clippy::type_complexity, clippy::multiple_crate_versions)]

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
    window::{close_on_esc, EnabledButtons, PresentMode, WindowMode},
    winit::WinitWindows,
};

use bevy_rapier2d::prelude::*;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use winit::window::Icon;

mod asset_loader;
mod enemy;
mod game_time;
mod player;
mod projectile;
mod score;
mod window {
    pub const DIMENSIONS: (u16, u16) = (1280, 720);
    pub const WIDTH: u16 = DIMENSIONS.0;
    pub const HEIGHT: u16 = DIMENSIONS.1;
    pub const HALF_WIDTH: u16 = WIDTH / 2;
    pub const HALF_HEIGHT: u16 = HEIGHT / 2;
}

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
                    resizable: false,
                    present_mode: PresentMode::AutoNoVsync,
                    resolution: window::DIMENSIONS.into(),
                    mode: WindowMode::Windowed,
                    enabled_buttons: EnabledButtons {
                        maximize: false,
                        ..default()
                    },
                    ..default()
                }),
                ..default()
            }),
    )
    .add_plugins((
        ScreenDiagnosticsPlugin::default(),
        ScreenFrameDiagnosticsPlugin,
    ))
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .add_plugins((
        asset_loader::Plugin,
        score::Plugin,
        game_time::Plugin,
        player::Plugin,
        projectile::Plugin,
        enemy::Plugin,
    ));

    app.add_state::<AppState>();

    app.add_systems(Startup, (setup_camera, set_window_icon))
        .add_systems(Update, close_on_esc);

    #[cfg(debug_assertions)]
    {
        use bevy_inspector_egui::quick::WorldInspectorPlugin;
        app.add_plugins(RapierDebugRenderPlugin::default())
            .add_plugins(WorldInspectorPlugin::default());
        bevy_mod_debugdump::print_schedule_graph(&mut app, Update);
    }

    app.run();
}

#[derive(Component)]
struct GameCameraMarker;

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), GameCameraMarker));
}

// TODO: Change this when Bevy adds native Window Icon Support
fn set_window_icon(
    // we have to use `NonSend` here
    windows: NonSend<WinitWindows>,
) {
    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/game_icon_x512.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    // do it for all windows
    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}
