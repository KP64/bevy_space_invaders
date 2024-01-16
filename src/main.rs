use std::ops::RangeInclusive;

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
    window::{close_on_esc, EnabledButtons, WindowMode},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use itertools::iproduct;

mod window {
    pub const DIMENSIONS: (u16, u16) = (1280, 720);
    pub const WIDTH: u16 = DIMENSIONS.0;
    pub const HEIGHT: u16 = DIMENSIONS.1;
    pub const HALF_WIDTH: u16 = WIDTH / 2;
    pub const HALF_HEIGHT: u16 = HEIGHT / 2;
}
mod player;
mod projectile;
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
    .add_plugins((player::Plugin, projectile::Plugin));

    app.add_systems(Startup, (setup_camera, setup_enemies))
        .add_systems(Update, close_on_esc);

    bevy_mod_debugdump::print_schedule_graph(&mut app, Update);

    app.run();
}

#[derive(Component)]
struct GameCameraMarker;
fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), GameCameraMarker));
}

fn setup_enemies(mut commands: Commands, asset_server: Res<AssetServer>) {
    const X_OFFSET: f32 = window::WIDTH as f32 / 11.0;
    const Y_OFFSET: f32 = window::HEIGHT as f32 / 11.0;
    const ENEMY_COLUMNS: RangeInclusive<u8> = 1..=10;
    const ENEMY_ROWS: RangeInclusive<u8> = 1..=5;

    for (col, row) in iproduct!(ENEMY_COLUMNS, ENEMY_ROWS) {
        let enemy_type = match row {
            0 => "invader_A1.png",
            1 => "invader_A2.png",
            2 => "invader_B1.png",
            3 => "invader_B2.png",
            4 => "invader_C1.png",
            5 => "invader_C2.png",
            _ => unreachable!(),
        };
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load(enemy_type),
                transform: Transform::from_xyz(
                    X_OFFSET.mul_add(f32::from(col), -f32::from(window::HALF_WIDTH)),
                    Y_OFFSET.mul_add(-f32::from(row), f32::from(window::HALF_HEIGHT)),
                    0.0,
                ),
                ..default()
            },
            Name::new(format!("Enemy {col}:{row}")),
        ));
    }
}
