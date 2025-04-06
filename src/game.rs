mod grid;

use std::cell::Cell;

use bevy::prelude::*;
use grid::Grid;

use crate::{colors::GameColors, score::Score, state::GameState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            (init_game, spawn_walls, game_setup).chain(),
        );
        app.add_systems(Update, check_timer.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);

const CELL_SIZE: f32 = 10.0;

#[derive(Clone, Copy, Debug)]
enum CellContents {
    Empty,
    Snake,
    Wall,
    Food,
}

#[derive(Resource)]
struct Game {
    grid: Grid<CellContents>,
}

impl Game {
    fn new(width: usize, height: usize) -> Self {
        let mut grid = Grid::new(width, height, CellContents::Empty);

        // Setup walls
        for x in (0..width) {
            *grid.get_mut(x, 0) = CellContents::Wall;
            *grid.get_mut(x, height - 1) = CellContents::Wall;
        }

        for y in (0..height) {
            *grid.get_mut(0, y) = CellContents::Wall;
            *grid.get_mut(width - 1, y) = CellContents::Wall;
        }

        Self { grid }
    }

    fn get_coords(&self, x: usize, y: usize) -> Vec2 {
        let xpos = x as f32 * CELL_SIZE;
        let ypos = y as f32 * CELL_SIZE;

        let midx = (self.grid.width() as f32 * CELL_SIZE) / 2.0;
        let midy = (self.grid.height() as f32 * CELL_SIZE) / 2.0;

        Vec2 {
            x: xpos - midx,
            y: ypos - midy,
        }
    }
}

fn init_game(mut commands: Commands, window_query: Query<&Window>) {
    info!("Initializing game state");

    let window = window_query.single();
    info!("Window size is {} x {}", window.width(), window.height());

    let width = (window.width() / CELL_SIZE).floor() as usize;
    let height = (window.height() / CELL_SIZE).floor() as usize;

    info!("Grid size is {} x {}", width, height);
    let game = Game::new(width, height);

    commands.insert_resource(game);
}

fn spawn_walls(mut commands: Commands, game: Res<Game>) {
    for (x, y, c) in game.grid.enumerate() {
        if matches!(c, CellContents::Wall) {
            commands.spawn((
                StateScoped(GameState::Playing),
                Sprite::from_color(GameColors::WALL, Vec2::new(CELL_SIZE, CELL_SIZE)),
                Transform {
                    translation: game.get_coords(x, y).extend(1.0),
                    ..default()
                },
            ));
        }
    }
}

fn game_setup(mut commands: Commands, window_query: Query<&Window>) {
    commands.spawn((StateScoped(GameState::Playing), Text::new("Running game")));
    commands.insert_resource(GameTimer(Timer::from_seconds(2.0, TimerMode::Once)));
    commands.spawn((
        StateScoped(GameState::Playing),
        Sprite::from_color(GameColors::PRIMARY, Vec2::new(20.0, 20.0)),
    ));
}

fn check_timer(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    mut score: ResMut<Score>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu);
        score.current += 1;
    }
}
