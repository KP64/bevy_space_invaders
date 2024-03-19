use bevy::{app, prelude::*};
use leafwing_input_manager::prelude::*;

mod moving;
mod shooting;

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default())
            .add_plugins((moving::Plugin, shooting::Plugin));
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum Action {
    Left,
    Right,
    Shoot,
    TogglePause,
}

impl Action {
    pub(super) fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        // * MOVEMENT - START
        input_map.insert_one_to_many(Self::Left, [KeyCode::ArrowLeft, KeyCode::KeyA]);
        input_map.insert(Self::Left, GamepadButtonType::DPadLeft);

        input_map.insert_one_to_many(Self::Right, [KeyCode::ArrowRight, KeyCode::KeyD]);
        input_map.insert(Self::Right, GamepadButtonType::DPadRight);
        // * MOVEMENT - END

        input_map.insert(Self::Shoot, KeyCode::Space);
        input_map.insert(Self::Shoot, GamepadButtonType::RightTrigger2);

        input_map.insert(Self::TogglePause, KeyCode::KeyP);
        input_map.insert(Self::TogglePause, GamepadButtonType::Select);

        input_map
    }
}
