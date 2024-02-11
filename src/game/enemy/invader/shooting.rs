use crate::game::{self, enemy::Enemy, projectile};
use bevy::{app, prelude::*, time};
use bevy_rand::{prelude::ChaCha8Rng, resource::GlobalEntropy};
use bevy_rapier2d::prelude::*;
use rand::Rng;

const SECONDS_TILL_SPAWN: f32 = 1.5;
const PROBABILITY_TO_SHOOT: f64 = 0.3;
pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (tick_timer, shoot).run_if(in_state(game::State::Playing)),
        );
    }
}

fn tick_timer(time: Res<Time>, mut query: Query<&mut Timer>) {
    for mut timer in &mut query {
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
    (mut projectile_spawn_event, mut rng): (
        EventWriter<projectile::Spawn>,
        ResMut<GlobalEntropy<ChaCha8Rng>>,
    ),
    query: Query<(&Timer, &GlobalTransform), With<Enemy>>,
) {
    for (_, transform) in query
        .iter()
        .filter(|(timer, _)| timer.finished() && rng.gen_bool(PROBABILITY_TO_SHOOT))
    {
        projectile_spawn_event.send(projectile::Spawn {
            velocity: Velocity::linear(Vec2::new(0.0, -400.0)),
            collision_target_groups: CollisionGroups::new(
                Group::GROUP_4,
                Group::GROUP_1 | Group::GROUP_3,
            ),
            transform: transform.compute_transform(),
        });
    }
}
