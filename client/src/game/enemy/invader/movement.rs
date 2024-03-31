use super::Invader;
use crate::game;
use bevy::{
    app,
    prelude::*,
    time::{self, Stopwatch},
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

#[derive(Component)]
struct Task {
    transform: Transform,
    itype: super::Type,
    entity: Entity,
    sw: Stopwatch,
    delay: Delay,
}

#[derive(Component, Clone, Copy, Deref, DerefMut)]
pub(super) struct Delay(pub(super) Duration);

fn spawn_tasks(
    mut commands: Commands,
    (mut movement, mut movement_timer, time): (ResMut<direction::Next>, ResMut<Timer>, Res<Time>),
    (tasks, invader_query): (
        Query<(), With<Task>>,
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

    let direction = Vec2::from(movement.direction);

    for (entity, &itype, &trnsfrm, &delay) in &invader_query {
        let task = Task {
            transform: Transform::from_translation(trnsfrm.translation + direction.extend(0.0)),
            itype,
            entity: commands.entity(entity).id(),
            sw: Stopwatch::new(),
            delay,
        };
        commands.spawn(task);
    }
    movement.next();
}

fn handle_tasks(
    mut commands: Commands,
    (asset_loader, time): (Res<AssetServer>, Res<Time>),
    mut tasks: Query<(Entity, &mut Task)>,
) {
    for (task_entity, mut mvmnt_task) in &mut tasks {
        if mvmnt_task.sw.tick(time.delta()).elapsed() < mvmnt_task.delay.0 {
            continue;
        };

        if let Some(mut moving_entity) = commands.get_entity(mvmnt_task.entity) {
            let nxt_type = mvmnt_task.itype.next();
            moving_entity.try_insert((
                nxt_type,
                SpriteBundle {
                    texture: asset_loader.load(nxt_type.to_string()),
                    transform: mvmnt_task.transform,
                    ..default()
                },
            ));
        }
        commands.entity(task_entity).despawn();
    }
}
