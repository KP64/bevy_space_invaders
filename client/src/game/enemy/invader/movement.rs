use super::Invader;
use crate::game::{self};
use bevy::{
    app,
    prelude::*,
    tasks::{self, block_on, poll_once, AsyncComputeTaskPool},
    time,
};
use std::time::Duration;

mod direction;

const SECONDS_TILL_MOVE: f32 = 3.0;

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(game::State::LvlStartup), init)
            .add_systems(
                Update,
                (spawn_tasks, handle_tasks).run_if(in_state(game::State::Playing)),
            )
            .add_systems(OnEnter(game::State::LvlFinished), cleanup);
    }
}

fn init(mut commands: Commands) {
    commands.insert_resource(Timer::default());
    commands.insert_resource(direction::Next::default());
}

fn cleanup(mut commands: Commands, tasks: Query<Entity, With<Task>>) {
    for task in &tasks {
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
struct Task(tasks::Task<(Transform, super::Type, Entity)>);

#[derive(Component, Clone, Copy, Deref, DerefMut)]
pub(super) struct Delay(pub(super) f32);

fn spawn_tasks(
    mut commands: Commands,
    (mut movement, mut movement_timer, time): (ResMut<direction::Next>, ResMut<Timer>, Res<Time>),
    (tasks, row_query): (
        Query<&Task>,
        Query<(Entity, &super::Type, &Transform, &Delay), With<Invader>>,
    ),
) {
    if !tasks.is_empty() {
        movement_timer.pause();
        movement_timer.reset();
        return;
    }

    movement_timer.unpause();
    if !movement_timer.tick(time.delta()).finished() {
        return;
    }

    let task_pool = AsyncComputeTaskPool::get();
    let direction = Vec2::from(movement.direction);

    for (entity, &itype, &transform, &delay) in &row_query {
        let invader_id = commands.entity(entity).id();
        let task = task_pool.spawn(async move {
            // FIXME: Task ticks "sleep timer" further even if game is game::state::Paused
            std::thread::sleep(Duration::from_secs_f32(delay.0));

            (
                Transform::from_translation(transform.translation + direction.extend(0.0)),
                itype,
                invader_id,
            )
        });
        commands.spawn((Name::new("Invader Movement Task"), Task(task)));
    }
    movement.next();
}

fn handle_tasks(
    mut commands: Commands,
    asset_loader: Res<AssetServer>,
    mut tasks: Query<(Entity, &mut Task)>,
) {
    for (task, mut movement_task) in &mut tasks {
        let Some((new_position, itype, moving_entity)) = block_on(poll_once(&mut movement_task.0))
        else {
            continue;
        };

        if let Some(mut moving_entity) = commands.get_entity(moving_entity) {
            let nxt_type = itype.next();
            moving_entity.insert((
                nxt_type,
                SpriteBundle {
                    texture: asset_loader.load(nxt_type.to_string()),
                    transform: new_position,
                    ..default()
                },
            ));
        }
        commands.entity(task).despawn();
    }
}
