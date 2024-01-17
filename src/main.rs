use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
    window::{close_on_esc, EnabledButtons, WindowMode},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};

mod asset_loader;
mod enemy;
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
    .add_plugins(WorldInspectorPlugin::default())
    .add_plugins((
        asset_loader::Plugin,
        score::Plugin,
        player::Plugin,
        projectile::Plugin,
        enemy::Plugin,
    ));

    app.add_systems(Startup, setup_camera)
        .add_systems(Update, close_on_esc);

    bevy_mod_debugdump::print_schedule_graph(&mut app, Update);

    app.run();
}

#[derive(Component)]
struct GameCameraMarker;
fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), GameCameraMarker));
}
