use super::GuiData;
use crate::{
    game,
    menu::{button, FONT_SIZE, TEXT_COLOR},
};
use bevy::{app, prelude::*};

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(game::State::GameOver), setup);
    }
}

#[derive(Component)]
enum Buttons {
    Restart,
    LeaderBoard,
    Submit,
    Quit,
}

fn setup(mut commands: Commands, mut ui_data: ResMut<GuiData>) {
    let ui_entity = commands
        .spawn((
            Name::new("Game Over UI Node"),
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
        .with_children(setup_restart_button)
        .with_children(setup_quit)
        .id();

    ui_data.push(ui_entity);
}

fn setup_header(parent: &mut ChildBuilder) {
    parent.spawn((
        Name::new("Game Over Header"),
        TextBundle::from_section(
            "Game Over",
            TextStyle {
                font_size: FONT_SIZE,
                color: Color::WHITE,
                ..default()
            },
        ),
    ));
}

fn setup_restart_button(parent: &mut ChildBuilder) {
    parent
        .spawn((
            Name::new("Restart Button"),
            Buttons::Restart,
            ButtonBundle {
                style: Style {
                    min_width: button::size::MIN_WIDTH,
                    min_height: button::size::MIN_HEIGHT,
                    width: button::size::WIDTH,
                    height: button::size::HEIGHT,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Restart",
                TextStyle {
                    font_size: FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ));
        });
}

fn setup_quit(parent: &mut ChildBuilder) {
    parent
        .spawn((
            Name::new("Quit Button"),
            Buttons::Quit,
            ButtonBundle {
                style: Style {
                    min_width: button::size::MIN_WIDTH,
                    min_height: button::size::MIN_HEIGHT,
                    width: button::size::WIDTH,
                    height: button::size::HEIGHT,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Quit",
                TextStyle {
                    font_size: FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ));
        });
}
