pub mod colors;
pub mod game;
pub mod menu;
pub mod score;
pub mod state;

use bevy::{
    prelude::*,
    render::{
        RenderPlugin,
        settings::{Backends, RenderCreation, WgpuSettings},
    },
};
use colors::GameColors;
use game::GamePlugin;
use menu::MenuPlugin;
use score::Score;
use state::GameState;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                backends: Some(Backends::VULKAN),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .init_state::<GameState>()
        .enable_state_scoped_entities::<GameState>()
        .insert_resource(ClearColor(GameColors::BACKGROUND))
        .insert_resource(Score {
            current: 0,
            best: 0,
        })
        .add_plugins(MenuPlugin)
        .add_plugins(GamePlugin)
        .run();
}
