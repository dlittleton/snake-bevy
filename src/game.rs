use bevy::prelude::*;

use crate::{score::Score, state::GameState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), game_setup);
        app.add_systems(Update, check_timer.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);

fn game_setup(mut commands: Commands) {
    info!("Setting up game state");
    commands.spawn((StateScoped(GameState::Playing), Text::new("Running game")));
    commands.insert_resource(GameTimer(Timer::from_seconds(2.0, TimerMode::Once)));
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
