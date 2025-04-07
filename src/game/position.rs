use bevy::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/**
 * Grid position associated with an entity.
 */
#[derive(Component)]
pub struct Position(pub usize, pub usize);

impl Position {
    pub fn get_next_position(&self, dir: Direction) -> Self {
        let Position(x, y) = *self;

        match dir {
            Direction::Up => Self(x, y + 1),
            Direction::Down => Self(x, y - 1),
            Direction::Left => Self(x - 1, y),
            Direction::Right => Self(x + 1, y),
        }
    }
}
