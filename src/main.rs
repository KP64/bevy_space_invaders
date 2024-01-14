use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use itertools::iproduct;
fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(LogPlugin {
                level: Level::DEBUG,
                filter: "info,wgpu_core=warn,wgpu_hal=warn,cs_billo=debug".into(),
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Space Invaders".into(),
                    ..default()
                }),
                ..default()
            }),
    )
    .add_plugins((
        ScreenDiagnosticsPlugin::default(),
        ScreenFrameDiagnosticsPlugin,
    ))
    .add_plugins(WorldInspectorPlugin::default());
    app.add_systems(Startup, (setup_camera, setup_player, setup_enemies))
        .add_systems(Update, (player_movement, player_shooting));
    bevy_mod_debugdump::print_schedule_graph(&mut app, Update);

    app.run();
}

#[derive(Component)]
struct GameCameraMarker;
fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), GameCameraMarker));
}

const WINDOW_DIMENSIONS: (u16, u16) = (1280, 720);
const WINDOW_WIDTH: u16 = WINDOW_DIMENSIONS.0;
const WINDOW_HEIGHT: u16 = WINDOW_DIMENSIONS.1;
const WINDOW_HALF_WIDTH: u16 = WINDOW_WIDTH / 2;
const WINDOW_HALF_HEIGHT: u16 = WINDOW_HEIGHT / 2;
fn setup_enemies(mut commands: Commands, asset_server: Res<AssetServer>) {
    /* TODO: Work on Enemy Offsets */
    const X_OFFSET: f32 = WINDOW_WIDTH as f32 / 11.0;
    const Y_OFFSET: f32 = WINDOW_HEIGHT as f32 / 11.0;

    for (i, j) in iproduct!(1..=11_u8, 1..=5_u8) {
        let texture = match j {
            0 => asset_server.load("invader_A1.png"),
            1 => asset_server.load("invader_A2.png"),
            2 => asset_server.load("invader_B1.png"),
            3 => asset_server.load("invader_B2.png"),
            4 => asset_server.load("invader_C1.png"),
            5 => asset_server.load("invader_C2.png"),
            _ => unreachable!(),
        };
        commands.spawn((
            SpriteBundle {
                texture,
                transform: Transform::from_xyz(
                    X_OFFSET.mul_add(f32::from(i), -f32::from(WINDOW_HALF_WIDTH)),
                    Y_OFFSET.mul_add(-f32::from(j), f32::from(WINDOW_HALF_HEIGHT)),
                    0.0,
                ),
                ..default()
            },
            Name::new(format!("Enemy {i}:{j}")),
        ));
    }
}

#[derive(Component)]
struct Player;
fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("player.png"),
            transform: Transform::from_xyz(0.0, -300.0, 0.0),
            ..default()
        },
        Player,
        Name::new("Player"),
    ));
}

fn player_movement(keys: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Player>>) {
    const PLAYER_HALF_LENGTH: f32 = 13.0;
    let player_velocity = keys
        .get_pressed()
        .map(|key| match key {
            KeyCode::A => Vec3::new(-1.0, 0.0, 0.0),
            KeyCode::D => Vec3::new(1.0, 0.0, 0.0),
            _ => Vec3::splat(0.0),
        })
        .sum::<Vec3>();

    for mut transform in &mut query {
        transform.translation += player_velocity;
        transform.translation.x = transform.translation.x.clamp(
            -f32::from(WINDOW_HALF_WIDTH) + PLAYER_HALF_LENGTH,
            f32::from(WINDOW_HALF_WIDTH) - PLAYER_HALF_LENGTH,
        );
    }
}

fn player_shooting(keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        warn!("PENG!");
    }
}
