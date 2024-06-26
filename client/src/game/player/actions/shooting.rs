use super::Action;
use crate::game::{
    self,
    player::{self, Player},
    projectile,
};
use bevy::{app, audio, ecs::query::QuerySingleError, prelude::*};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;
use std::time::Duration;

const SECONDS_TO_SHOOT: f32 = 0.25;

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Cooldown>()
            .add_systems(OnEnter(game::State::LvlStartup), setup)
            .add_systems(
                Update,
                (tick_timer, shoot).run_if(in_state(game::State::Playing)),
            );
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(Cooldown::default());
}

#[derive(Resource, Deref, DerefMut)]
struct Cooldown(Timer);

impl Default for Cooldown {
    fn default() -> Self {
        let duration = Duration::from_secs_f32(SECONDS_TO_SHOOT);
        let mut timer = Timer::new(duration, TimerMode::Once);
        timer.tick(duration);

        Self(timer)
    }
}

fn tick_timer((time, mut timer): (Res<Time>, ResMut<Cooldown>)) {
    timer.tick(time.delta());
}

fn shoot(
    mut commands: Commands,
    (asset_server, mut projectile_spawn_event, mut cooldown): (
        Res<AssetServer>,
        EventWriter<projectile::Spawn>,
        ResMut<Cooldown>,
    ),
    query: Query<(&Transform, &ActionState<Action>), With<Player>>,
) {
    let (&transform, action_state) = match query.get_single() {
        Ok(p) => p,
        Err(e) => match e {
            QuerySingleError::NoEntities(_) => return,
            QuerySingleError::MultipleEntities(_) => panic!("There Can't be multiple players"),
        },
    };

    if !action_state.just_pressed(&Action::Shoot) {
        return;
    }

    if !cooldown.finished() {
        return;
    }

    let translation = transform.translation;
    let transform = Transform::from_xyz(
        translation.x,
        translation.y + player::DIMENSIONS.y,
        translation.z,
    );

    projectile_spawn_event.send(projectile::Spawn {
        velocity: Velocity::linear(Vec2::new(0.0, 400.0)),
        collision_target_groups: CollisionGroups::new(
            Group::GROUP_3,
            Group::GROUP_2 | Group::GROUP_4,
        ),
        transform,
        dimensions: Vec2::new(6.0, 18.0),
        color: projectile::Color(Color::PINK),
    });

    commands.spawn((
        Name::new("Player Shooting Sound"),
        AudioBundle {
            source: asset_server.load("sounds/player/shoot.wav"),
            settings: PlaybackSettings {
                mode: audio::PlaybackMode::Despawn,
                ..default()
            },
        },
    ));

    cooldown.reset();
}
