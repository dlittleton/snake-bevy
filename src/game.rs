use bevy::prelude::*;

use crate::{colors::GameColors, score::Score, state::GameState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), game_setup);
        app.add_systems(Update, check_timer.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);

fn game_setup(mut commands: Commands, window_query: Query<&Window>) {
    info!("Setting up game state");

    let window = window_query.single();
    info!("Window size is {} x {}", window.width(), window.height());

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
