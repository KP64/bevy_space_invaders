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
pub(super) enum Action {
    Left,
    Right,
    Shoot,
    TogglePause,
}

impl Action {
    pub(super) fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        // * MOVEMENT - START
        input_map.insert_many_to_one([KeyCode::Left, KeyCode::A], Self::Left);
        input_map.insert(GamepadButtonType::DPadLeft, Self::Left);

        input_map.insert_many_to_one([KeyCode::Right, KeyCode::D], Self::Right);
        input_map.insert(GamepadButtonType::DPadRight, Self::Right);
        // * MOVEMENT - END

        input_map.insert(KeyCode::Space, Self::Shoot);
        input_map.insert(GamepadButtonType::RightTrigger2, Self::Shoot);

        input_map.insert(KeyCode::P, Self::TogglePause);
        input_map.insert(GamepadButtonType::Select, Self::TogglePause);

        input_map
    }
}
