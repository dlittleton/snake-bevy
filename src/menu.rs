use bevy::prelude::*;

pub struct MenuPlugin;

const MENU_FONT_SIZE: f32 = 32.0;
const FONT_COLOR: Color = Color::srgb(0.0, 0.8, 0.0);

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_menu);
    }
}

fn add_menu(mut commands: Commands) {
    commands
        .spawn(Node {
            // General screen container to center children
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((Node {
                    // Text container to stack children in a column
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },))
                .with_children(|p| {
                    p.spawn((
                        Text::new("Press space to start..."),
                        TextFont {
                            font_size: MENU_FONT_SIZE,
                            ..default()
                        },
                        TextColor(FONT_COLOR),
                    ));

                    p.spawn((
                        Text::new("More Text?"),
                        TextFont {
                            font_size: MENU_FONT_SIZE,
                            ..default()
                        },
                    ));
                });
        });
}
