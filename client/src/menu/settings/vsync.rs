use bevy::{prelude::*, window::PresentMode};

pub(super) struct Config {
    pub(super) text: &'static str,
    pub(super) color: Color,
}

impl TryFrom<&Window> for Config {
    type Error = String;
    fn try_from(value: &Window) -> Result<Self, Self::Error> {
        match value.present_mode {
            PresentMode::AutoVsync => Ok(ON),
            PresentMode::AutoNoVsync => Ok(OFF),
            e => Err(format!(
                "Only the Auto(No)Vsync Modes should be used! Current Mode: {e:?}"
            )),
        }
    }
}

const ON: Config = Config {
    text: "Vsync on",
    color: Color::LIME_GREEN,
};

const OFF: Config = Config {
    text: "Vsync off",
    color: Color::RED,
};
