use bevy::prelude::*;

#[derive(Resource, Default, Debug)]
pub struct Score {
    pub current: usize,
    pub best: usize,
}
