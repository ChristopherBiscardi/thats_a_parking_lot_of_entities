use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(
            Color::hex("#1e1e2e")
                .expect("a valid hex color"),
        ))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("ferris.png"),
        ..Default::default()
    });
}
