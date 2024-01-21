use bevy::{app, prelude::*};

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (tick_timer, despawn));
    }
}

#[derive(Component)]
pub struct Explosion {
    timer: Timer,
}

impl Default for Explosion {
    fn default() -> Self {
        const TIME_TILL_DESPAWN: f32 = 0.5;
        Self {
            timer: Timer::from_seconds(TIME_TILL_DESPAWN, TimerMode::Once),
        }
    }
}

fn tick_timer(time: Res<Time>, mut query: Query<&mut Explosion>) {
    for mut ex in &mut query {
        ex.timer.tick(time.delta());
    }
}

fn despawn(mut commands: Commands, query: Query<(Entity, &Explosion)>) {
    for (entity, _) in query
        .iter()
        .filter(|(_, explosion)| explosion.timer.finished())
    {
        commands.entity(entity).despawn();
    }
}
