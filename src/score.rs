use bevy::{app, prelude::*};

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>();
        app.add_systems(Startup, setup_score_text)
            .add_systems(Update, update_score_text);
    }
}

#[derive(Resource, Default)]
pub struct Score(pub usize);

#[derive(Component)]
struct ScoreText;

fn setup_score_text(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "Text Example",
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

fn update_score_text(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    let Ok(mut text) = query.get_single_mut() else {
        error!("Could not get ScoreText.");
        return;
    };
    text.sections[0].value = score.0.to_string();
}
