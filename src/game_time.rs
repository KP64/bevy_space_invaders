use bevy::{app, prelude::*};

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(Update, update);
    }
}

#[derive(Component)]
struct GameTime;

fn setup(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "Time",
            TextStyle {
                font_size: 30.0,
                ..default()
            },
        )
        .with_style(Style {
            margin: UiRect {
                left: Val::Vw(87.0),
                ..default()
            },
            ..default()
        }),
        GameTime,
        Label,
    ));
}

fn update(time: Res<Time>, mut query: Query<&mut Text, With<GameTime>>) {
    for mut game_time in &mut query {
        let time = time.elapsed().as_secs();
        let (minutes, seconds) = (time / 60, time % 60);

        game_time.sections[0].value = format!("{minutes:02}:{seconds:02}");
    }
}
