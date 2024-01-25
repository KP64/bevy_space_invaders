use bevy::{app, prelude::*, render::camera::ScalingMode};

use crate::{get_single, get_single_mut, window};

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, zoom_scalingmode);
    }
}

#[derive(Component)]
struct GameCameraMarker;

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), GameCameraMarker));
}

fn zoom_scalingmode(
    (mut camera_query, window_query): (
        Query<&mut OrthographicProjection, With<GameCameraMarker>>,
        Query<&Window>,
    ),
) {
    let window = get_single!(window_query);

    let w_scale = window::DIMENSIONS.x / window.width();
    let h_scale = window::DIMENSIONS.y / window.height();
    let final_scale = w_scale.max(h_scale);

    let mut projection = get_single_mut!(camera_query);
    projection.scaling_mode = ScalingMode::WindowSize(1.0 / final_scale);
}
