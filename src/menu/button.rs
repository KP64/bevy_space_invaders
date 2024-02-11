pub(super) mod color {
    use bevy::prelude::*;

    pub const NORMAL: Color = Color::rgb(0.15, 0.15, 0.15);
    pub const HOVERED: Color = Color::rgb(0.25, 0.25, 0.25);
    pub const PRESSED: Color = Color::rgb(0.35, 0.75, 0.35);
}

pub(super) mod size {
    use bevy::prelude::*;

    pub const MIN_WIDTH: Val = Val::Px(175.0);
    pub const MIN_HEIGHT: Val = Val::Px(50.0);
    pub const WIDTH: Val = Val::Vw(20.0);
    pub const HEIGHT: Val = Val::Vh(10.0);
}
