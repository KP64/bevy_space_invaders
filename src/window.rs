use crate::{get_single_mut, AppState};
use bevy::{
    app,
    prelude::*,
    window::{close_on_esc, PresentMode, PrimaryWindow, WindowMode},
    winit::WinitWindows,
};
use winit::window::Icon;

#[derive(Event, Default)]
pub struct Fullscreen;

#[derive(Event, Default)]
pub struct VsyncToggle;

pub const DIMENSIONS: Vec2 = Vec2::new(1280.0, 720.0);

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Fullscreen>()
            .add_event::<VsyncToggle>()
            .add_systems(Startup, set_window_icon)
            .add_systems(Update, (close_on_esc, toggle_vsync))
            .add_systems(Update, fullscreen.run_if(in_state(AppState::Settings)));
    }
}

fn fullscreen(
    mut event: EventReader<Fullscreen>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
) {
    for _ in event.read() {
        let mut window = get_single_mut!(window);
        window.mode = if window.mode == WindowMode::Windowed {
            WindowMode::Fullscreen
        } else {
            WindowMode::Windowed
        };
    }
}

fn toggle_vsync(
    mut event: EventReader<VsyncToggle>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = get_single_mut!(window);
    for _ in event.read() {
        let present_mode = if window.present_mode == PresentMode::AutoVsync {
            PresentMode::AutoNoVsync
        } else {
            PresentMode::AutoVsync
        };
        window.present_mode = present_mode;
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
