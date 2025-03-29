use bevy::{
    prelude::*,
    render::{
        RenderPlugin,
        settings::{Backends, RenderCreation, WgpuSettings},
    },
};

struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HelloTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update, greet_people);
    }
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct HelloTimer(Timer);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("foo".to_string())));
    commands.spawn((Person, Name("bar".to_string())));
    commands.spawn((Person, Name("baz".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<HelloTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                backends: Some(Backends::DX12),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(HelloPlugin)
        .run();
}
