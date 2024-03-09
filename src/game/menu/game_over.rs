use super::GuiData;
use crate::{
    game,
    menu::{button, FONT_SIZE, TEXT_COLOR},
};
use bevy::{
    app::{self, AppExit},
    prelude::*,
};

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(game::State::GameOver), setup)
            .add_systems(Update, game_over.run_if(in_state(game::State::GameOver)))
            .add_systems(OnExit(game::State::GameOver), cleanup);
    }
}

fn cleanup(mut commands: Commands, mut menu_data: ResMut<GuiData>) {
    for entity in menu_data.drain(..) {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component)]
enum Buttons {
    Restart,
    LeaderBoard,
    Quit,
}

fn setup(mut commands: Commands, (mut gui_data, score): (ResMut<GuiData>, Res<game::Score>)) {
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
        .with_children(|parent| setup_score(parent, score))
        .with_children(setup_restart)
        .with_children(setup_leaderboard)
        .with_children(setup_quit)
        .id();

    gui_data.push(ui_entity);
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

fn setup_score(parent: &mut ChildBuilder, res: Res<game::Score>) {
    parent.spawn((
        Name::new("Score"),
        TextBundle::from_sections([
            TextSection {
                value: "Score: ".to_string(),
                style: TextStyle {
                    font_size: FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            },
            TextSection {
                value: res.to_string(),
                style: TextStyle {
                    font_size: FONT_SIZE,
                    color: Color::from(*res),
                    ..default()
                },
            },
        ]),
    ));
}

fn setup_restart(parent: &mut ChildBuilder) {
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

fn setup_leaderboard(parent: &mut ChildBuilder) {
    parent
        .spawn((
            Name::new("LeaderBoard Button"),
            Buttons::LeaderBoard,
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
                "Leader Board",
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

fn game_over(
    (mut next_state, mut exit): (ResMut<NextState<game::State>>, EventWriter<AppExit>),
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Buttons),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, button_type) in &mut interaction_query {
        *color = match interaction {
            Interaction::Pressed => {
                match button_type {
                    Buttons::Restart => next_state.set(game::State::Setup),
                    Buttons::LeaderBoard => todo!("Leader Board is still to be implemented!"),
                    Buttons::Quit => {
                        exit.send_default();
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
