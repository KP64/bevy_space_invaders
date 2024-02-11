use super::Action;
use crate::{
    game::{
        self,
        player::{self, Player},
        projectile,
    },
    get_single,
};
use bevy::{app, prelude::*, time};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;
use std::time::Duration;

const SECONDS_TO_SHOOT: f32 = 0.25;

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Timer>().add_systems(
            Update,
            (tick_timer, shoot).run_if(in_state(game::State::Playing)),
        );
    }
}

#[derive(Resource, Deref, DerefMut)]
struct Timer(time::Timer);

impl Default for Timer {
    fn default() -> Self {
        let duration = Duration::from_secs_f32(SECONDS_TO_SHOOT);
        let mut timer = time::Timer::new(duration, TimerMode::Once);
        timer.tick(duration);

        Self(timer)
    }
}

fn tick_timer((time, mut timer): (Res<Time>, ResMut<Timer>)) {
    timer.tick(time.delta());
}

fn shoot(
    (mut projectile_spawn_event, mut shoot_timer): (EventWriter<projectile::Spawn>, ResMut<Timer>),
    query: Query<(&Transform, &ActionState<Action>), With<Player>>,
) {
    let (&transform, action_state) = get_single!(query);

    if !action_state.just_pressed(Action::Shoot) {
        return;
    }

    if !shoot_timer.finished() {
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
    });
    shoot_timer.reset();
}
