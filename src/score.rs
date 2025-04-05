use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Score {
    pub current: usize,
    pub best: usize,
}
