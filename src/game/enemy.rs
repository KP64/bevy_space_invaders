use super::{level::LevelUp, Score};
use bevy::{app, prelude::*};

pub(super) mod invader;
mod ufo;

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Death>()
            .add_plugins((invader::Plugin, ufo::Plugin))
            .add_systems(
                Update,
                (when_hit, lvl_up).run_if(in_state(super::State::Playing)),
            );
    }
}

#[derive(Component)]
struct Enemy;

#[derive(Component, Default, Clone, Copy, Deref, DerefMut)]
pub(super) struct PointsWorth(pub(super) usize);

#[derive(Event)]
pub(super) struct Death(pub(super) PointsWorth);

fn when_hit((mut death_event, mut score): (EventReader<Death>, ResMut<Score>)) {
    score.0 += death_event
        .read()
        .map(|&Death(PointsWorth(points))| points)
        .sum::<usize>();
}

fn lvl_up(mut lvl_up_event: EventWriter<LevelUp>, enemies: Query<(), With<Enemy>>) {
    if enemies.is_empty() {
        lvl_up_event.send_default();
    }
}
