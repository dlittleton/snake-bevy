pub mod colors;
pub mod menu;
pub mod score;

use bevy::{
    prelude::*,
    render::{
        RenderPlugin,
        settings::{Backends, RenderCreation, WgpuSettings},
    },
};
use colors::GameColors;
use menu::MenuPlugin;
use score::Score;

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
        .insert_resource(ClearColor(GameColors::BACKGROUND))
        .insert_resource(Score {
            current: 0,
            best: 0,
        })
        .add_plugins(MenuPlugin)
        .run();
}
