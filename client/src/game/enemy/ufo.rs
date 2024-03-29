use super::PointsWorth;
use crate::{
    game::{self, enemy::Enemy},
    window,
};
use bevy::{app, audio, prelude::*};
use bevy_rand::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;
use std::cmp::Ordering;

const DIMENSIONS: Vec2 = Vec2::new(48.0, 21.0);

// TODO: Make Velocity.x Random sign (+ || -)?
const VELOCITY: Velocity = Velocity::linear(Vec2::new(200.0, 0.0));

const POINTS: [usize; 5] = [50, 100, 150, 200, 300];

const X_OFFSET: f32 = window::DIMENSIONS.x / 2.0 + DIMENSIONS.x;
const SECONDS_TILL_SPAWN: f32 = 10.0;
const MAX_XTRA_SECONDS_TILL_SPAWN: f32 = 8.0;

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Spawn>()
            .add_systems(
                Update,
                spawn_spawner.run_if(
                    resource_exists::<Spawner>
                        .map(|d| !d)
                        .and_then(in_state(game::State::Playing)),
                ),
            )
            .add_systems(
                Update,
                tick_spawner
                    .run_if(resource_exists::<Spawner>.and_then(in_state(game::State::Playing))),
            )
            .add_systems(
                Update,
                (spawn, despawn_out_of_window).run_if(in_state(game::State::Playing)),
            )
            .add_systems(OnEnter(game::State::Paused), freeze)
            .add_systems(OnExit(game::State::Paused), unfreeze)
            .add_systems(OnEnter(game::State::GameOver), cleanup);
    }
}

fn freeze(mut velocities: Query<(&mut Velocity, &AudioSink), With<Ufo>>) {
    for (mut velocity, sfx) in &mut velocities {
        *velocity = Velocity::zero();
        sfx.pause();
    }
}

fn unfreeze(mut velocities: Query<(&mut Velocity, &AudioSink), With<Ufo>>) {
    for (mut velocity, sfx) in &mut velocities {
        *velocity = VELOCITY;
        sfx.play();
    }
}

#[derive(Component)]
struct Ufo;

#[derive(Bundle)]
struct Bundle {
    ufo: Ufo,
    enemy: Enemy,
    points: PointsWorth,
    sprite: SpriteBundle,
    rigidbody: RigidBody,
    velocity: Velocity,
    sensor: Sensor,
    active_collision_types: ActiveCollisionTypes,
    active_events: ActiveEvents,
    collider: Collider,
    collision_groups: CollisionGroups,
}

impl Bundle {
    fn new(points: PointsWorth, sprite: SpriteBundle) -> Self {
        Self {
            ufo: Ufo,
            enemy: Enemy,
            points,
            sprite,
            rigidbody: RigidBody::KinematicVelocityBased,
            velocity: VELOCITY,
            sensor: Sensor,
            active_collision_types: ActiveCollisionTypes::KINEMATIC_KINEMATIC
                | ActiveCollisionTypes::KINEMATIC_STATIC,
            active_events: ActiveEvents::COLLISION_EVENTS,
            collider: Collider::cuboid(DIMENSIONS.x / 2.0, DIMENSIONS.y / 2.0),
            collision_groups: CollisionGroups::new(Group::GROUP_2, Group::GROUP_3),
        }
    }
}

#[derive(Resource, Deref, DerefMut)]
struct Spawner(Timer);

impl Spawner {
    fn with_extra(seconds: f32) -> Self {
        Self(Timer::from_seconds(
            SECONDS_TILL_SPAWN + seconds,
            TimerMode::Once,
        ))
    }
}

fn spawn_spawner(mut commands: Commands, mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>) {
    commands.insert_resource(Spawner::with_extra(
        rng.gen::<f32>() * MAX_XTRA_SECONDS_TILL_SPAWN,
    ));
}

fn tick_spawner(
    mut commands: Commands,
    (time, mut spawner, mut spawn_event): (Res<Time>, ResMut<Spawner>, EventWriter<Spawn>),
    ufos: Query<(), With<Ufo>>,
) {
    if !ufos.is_empty() {
        return;
    }

    if spawner.tick(time.delta()).just_finished() {
        spawn_event.send_default();
        commands.remove_resource::<Spawner>();
    }
}

fn get_random_points(rng: &mut GlobalEntropy<ChaCha8Rng>) -> usize {
    let idx = rng.gen_range(0..POINTS.len());
    POINTS[idx]
}

#[derive(Event, Default)]
struct Spawn;

fn spawn(
    mut commands: Commands,
    (asset_server, game_board, mut spawn_event, mut rng): (
        Res<AssetServer>,
        Res<game::Board>,
        EventReader<Spawn>,
        ResMut<GlobalEntropy<ChaCha8Rng>>,
    ),
) {
    let first_y_cell = game_board
        .get(1)
        .expect("Game Board has less than 2 Rows")
        .first()
        .expect("Game Board Columns should not be Empty")
        .y;
    for _ in spawn_event.read() {
        commands.spawn((
            Name::new("Ufo"),
            Bundle::new(
                PointsWorth(get_random_points(&mut rng)),
                SpriteBundle {
                    texture: asset_server.load("sprites/ufo.png"),
                    transform: Transform::from_xyz(-X_OFFSET, first_y_cell, 0.0),
                    ..default()
                },
            ),
            AudioBundle {
                source: asset_server.load("sounds/ufo/highpitch.wav"),
                settings: PlaybackSettings {
                    mode: audio::PlaybackMode::Loop,
                    ..default()
                },
            },
        ));
    }
}

fn despawn_out_of_window(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Velocity), With<Ufo>>,
) {
    for (ufo, x_pos, x_vel) in query
        .iter()
        .map(|(ufo, transform, velocity)| (ufo, transform.translation.x, velocity.linvel.x))
    {
        match x_vel.total_cmp(&0.0) {
            Ordering::Less => {
                if x_pos < -X_OFFSET {
                    commands.entity(ufo).despawn();
                }
            }
            Ordering::Equal => unreachable!("Ufo Velocity Should not Be Zero When Out of Window"),
            Ordering::Greater => {
                if x_pos > X_OFFSET {
                    commands.entity(ufo).despawn();
                }
            }
        }
    }
}

fn cleanup(mut commands: Commands, ufos: Query<Entity, With<Ufo>>) {
    for ufo in &ufos {
        commands.entity(ufo).despawn();
    }
    commands.remove_resource::<Spawner>();
}
