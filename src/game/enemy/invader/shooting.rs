use crate::game::{self, enemy::Enemy, projectile};
use bevy::{app, prelude::*, time};
use bevy_rand::{prelude::ChaCha8Rng, resource::GlobalEntropy};
use bevy_rapier2d::prelude::*;
use rand::Rng;

use self::probability::Probability;

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
        // TODO: Try to switch this to bevy_rand ?
        Self(time::Timer::from_seconds(
            rand::random::<f32>().mul_add(7.0, SECONDS_TILL_SPAWN),
            TimerMode::Repeating,
        ))
    }
}

fn shoot(
    (mut projectile_spawn_event, res, mut rng): (
        EventWriter<projectile::Spawn>,
        Res<Probability>,
        ResMut<GlobalEntropy<ChaCha8Rng>>,
    ),
    query: Query<(&Timer, &GlobalTransform), With<Enemy>>,
) {
    let to_spawn = query
        .iter()
        .filter(|(timer, _)| timer.finished() && rng.gen_bool(res.0))
        .map(|(_, glob_transform)| projectile::Spawn {
            velocity: Velocity::linear(Vec2::new(0.0, -400.0)),
            collision_target_groups: CollisionGroups::new(
                Group::GROUP_4,
                Group::GROUP_1 | Group::GROUP_3,
            ),
            transform: glob_transform.compute_transform(),
        });
    projectile_spawn_event.send_batch(to_spawn);
}
