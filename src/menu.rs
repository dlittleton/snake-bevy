use bevy::prelude::*;

use crate::{colors::GameColors, score::Score, state::GameState};

pub struct MenuPlugin;

const MENU_FONT_SIZE: f32 = 32.0;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), menu_setup);
        app.add_systems(Update, wait_for_space.run_if(in_state(GameState::Menu)));
    }
}

#[derive(Bundle)]
struct TextBundle(Text, TextFont, TextColor);

impl TextBundle {
    fn new(text: impl Into<String>) -> Self {
        Self(
            Text::new(text),
            TextFont {
                font_size: MENU_FONT_SIZE,
                ..default()
            },
            TextColor(GameColors::PRIMARY),
        )
    }

    fn hr() -> Self {
        Self::new("------------------------------------------------------------")
    }
}

fn menu_setup(mut commands: Commands, score: Res<Score>) {
    info!("Setting up menu state");
    commands
        .spawn((
            StateScoped(GameState::Menu),
            Node {
                // General screen container to center children
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((Node {
                    // Text container to stack children in a column
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },))
                .with_children(|p| {
                    p.spawn(TextBundle::new("Snake"));
                    p.spawn(TextBundle::hr());
                    p.spawn(TextBundle::new(format!("High Score: {}", score.best)));
                    p.spawn(TextBundle::new(format!("Last Round: {}", score.current)));
                    p.spawn(TextBundle::hr());
                    p.spawn(TextBundle::new("Press space to start..."));
                });
        });
}

fn wait_for_space(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        game_state.set(GameState::Playing);
    }
}
