#![allow(clippy::type_complexity, clippy::multiple_crate_versions)]

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
    render::camera::ScalingMode,
    window::{close_on_esc, PresentMode, WindowMode},
    winit::WinitWindows,
};

use bevy_rapier2d::prelude::*;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use enemy::invader::Invader;
use player::Player;
use winit::window::Icon;

mod asset_loader;
mod enemy;
mod game_time;
mod player;
mod projectile;
mod score;
mod window {
    use bevy::math::Vec2;

    pub const WIDTH: u16 = 1280;
    pub const HEIGHT: u16 = 720;
    pub const DIMENSIONS: Vec2 = Vec2::new(WIDTH as f32, HEIGHT as f32);
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
        .add_systems(Update, (close_on_esc, zoom_scalingmode, toggle_vsync, is_game_over));

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

fn zoom_scalingmode(
    windows: Query<&Window>,
    mut query_camera: Query<&mut OrthographicProjection, With<GameCameraMarker>>,
) {
    let window = windows.single();

    let w_scale = window::DIMENSIONS.x / window.width();
    let h_scale = window::DIMENSIONS.y / window.height();
    let final_scale = w_scale.max(h_scale);

    let mut projection = query_camera.single_mut();
    projection.scaling_mode = ScalingMode::WindowSize(1.0 / final_scale);
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

/// This system toggles the vsync mode when pressing the button V.
/// You'll see fps increase displayed in the console.
fn toggle_vsync(input: Res<Input<KeyCode>>, mut window_query: Query<&mut Window>) {
    if !input.just_pressed(KeyCode::V) {
        return;
    }

    let mut window = get_single_mut!(window_query);

    window.present_mode = if matches!(window.present_mode, PresentMode::AutoVsync) {
        PresentMode::AutoNoVsync
    } else {
        PresentMode::AutoVsync
    };

    info!("PRESENT_MODE: {:?}", window.present_mode);
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
