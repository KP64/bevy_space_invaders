use super::{cell, Level, Score, Time};
use crate::{game, get_single_mut, AppState};
use bevy::{app, prelude::*, time};

pub struct Plugin;

const FONT_SIZE: f32 = 25.0;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiData>()
            .init_resource::<Time>()
            .add_systems(
                OnTransition {
                    from: AppState::MainMenu,
                    to: AppState::Game,
                },
                setup,
            )
            .add_systems(
                Update,
                (tick_timer, update_time, update_score, update_level)
                    .run_if(in_state(AppState::Game)),
            )
            .add_systems(OnEnter(game::State::Exit), cleanup);
    }
}

#[derive(Component)]
struct TimeText;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct LevelText;

#[derive(Resource, Default, Deref, DerefMut)]
struct UiData(Vec<Entity>);

fn tick_timer((mut timer, time): (ResMut<Time>, Res<time::Time>)) {
    timer.tick(time.delta());
}

fn setup(
    mut commands: Commands,
    (mut ui_data, score, level, time): (ResMut<UiData>, Res<Score>, Res<Level>, Res<Time>),
) {
    let ui_entity = commands
        .spawn((
            Name::new("Game UI Node"),
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(cell::DIMENSIONS.y),
                    justify_content: JustifyContent::SpaceEvenly,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| setup_score(parent, score))
        .with_children(|parent| setup_level(parent, level))
        .with_children(|parent| setup_time(parent, time))
        .id();
    ui_data.push(ui_entity);
}

fn setup_time(parent: &mut ChildBuilder, res: Res<Time>) {
    parent.spawn((
        Name::new("Time Counter"),
        TimeText,
        TextBundle::from_section(
            res.to_string(),
            TextStyle {
                font_size: FONT_SIZE,
                color: Color::WHITE,
                ..default()
            },
        ),
    ));
}

fn update_time(res: Res<Time>, mut query: Query<&mut Text, With<TimeText>>) {
    let mut time_text = get_single_mut!(query);
    time_text.sections[0].value = res.to_string();
}

fn setup_level(parent: &mut ChildBuilder, res: Res<Level>) {
    parent.spawn((
        Name::new("Level Counter"),
        LevelText,
        TextBundle::from_sections([
            TextSection {
                value: "Level\n".to_string(),
                style: TextStyle {
                    font_size: FONT_SIZE,
                    color: Color::WHITE,
                    ..default()
                },
            },
            TextSection {
                value: res.to_string(),
                style: TextStyle {
                    font_size: FONT_SIZE,
                    color: res.get_color(),
                    ..default()
                },
            },
        ]),
    ));
}
fn update_level(level: Res<Level>, mut query: Query<&mut Text, With<LevelText>>) {
    let mut level_text = get_single_mut!(query);

    let level_section = level_text
        .sections
        .get_mut(1)
        .expect("Could not get the Text Section for the Level Count");
    level_section.value = level.to_string();
    level_section.style.color = level.get_color();
}

fn setup_score(parent: &mut ChildBuilder, res: Res<Score>) {
    parent.spawn((
        Name::new("Score Counter"),
        ScoreText,
        TextBundle::from_sections([
            TextSection {
                value: "Score\n".to_string(),
                style: TextStyle {
                    font_size: FONT_SIZE,
                    color: Color::WHITE,
                    ..default()
                },
            },
            TextSection {
                value: res.to_string(),
                style: TextStyle {
                    font_size: FONT_SIZE,
                    color: res.get_color(),
                    ..default()
                },
            },
        ]),
    ));
}

fn update_score(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    let mut score_text = get_single_mut!(query);

    let score_section = score_text
        .sections
        .get_mut(1)
        .expect("Could not get the Text Section for the Score Count");
    score_section.value = score.to_string();
    score_section.style.color = score.get_color();
}

fn cleanup(mut commands: Commands, mut menu_data: ResMut<UiData>) {
    for entity in menu_data.drain(..) {
        commands.entity(entity).despawn_recursive();
    }
}
