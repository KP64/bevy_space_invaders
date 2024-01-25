use std::ops::Range;

use bevy::{app, prelude::*};
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{asset_loader::TextureAssets, enemy::Points, get_single, window};

use super::{invader, Enemy};

const LENGTH: u8 = 48;
const HEIGHT: u8 = 21;
const HALF_LENGTH: f32 = LENGTH as f32 / 2.0;
const HALF_HEIGHT: f32 = HEIGHT as f32 / 2.0;
const SPAWN_TIME_DURATION: f32 = 30.0;
const VELOCITY: Vec2 = Vec2::new(250.0, 0.0);

const POSSIBLE_POINTS: [u16; 5] = [50, 100, 150, 200, 300];
const POSSIBLE_POINTS_RANGE: Range<usize> = 0..POSSIBLE_POINTS.len();

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpawnTimer>().add_systems(
            Update,
            (tick_spawn_timer, despawn_out_of_window, spawn_over_time),
        );
    }
}

#[derive(Component)]
pub struct Ufo;

#[derive(Resource)]
struct SpawnTimer {
    timer: Timer,
}

impl Default for SpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(SPAWN_TIME_DURATION, TimerMode::Repeating),
        }
    }
}

fn tick_spawn_timer(mut spawn_timer: ResMut<SpawnTimer>, time: Res<Time>) {
    spawn_timer.timer.tick(time.delta());
}

fn spawn_over_time(
    mut commands: Commands,
    (texture_assets, spawn_timer): (Res<TextureAssets>, Res<SpawnTimer>),
    (window_query, ufo_query): (Query<&Window>, Query<&Ufo>),
) {
    if !spawn_timer.timer.finished() || ufo_query.get_single().is_ok() {
        return;
    }

    let window = get_single!(window_query);
    commands.spawn((
        Ufo,
        Name::new("Ufo"),
        Enemy,
        Points(get_random_points()),
        SpriteBundle {
            texture: texture_assets.ufo.clone(),
            transform: Transform::from_xyz(
                -window.width() - 100.0,
                -invader::Y_OFFSET + f32::from(window::HALF_HEIGHT),
                0.0,
            ),
            ..default()
        },
        RigidBody::KinematicVelocityBased,
        Velocity::linear(VELOCITY),
        Sensor,
        CollisionGroups::new(Group::GROUP_2, Group::GROUP_3),
        ActiveCollisionTypes::KINEMATIC_STATIC,
        ActiveEvents::COLLISION_EVENTS,
        Collider::cuboid(HALF_LENGTH, HALF_HEIGHT),
    ));
}

fn get_random_points() -> u16 {
    POSSIBLE_POINTS[rand::thread_rng().gen_range(POSSIBLE_POINTS_RANGE)]
}

fn despawn_out_of_window(mut commands: Commands, query: Query<(Entity, &Transform), With<Ufo>>) {
    let (entity, transform) = get_single!(query);

    let translation = transform.translation.x - HALF_LENGTH;
    if f32::from(window::HALF_WIDTH) <= translation {
        commands.entity(entity).despawn();
    }
}
