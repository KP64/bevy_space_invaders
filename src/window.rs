use bevy::{
    app,
    prelude::*,
    window::{close_on_esc, PresentMode},
    winit::WinitWindows,
};
use winit::window::Icon;

use crate::get_single_mut;

pub const WIDTH: u16 = 1280;
pub const HEIGHT: u16 = 720;
pub const DIMENSIONS: Vec2 = Vec2::new(WIDTH as f32, HEIGHT as f32);
pub const HALF_WIDTH: u16 = WIDTH / 2;
pub const HALF_HEIGHT: u16 = HEIGHT / 2;

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, set_window_icon)
            .add_systems(Update, (close_on_esc, toggle_vsync));
    }
}

// TODO: Change this when Bevy adds native Window Icon Support
fn set_window_icon(windows: NonSend<WinitWindows>) {
    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/game_icon_x512.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    // do it for all windows
    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}

/// This system toggles the vsync mode when pressing the button V.
/// You'll see fps increase displayed in the console.
fn toggle_vsync(input: Res<Input<KeyCode>>, mut window_query: Query<&mut Window>) {
    if !input.just_pressed(KeyCode::V) {
        return;
    }

    let mut window = get_single_mut!(window_query);

    window.present_mode = if window.present_mode == PresentMode::AutoVsync {
        PresentMode::AutoNoVsync
    } else {
        PresentMode::AutoVsync
    };

    info!("PRESENT_MODE: {:?}", window.present_mode);
}
