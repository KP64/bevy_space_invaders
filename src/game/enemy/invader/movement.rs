use super::Row;
use crate::game::{self};
use bevy::{
    app,
    prelude::*,
    tasks::{self, block_on, AsyncComputeTaskPool},
    time,
};
use futures_lite::future;
use std::time::{Duration, Instant};

mod direction;

const SECONDS_TILL_MOVE: f32 = 3.0;

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(game::State::Setup), init)
            .add_systems(
                Update,
                (spawn_tasks, handle_tasks).run_if(in_state(game::State::Playing)),
            )
            .add_systems(OnEnter(game::State::LvlFinished), cleanup);
    }
}

fn init(mut commands: Commands) {
    commands.init_resource::<Timer>();
    commands.init_resource::<direction::Next>();
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<Task>>) {
    for task in &query {
        commands.entity(task).despawn();
    }
    commands.remove_resource::<Timer>();
    commands.remove_resource::<direction::Next>();
}

#[derive(Resource, Deref, DerefMut)]
struct Timer(time::Timer);
impl Default for Timer {
    fn default() -> Self {
        Self(time::Timer::from_seconds(
            SECONDS_TILL_MOVE,
            TimerMode::Repeating,
        ))
    }
}

#[derive(Component, Deref, DerefMut)]
struct Task(tasks::Task<(Transform, Entity)>);

/// The Movement Delays of each Invader `Row`
/// going from top to bottom.
/// The bottom Rows will Move first
const MOVE_DELAYS: [f32; 5] = [0.8, 0.6, 0.4, 0.2, 0.0];

fn spawn_tasks(
    mut commands: Commands,
    (mut movement_direction, mut movement_timer, time): (
        ResMut<direction::Next>,
        ResMut<Timer>,
        Res<Time>,
    ),
    (task_query, row_query): (Query<&Task>, Query<(Entity, &Transform), With<Row>>),
) {
    if !task_query.is_empty() {
        movement_timer.pause();
        movement_timer.reset();
        return;
    }

    movement_timer.unpause();
    if !movement_timer.tick(time.delta()).finished() {
        return;
    }

    let thread_pool = AsyncComputeTaskPool::get();
    let direction = movement_direction.to_vec2();

    for (entity, translation, delay) in
        row_query
            .iter()
            .zip(MOVE_DELAYS)
            .map(|((entity, transform), delay)| {
                (
                    entity,
                    transform.translation,
                    Duration::from_secs_f32(delay),
                )
            })
    {
        let row = commands.entity(entity).id();
        let task = thread_pool.spawn(async move {
            let start_time = Instant::now();

            // TODO: Replace with std::thread::sleep(duration) ?
            while start_time.elapsed() < delay {}

            (
                Transform::from_translation(translation + direction.extend(0.0)),
                row,
            )
        });
        commands.spawn((Name::new("Invader Movement Task"), Task(task)));
    }
    movement_direction.next();
}

fn handle_tasks(mut commands: Commands, mut move_tasks: Query<(Entity, &mut Task)>) {
    for (task, mut movement_task) in &mut move_tasks {
        let Some((new_position, moving_entity)) = block_on(future::poll_once(&mut movement_task.0))
        else {
            continue;
        };

        commands.entity(moving_entity).insert(new_position);
        commands.entity(task).despawn();
    }
}