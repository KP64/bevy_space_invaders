use crate::game::{self, enemy::Enemy, projectile};
use bevy::{app, prelude::*};
use bevy_rand::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

pub mod probability;

const SECONDS_TILL_SPAWN: f32 = 1.5;
const MAX_XTRA_SECONDS_TILL_SPAWN: f32 = 7.0;

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(game::State::Setup), setup)
            .add_systems(
                Update,
                (tick_cooldown, shoot).run_if(in_state(game::State::Playing)),
            );
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(Probability::default());
}

#[derive(Resource, Deref, DerefMut)]
pub struct Probability(pub f64);

impl Default for Probability {
    fn default() -> Self {
        Self(probability::LOWEST)
    }
}

impl Probability {
    pub fn increase(&mut self) {
        let new_prob = self.0 + probability::INCREMENT_RATE;
        self.0 = new_prob.clamp(probability::LOWEST, probability::HIGHEST);
    }
}

fn tick_cooldown(time: Res<Time>, mut timers: Query<&mut Cooldown>) {
    for mut cooldown in &mut timers {
        cooldown.tick(time.delta());
    }
}

#[derive(Component, Deref, DerefMut)]
pub(super) struct Cooldown(Timer);

impl Default for Cooldown {
    fn default() -> Self {
        // TODO: Replace Timer after each shot?
        // ? For Guidance: Look at the ufo::Spawner
        Self(Timer::from_seconds(
            rand::random::<f32>().mul_add(MAX_XTRA_SECONDS_TILL_SPAWN, SECONDS_TILL_SPAWN),
            TimerMode::Repeating,
        ))
    }
}

fn shoot(
    (mut projectile_spawn_event, res): (EventWriter<projectile::Spawn>, Res<Probability>),
    mut query: Query<
        (
            &Cooldown,
            &mut EntropyComponent<ChaCha8Rng>,
            &GlobalTransform,
        ),
        With<Enemy>,
    >,
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
