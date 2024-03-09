use crate::{
    game,
    menu::{button, FONT_SIZE, TEXT_COLOR},
};

use super::GuiData;
use bevy::{app, prelude::*};

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(game::State::Leaderboard), setup)
            .add_systems(
                Update,
                leaderboard.run_if(in_state(game::State::Leaderboard)),
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

fn setup(mut commands: Commands, (mut gui_data, score): (ResMut<GuiData>, Res<game::Score>)) {
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
        .with_children(|parent| setup_header(parent, score))
        .with_children(setup_back)
        .id();

    gui_data.push(ui_entity);
}

fn setup_header(parent: &mut ChildBuilder, score: Res<game::Score>) {
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
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                format!("Score: {}", score.0),
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
                    Buttons::Submit => todo!("Submiting is not supported yet"),
                }
                button::color::PRESSED
            }
            Interaction::Hovered => button::color::HOVERED,
            Interaction::None => button::color::NORMAL,
        }
        .into();
    }
}
