use bevy::prelude::*;

#[non_exhaustive]
pub struct GameColors;

impl GameColors {
    pub const BACKGROUND: Color = Color::srgb(0.13, 0.17, 0.20);
    pub const PRIMARY: Color = Color::srgb(0.34, 0.87, 0.42);
    pub const WALL: Color = Color::srgb(0.16, 0.45, 0.13);
    pub const FOOD: Color = Color::srgb(0.95, 0.62, 0.30);
}
