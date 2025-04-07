mod grid;

use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::GlobalEntropy;
use grid::Grid;
use rand_core::RngCore;

use crate::{colors::GameColors, score::Score, state::GameState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            (init_game, spawn_walls, spawn_snake).chain(),
        );
        app.add_systems(Update, read_input.run_if(in_state(GameState::Playing)));
        app.add_systems(
            FixedUpdate,
            (despawn_tail, move_snake, spawn_food)
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
        app.add_systems(OnExit(GameState::Playing), save_score);
        app.insert_resource(Time::<Fixed>::from_seconds(0.1));
    }
}

const CELL_SIZE: f32 = 10.0;
const INITIAL_LENGTH: usize = 5;
const INITIAL_DIRECTION: Direction = Direction::Right;
const INITIAL_SNAKE_X: usize = 5;

#[derive(Clone, Copy, Debug)]
enum CellContents {
    Empty,
    Snake,
    Wall,
    Food,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
struct Position(usize, usize);

impl Position {
    fn get_next_position(&self, dir: Direction) -> Self {
        let Position(x, y) = *self;

        match dir {
            Direction::Up => Self(x, y + 1),
            Direction::Down => Self(x, y - 1),
            Direction::Left => Self(x - 1, y),
            Direction::Right => Self(x + 1, y),
        }
    }
}

#[derive(Bundle)]
struct CellBundle(StateScoped<GameState>, Position, Sprite, Transform);

impl CellBundle {
    fn new(contents: CellContents, x: usize, y: usize, game: &Game) -> Self {
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

#[derive(Resource)]
struct Game {
    grid: Grid<CellContents>,
    middle: Vec2,
    max_length: usize,
    current_direction: Direction,
    next_direction: Direction,
    snake: VecDeque<Entity>,
    head: Position,
    food: Option<Entity>,
}

impl Game {
    fn new(width: usize, height: usize) -> Self {
        let mut grid = Grid::new(width, height, CellContents::Empty);

        // Setup walls
        for x in 0..width {
            *grid.get_mut(x, 0) = CellContents::Wall;
            *grid.get_mut(x, height - 1) = CellContents::Wall;
        }

        for y in 0..height {
            *grid.get_mut(0, y) = CellContents::Wall;
            *grid.get_mut(width - 1, y) = CellContents::Wall;
        }

        let middle = Vec2 {
            x: (width - 1) as f32 * CELL_SIZE / 2.0,
            y: (height - 1) as f32 * CELL_SIZE / 2.0,
        };

        Self {
            grid,
            middle,
            max_length: INITIAL_LENGTH,
            current_direction: INITIAL_DIRECTION,
            next_direction: INITIAL_DIRECTION,
            snake: VecDeque::new(),
            head: Position(0, 0),
            food: None,
        }
    }

    fn get_coords(&self, x: usize, y: usize) -> Vec2 {
        let pos = Vec2 {
            x: x as f32 * CELL_SIZE,
            y: y as f32 * CELL_SIZE,
        };

        pos - self.middle
    }

    fn move_snake(&mut self, x: usize, y: usize) -> CellContents {
        let contents = *self.grid.get(x, y);
        *self.grid.get_mut(x, y) = CellContents::Snake;

        self.head = Position(x, y);

        contents
    }

    fn clear_position(&mut self, x: usize, y: usize) {
        let p = self.grid.get_mut(x, y);
        assert!(
            matches!(p, CellContents::Snake),
            "Expected cleared cell to be a snake"
        );

        *p = CellContents::Empty;
    }

    fn place_food(&mut self, mut rng: GlobalEntropy<WyRand>) -> Position {
        // Walls on both ends
        let xrange = self.grid.width() as u64 - 2;
        let yrange = self.grid.height() as u64 - 2;

        loop {
            let x = ((rng.next_u64() % xrange) + 1) as usize;
            let y = ((rng.next_u64() % yrange) + 1) as usize;

            let p = self.grid.get_mut(x, y);
            if matches!(p, CellContents::Empty) {
                *p = CellContents::Food;
                return Position(x, y);
            }
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
            commands.spawn(CellBundle::new(CellContents::Wall, x, y, &game));
        }
    }
}

fn spawn_snake(mut commands: Commands, mut game: ResMut<Game>) {
    let x = INITIAL_SNAKE_X;
    let y = game.grid.height() / 2;

    let contents = game.move_snake(x, y);
    assert!(
        matches!(contents, CellContents::Empty),
        "Expected initial position to be empty"
    );

    let entity = commands
        .spawn(CellBundle::new(CellContents::Snake, x, y, &game))
        .id();

    game.snake.push_back(entity);
}

fn read_input(keyboard_input: Res<ButtonInput<KeyCode>>, mut game: ResMut<Game>) {
    if keyboard_input.any_just_pressed([KeyCode::KeyW, KeyCode::ArrowUp])
        && !matches!(game.current_direction, Direction::Down)
    {
        game.next_direction = Direction::Up;
    } else if keyboard_input.any_just_pressed([KeyCode::KeyA, KeyCode::ArrowLeft])
        && !matches!(game.current_direction, Direction::Right)
    {
        game.next_direction = Direction::Left;
    } else if keyboard_input.any_just_pressed([KeyCode::KeyS, KeyCode::ArrowDown])
        && !matches!(game.current_direction, Direction::Up)
    {
        game.next_direction = Direction::Down;
    } else if keyboard_input.any_just_pressed([KeyCode::KeyD, KeyCode::ArrowRight])
        && !matches!(game.current_direction, Direction::Left)
    {
        game.next_direction = Direction::Right;
    }
}

fn despawn_tail(mut commands: Commands, mut game: ResMut<Game>, pos_query: Query<&Position>) {
    while game.snake.len() > game.max_length {
        let tail = game.snake.pop_front().unwrap(); // length checked above

        if let Ok(Position(x, y)) = pos_query.get(tail) {
            game.clear_position(*x, *y);
        }

        commands.entity(tail).despawn();
    }
}

fn move_snake(mut commands: Commands, mut game: ResMut<Game>) {
    let Position(x, y) = game.head.get_next_position(game.next_direction);
    game.current_direction = game.next_direction;

    let contents = game.move_snake(x, y);
    if matches!(contents, CellContents::Snake | CellContents::Wall) {
        // Collision. Game over!
        commands.set_state(GameState::Menu);
        return;
    }

    let entity = commands
        .spawn(CellBundle::new(CellContents::Snake, x, y, &game))
        .id();

    game.snake.push_back(entity);
}

fn spawn_food(mut commands: Commands, mut game: ResMut<Game>, rng: GlobalEntropy<WyRand>) {
    if game.food.is_none() {
        let Position(x, y) = game.place_food(rng);

        let entity = commands
            .spawn(CellBundle::new(CellContents::Food, x, y, &game))
            .id();

        game.food = Some(entity);
    }
}

fn save_score(mut score: ResMut<Score>, game: Res<Game>) {
    info!("Saving score.");
    let round = game.max_length;

    score.current = round;
    score.best = round.max(score.best);
    info!("Score updated {:?}", score);
    info!("Direction is {:?}", game.next_direction);
}
