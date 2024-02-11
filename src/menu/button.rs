use bevy::prelude::*;

pub(super) const NORMAL: Color = Color::rgb(0.15, 0.15, 0.15);
pub(super) const HOVERED: Color = Color::rgb(0.25, 0.25, 0.25);
pub(super) const PRESSED: Color = Color::rgb(0.35, 0.75, 0.35);

pub(super) const MIN_WIDTH: Val = Val::Px(175.0);
pub(super) const MIN_HEIGHT: Val = Val::Px(50.0);
pub(super) const WIDTH: Val = Val::Vw(20.0);
pub(super) const HEIGHT: Val = Val::Vh(10.0);
