use super::GuiData;
use crate::{
    game,
    menu::{button, FONT_SIZE, TEXT_COLOR},
};
use bevy::{app, prelude::*};
use bevy_jornet::Leaderboard;

pub struct Plugin;

// TODO: Replace bevy_jornet with inhouse leaderboard
impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(game::State::Leaderboard), setup)
            .add_systems(
                Update,
                (leaderboard, display_scores).run_if(in_state(game::State::Leaderboard)),
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
    Submit,
    Back,
}

fn setup(mut commands: Commands, (mut gui_data, leaderboard): (ResMut<GuiData>, Res<Leaderboard>)) {
    leaderboard.refresh_leaderboard();

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
        .with_children(setup_leaderboard)
        .with_children(setup_submit)
        .with_children(setup_back)
        .id();

    gui_data.push(ui_entity);
}

#[derive(Component)]
enum LeaderboardMarker {
    Player,
    Score,
}

fn display_scores(
    mut commands: Commands,
    leaderboard: Res<Leaderboard>,
    root_ui: Query<(Entity, &LeaderboardMarker)>,
) {
    if !leaderboard.is_changed() {
        return;
    }

    let mut leaderboard = leaderboard.get_leaderboard();
    leaderboard.sort_unstable_by(|s1, s2| s2.score.total_cmp(&s1.score));
    leaderboard.truncate(10);
    for (root_entity, marker) in &root_ui {
        commands.entity(root_entity).despawn_descendants();
        for score in &leaderboard {
            commands.entity(root_entity).with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    match marker {
                        LeaderboardMarker::Player => score.player.clone(),
                        LeaderboardMarker::Score => format!("{} ", score.score),
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

fn setup_submit(parent: &mut ChildBuilder) {
    parent
        .spawn((
            Name::new("Leaderboard submit Button"),
            Buttons::Submit,
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
                "Submit",
                TextStyle {
                    font_size: FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ));
        });
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
    (mut next_state, leaderboard, score): (
        ResMut<NextState<game::State>>,
        Res<Leaderboard>,
        Res<game::Score>,
    ),
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
                    Buttons::Submit => {
                        leaderboard
                            .send_score(score.0 as f32)
                            .expect("Can't submit if Player is not signed in");
                        next_state.set(game::State::GameOver);
                    }
                }
                button::color::PRESSED
            }
            Interaction::Hovered => button::color::HOVERED,
            Interaction::None => button::color::NORMAL,
        }
        .into();
    }
}
