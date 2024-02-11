use super::{LevelUp, Score};

use bevy::{app, prelude::*};

mod invader;
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

fn when_hit(mut death_event: EventReader<Death>, mut score: ResMut<Score>) {
    score.0 += death_event
        .read()
        .map(|&Death(PointsWorth(points))| points)
        .sum::<usize>();
}

fn lvl_up(query: Query<(), With<Enemy>>, mut lvl_up_event: EventWriter<LevelUp>) {
    if query.is_empty() {
        lvl_up_event.send_default();
    }
}
