use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_player))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_santa(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_empty()
        .insert(Sprite::from_image(asset_server.load("santa.png")));
}
