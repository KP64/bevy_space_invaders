use crate::AppState;
use bevy::{
    app,
    prelude::*,
    window::{PresentMode, PrimaryWindow, WindowMode},
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
            .add_systems(Update, toggle_vsync)
            .add_systems(Update, fullscreen.run_if(in_state(AppState::Settings)));
    }
}

fn fullscreen(
    mut event: EventReader<Fullscreen>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
) {
    for _ in event.read() {
        let mut window = window.single_mut();
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
    let mut window = window.single_mut();
    for _ in event.read() {
        window.present_mode = match window.present_mode {
            PresentMode::AutoVsync => PresentMode::AutoNoVsync,
            PresentMode::AutoNoVsync => PresentMode::AutoVsync,
            e => panic!("Only the Auto(No)Vsync Modes should be used! Current Mode: {e:?}"),
        };
    }
}

// TODO: Change this when Bevy adds native Window Icon Support
fn set_window_icon(windows: NonSend<WinitWindows>) {
    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("client/assets/game_icon_x512.png")
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
