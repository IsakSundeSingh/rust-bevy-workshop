use bevy::prelude::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Debug, Default, Clone, Hash, States, PartialEq, Eq)]
enum DebugMode {
    On,
    #[default]
    Off,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<DebugMode>()
        .add_plugins(WorldInspectorPlugin::new().run_if(in_state(DebugMode::On)))
        .add_systems(Startup, (spawn_camera, spawn_santa))
        .add_systems(Update, toggle_debug_mode)
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

fn toggle_debug_mode(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    debug_mode: Res<State<DebugMode>>,
    mut next_state: ResMut<NextState<DebugMode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyD) {
        next_state.set(match debug_mode.get() {
            DebugMode::On => DebugMode::Off,
            DebugMode::Off => DebugMode::On,
        });
    }
}
