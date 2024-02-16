use crate::{get_single, get_single_mut, window};
use bevy::{app, prelude::*, render::camera::ScalingMode, window::PrimaryWindow};

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, change_scale);
    }
}

#[derive(Component)]
struct Marker;

fn setup(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), Camera2dBundle::default(), Marker));
}

fn change_scale(
    (mut camera_query, window): (
        Query<&mut OrthographicProjection, With<Marker>>,
        Query<&Window, With<PrimaryWindow>>,
    ),
) {
    let window = get_single!(window);

    let w_scale = window::DIMENSIONS.x / window.width();
    let h_scale = window::DIMENSIONS.y / window.height();
    let final_scale = w_scale.max(h_scale);

    let mut projection = get_single_mut!(camera_query);
    projection.scaling_mode = ScalingMode::WindowSize(1.0 / final_scale);
}
