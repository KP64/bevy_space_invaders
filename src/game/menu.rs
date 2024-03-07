use bevy::{app, prelude::*};

mod game_over;

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GuiData>()
            .add_plugins(game_over::Plugin);
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
struct GuiData(Vec<Entity>);
