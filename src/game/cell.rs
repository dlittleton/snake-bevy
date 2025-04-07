use super::{Game, constants::CELL_SIZE, position::Position};
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

impl CellBundle {
    pub fn new(contents: CellContents, x: usize, y: usize, game: &Game) -> Self {
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
                translation: game.get_coords(x, y).extend(1.0),
                ..default()
            },
        )
    }
}
