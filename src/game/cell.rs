use super::{constants::CELL_SIZE, position::Position};
use crate::{colors::GameColors, state::GameState};
use bevy::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum CellContents {
    Empty,
    Snake,
    Wall,
    Food,
}

#[derive(Bundle)]
pub struct CellBundle(StateScoped<GameState>, Position, Sprite, Transform);

/**
 * Handles converting between grid coordinate and screen coordinates for sprites.
 */
#[derive(Resource)]
pub struct CoordinateTranslator {
    middle: Vec2,
}

impl CoordinateTranslator {
    pub fn new(width: usize, height: usize) -> Self {
        let middle = Vec2 {
            x: (width - 1) as f32 * CELL_SIZE / 2.0,
            y: (height - 1) as f32 * CELL_SIZE / 2.0,
        };

        Self { middle }
    }

    pub fn get_coords(&self, x: usize, y: usize) -> Vec2 {
        let pos = Vec2 {
            x: x as f32 * CELL_SIZE,
            y: y as f32 * CELL_SIZE,
        };

        pos - self.middle
    }
}

impl CellBundle {
    pub fn new(
        contents: CellContents,
        x: usize,
        y: usize,
        translator: &CoordinateTranslator,
    ) -> Self {
        Self(
            StateScoped(GameState::Playing),
            Position(x, y),
            Sprite::from_color(
                match contents {
                    CellContents::Food => GameColors::FOOD,
                    CellContents::Snake => GameColors::PRIMARY,
                    CellContents::Wall => GameColors::WALL,
                    CellContents::Empty => panic!("Attempt to spawn cell contents"),
                },
                Vec2::new(CELL_SIZE, CELL_SIZE),
            ),
            Transform {
                translation: translator.get_coords(x, y).extend(1.0),
                ..default()
            },
        )
    }
}
