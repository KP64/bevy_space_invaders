use super::Action;
use crate::{
    game::{
        self,
        player::{self, Player},
    },
    window,
};
use bevy::{app, prelude::*};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

const SPEED: f32 = 400.0;
const SIDE_WALLS: f32 = (window::DIMENSIONS.x - player::DIMENSIONS.x) / 2.0;

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (movement, correct_out_of_bounds).run_if(in_state(game::State::Playing)),
        )
        .add_systems(OnEnter(game::State::Paused), reset_velocity);
    }
}

fn movement(mut query: Query<(&mut Velocity, &ActionState<Action>), With<Player>>) {
    for (mut rb_velocity, action_state) in &mut query {
        let move_delta = action_state
            .get_pressed()
            .iter()
            .map(|action| match action {
                Action::Left => Vec2::NEG_X,
                Action::Right => Vec2::X,
                _ => Vec2::ZERO,
            })
            .sum::<Vec2>();

        rb_velocity.linvel = move_delta.normalize_or_zero() * SPEED;
    }
}

fn correct_out_of_bounds(mut transform_query: Query<&mut Transform, With<Player>>) {
    for mut player in &mut transform_query {
        player.translation.x = player.translation.x.clamp(-SIDE_WALLS, SIDE_WALLS);
    }
}

fn reset_velocity(mut velocity_query: Query<&mut Velocity, With<Player>>) {
    for mut rb_velocity in &mut velocity_query {
        *rb_velocity = Velocity::zero();
    }
}
