use super::{button, Entities, FONT_SIZE, TEXT_COLOR};
use crate::{get_single, get_single_mut, window, AppState};
use bevy::{app, prelude::*, window::PrimaryWindow};

mod vsync;

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Settings), setup)
            .add_systems(
                Update,
                (menu, update_vsync).run_if(in_state(AppState::Settings)),
            );
    }
}

#[derive(Component)]
enum Elements {
    Presentation,
    Vsync,
    Sound,
    Back,
}

fn setup(
    mut commands: Commands,
    mut menu_data: ResMut<Entities>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let node = commands
        .spawn((
            Name::new("Settings Node"),
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
        .with_children(setup_presentation_mode)
        .with_children(|parent| setup_vsync(parent, window))
        .with_children(setup_back)
        .id();
    menu_data.push(node);
}

#[derive(Component)]
struct VsyncText;

fn setup_header(parent: &mut ChildBuilder) {
    parent.spawn((
        Name::new("Settings Header"),
        TextBundle::from_section(
            "Settings",
            TextStyle {
                font_size: FONT_SIZE,
                color: TEXT_COLOR,
                ..default()
            },
        )
        .with_style(Style {
            min_width: button::size::MIN_WIDTH,
            min_height: button::size::MIN_HEIGHT,
            width: button::size::WIDTH,
            height: button::size::HEIGHT,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        }),
    ));
}

fn setup_presentation_mode(parent: &mut ChildBuilder) {
    parent
        .spawn((
            Name::new("Presentation Mode Setter"),
            Elements::Presentation,
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
                "FullScreen",
                TextStyle {
                    font_size: FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ));
        });
}

fn setup_vsync(parent: &mut ChildBuilder, window: Query<&Window, With<PrimaryWindow>>) {
    parent
        .spawn((
            Name::new("Vsync Node"),
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Name::new("Vsync Toggler"),
                    Elements::Vsync,
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
                        "Vsync",
                        TextStyle {
                            font_size: FONT_SIZE,
                            color: TEXT_COLOR,
                            ..default()
                        },
                    ));
                });
        })
        .with_children(|parent| {
            let window = get_single!(window);
            let vsync::Config { text, color } = vsync::Config::try_from(window).unwrap();

            parent.spawn((
                Name::new("Vsync State"),
                VsyncText,
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font_size: FONT_SIZE,
                        color,
                        ..default()
                    },
                ),
            ));
        });
}

fn setup_back(parent: &mut ChildBuilder) {
    parent
        .spawn((
            Name::new("Back Button"),
            Elements::Back,
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
                "Back",
                TextStyle {
                    font_size: FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ));
        });
}

fn menu(
    (mut next_state, mut fullscreen, mut vsync_toggle): (
        ResMut<NextState<AppState>>,
        EventWriter<window::Fullscreen>,
        EventWriter<window::VsyncToggle>,
    ),
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Elements),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, element_type) in &mut interaction_query {
        *color = match *interaction {
            Interaction::Pressed => {
                match element_type {
                    Elements::Presentation => fullscreen.send(window::Fullscreen),
                    Elements::Vsync => vsync_toggle.send(window::VsyncToggle),
                    Elements::Sound => todo!("Sound Capabilites have not been implemented yet."),
                    Elements::Back => next_state.set(AppState::MainMenu),
                }
                button::color::PRESSED
            }
            Interaction::Hovered => button::color::HOVERED,
            Interaction::None => button::color::NORMAL,
        }
        .into();
    }
}

fn update_vsync(
    (window, mut text_query): (
        Query<&Window, With<PrimaryWindow>>,
        Query<&mut Text, With<VsyncText>>,
    ),
) {
    let mut vsync_text = get_single_mut!(text_query);
    let window = get_single!(window);
    let vsync::Config { text, color } = vsync::Config::try_from(window).unwrap();

    let section = &mut vsync_text.sections[0];
    if section.value == text {
        return;
    };
    *section = TextSection {
        value: text.to_string(),
        style: TextStyle {
            font_size: FONT_SIZE,
            color,
            ..default()
        },
    }
}
