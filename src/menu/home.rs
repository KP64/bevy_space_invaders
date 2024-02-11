use super::{button, FONT_SIZE, TEXT_COLOR};
use crate::AppState;
use bevy::{
    app::{self, AppExit},
    prelude::*,
};

pub struct Plugin;

#[derive(Component)]
enum Buttons {
    Play,
    Settings,
    Quit,
}

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup)
            .add_systems(Update, menu.run_if(in_state(AppState::MainMenu)));
    }
}

fn setup(mut commands: Commands, mut menu_data: ResMut<super::Entities>) {
    let entity = commands
        .spawn((
            Name::new("Home Menu Node"),
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(setup_header)
        .with_children(setup_play_button)
        .with_children(setup_settings_button)
        .with_children(setup_quit_button)
        .id();
    menu_data.push(entity);
}

fn setup_header(parent: &mut ChildBuilder) {
    parent.spawn((
        Name::new("Home Header"),
        TextBundle::from_section(
            "Space Invaders",
            TextStyle {
                font_size: FONT_SIZE,
                color: TEXT_COLOR,
                ..default()
            },
        )
        .with_style(Style {
            margin: UiRect::bottom(Val::Vh(3.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        }),
    ));
}

fn setup_play_button(parent: &mut ChildBuilder) {
    parent
        .spawn((
            Name::new("Play Button"),
            Buttons::Play,
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
                background_color: button::color::NORMAL.into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Play",
                TextStyle {
                    font_size: FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ));
        });
}

fn setup_settings_button(parent: &mut ChildBuilder) {
    parent
        .spawn((
            Name::new("Settings Button"),
            Buttons::Settings,
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
                "Settings",
                TextStyle {
                    font_size: FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ));
        });
}
fn setup_quit_button(parent: &mut ChildBuilder) {
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

fn menu(
    (mut next_state, mut exit): (ResMut<NextState<AppState>>, EventWriter<AppExit>),
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Buttons),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, button_type) in &mut interaction_query {
        *color = match *interaction {
            Interaction::Pressed => {
                match button_type {
                    Buttons::Play => next_state.set(AppState::Game),
                    Buttons::Settings => next_state.set(AppState::Settings),
                    Buttons::Quit => exit.send_default(),
                }
                button::color::PRESSED
            }
            Interaction::Hovered => button::color::HOVERED,
            Interaction::None => button::color::NORMAL,
        }
        .into();
    }
}
