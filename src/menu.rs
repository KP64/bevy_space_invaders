use crate::AppState;
use bevy::{app, prelude::*};

pub mod button;
mod home;
pub mod settings;

pub const FONT_SIZE: f32 = 40.0;
pub const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(Resource, Deref, DerefMut, Default)]
struct GuiData(Vec<Entity>);

pub struct Plugin;

// TODO: Rework all GUI
impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GuiData>()
            .add_plugins((home::Plugin, settings::Plugin))
            .add_systems(OnExit(AppState::MainMenu), cleanup)
            .add_systems(OnExit(AppState::Settings), cleanup);
    }
}

fn cleanup(mut commands: Commands, mut menu_data: ResMut<GuiData>) {
    for entity in menu_data.drain(..) {
        commands.entity(entity).despawn_recursive();
    }
}
