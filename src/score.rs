use bevy::{app, prelude::*};

use crate::get_single_mut;

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(Startup, setup)
            .add_systems(Update, update);
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct Score(pub usize);

#[derive(Component)]
struct ScoreText;

fn setup(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "Score",
            TextStyle {
                font_size: 30.0,
                ..default()
            },
        )
        .with_style(Style {
            margin: UiRect {
                left: Val::Percent(50.),
                right: Val::Percent(50.),
                ..default()
            },
            ..default()
        }),
        ScoreText,
        Label,
    ));
}

fn update(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    let mut text = get_single_mut!(query);
    text.sections[0].value = score.to_string();
}
