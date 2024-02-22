use self::probability::Probability;
use crate::game::{self, enemy::Enemy, projectile};
use bevy::{app, prelude::*, time};
use bevy_rand::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

pub mod probability;

const SECONDS_TILL_SPAWN: f32 = 1.5;

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Probability>().add_systems(
            Update,
            (tick_timer, shoot).run_if(in_state(game::State::Playing)),
        );
    }
}

fn tick_timer(time: Res<Time>, mut timers: Query<&mut Timer>) {
    for mut timer in &mut timers {
        timer.tick(time.delta());
    }
}

#[derive(Component, Deref, DerefMut)]
pub(super) struct Timer(time::Timer);

impl Default for Timer {
    fn default() -> Self {
        Self(time::Timer::from_seconds(
            rand::random::<f32>().mul_add(7.0, SECONDS_TILL_SPAWN),
            TimerMode::Repeating,
        ))
    }
}

fn shoot(
    (mut projectile_spawn_event, res): (EventWriter<projectile::Spawn>, Res<Probability>),
    mut query: Query<(&Timer, &mut EntropyComponent<ChaCha8Rng>, &GlobalTransform), With<Enemy>>,
) {
    let to_spawn = query
        .iter_mut()
        .filter_map(|(timer, mut rng, glob_transform)| {
            if !(timer.finished() && rng.gen_bool(res.0)) {
                return None;
            }

            Some(projectile::Spawn {
                velocity: Velocity::linear(Vec2::new(0.0, -400.0)),
                collision_target_groups: CollisionGroups::new(
                    Group::GROUP_4,
                    Group::GROUP_1 | Group::GROUP_3,
                ),
                transform: glob_transform.compute_transform(),
            })
        });

    projectile_spawn_event.send_batch(to_spawn);
}
