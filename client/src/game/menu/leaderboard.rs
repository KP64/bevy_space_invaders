use super::GuiData;
use crate::{
    game,
    menu::{button, FONT_SIZE, TEXT_COLOR},
};
use bevy::{app, prelude::*};
use bevy_simple_text_input::{TextInputBundle, TextInputPlugin, TextInputSubmitEvent};
use utils::Entry;

const HOST_ADDRESS: &str = "http://127.0.0.1:3000";
const TOP_N_SCORES: usize = 8;

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TextInputPlugin)
            .add_systems(
                OnEnter(game::State::Leaderboard),
                (setup, display_scores).chain(),
            )
            .add_systems(
                Update,
                (leaderboard, name_input).run_if(in_state(game::State::Leaderboard)),
            )
            .add_systems(OnExit(game::State::Leaderboard), cleanup);
    }
}

fn cleanup(mut commands: Commands, mut menu_data: ResMut<GuiData>) {
    for entity in menu_data.drain(..) {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component)]
enum Buttons {
    Back,
}

fn setup(mut commands: Commands, mut gui_data: ResMut<GuiData>) {
    let ui_entity = commands
        .spawn((
            Name::new("Leaderboard UI Node"),
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Vh(1.0),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(setup_header)
        .with_children(setup_name_input)
        .with_children(setup_leaderboard)
        .with_children(setup_back)
        .id();

    gui_data.push(ui_entity);
}

#[derive(Component)]
enum LeaderboardMarker {
    Player,
    Score,
}

fn display_scores(mut commands: Commands, root_ui: Query<(Entity, &LeaderboardMarker)>) {
    let Ok(response) = reqwest::blocking::get(HOST_ADDRESS) else {
        return;
    };
    let mut leaderboard = response.json::<Vec<Entry>>().unwrap();

    leaderboard.sort_unstable_by(|s1, s2| s2.score.cmp(&s1.score));
    leaderboard.truncate(TOP_N_SCORES);
    for (root_entity, marker) in &root_ui {
        commands.entity(root_entity).despawn_descendants();
        for entry in &leaderboard {
            commands.entity(root_entity).with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    match marker {
                        LeaderboardMarker::Player => entry.name.clone(),
                        LeaderboardMarker::Score => format!("{} ", entry.score),
                    },
                    TextStyle {
                        font_size: FONT_SIZE,
                        color: TEXT_COLOR,
                        ..default()
                    },
                ));
            });
        }
    }
}

fn setup_leaderboard(parent: &mut ChildBuilder) {
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(300.0),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::all(Val::Px(20.0)),
                            ..default()
                        },

                        ..default()
                    },
                    LeaderboardMarker::Player,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Name",
                        TextStyle {
                            font_size: FONT_SIZE,
                            color: TEXT_COLOR,
                            ..default()
                        },
                    ));
                });
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::all(Val::Px(20.0)),
                            ..default()
                        },
                        ..default()
                    },
                    LeaderboardMarker::Score,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Score",
                        TextStyle {
                            font_size: FONT_SIZE,
                            color: TEXT_COLOR,
                            ..default()
                        },
                    ));
                });
        });
}

fn setup_header(parent: &mut ChildBuilder) {
    parent
        .spawn((
            Name::new("Leaderboard Header"),
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    display: Display::Flex,
                    justify_content: JustifyContent::SpaceEvenly,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Leaderboard",
                TextStyle {
                    font_size: FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ));
        });
}

fn setup_name_input(parent: &mut ChildBuilder) {
    const BORDER_COLOR_ACTIVE: Color = Color::rgb(0.75, 0.52, 1.0);
    const BACKGROUND_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(200.0),
                border: UiRect::all(Val::Px(5.0)),
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            border_color: BORDER_COLOR_ACTIVE.into(),
            background_color: BACKGROUND_COLOR.into(),
            ..default()
        },
        TextInputBundle::default().with_text_style(TextStyle {
            font_size: FONT_SIZE,
            color: TEXT_COLOR,
            ..default()
        }),
    ));
}

fn name_input(
    (mut events, mut next_state, score): (
        EventReader<TextInputSubmitEvent>,
        ResMut<NextState<game::State>>,
        Res<game::Score>,
    ),
) {
    for event in events.read().filter(|e| !e.value.is_empty()) {
        let response = reqwest::blocking::Client::new()
            .post(HOST_ADDRESS)
            .body(format!(
                "{{ \"name\": \"{}\", \"score\": {} }}",
                event.value, score.0
            ))
            .send();
        if response.is_ok() {
            next_state.set(game::State::GameOver);
        }
    }
}

fn setup_back(parent: &mut ChildBuilder) {
    parent
        .spawn((
            Name::new("Leaderboard Back Button"),
            Buttons::Back,
            ButtonBundle {
                style: Style {
                    min_width: button::size::MIN_WIDTH,
                    min_height: button::size::MIN_HEIGHT,
                    width: button::size::WIDTH,
                    height: button::size::HEIGHT,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Vh(1.0)),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Back",
                TextStyle {
                    font_size: FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ));
        });
}

fn leaderboard(
    mut next_state: ResMut<NextState<game::State>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Buttons),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, button_type) in &mut interaction_query {
        *color = match interaction {
            Interaction::Pressed => {
                match button_type {
                    Buttons::Back => next_state.set(game::State::GameOver),
                }
                button::color::PRESSED
            }
            Interaction::Hovered => button::color::HOVERED,
            Interaction::None => button::color::NORMAL,
        }
        .into();
    }
}
