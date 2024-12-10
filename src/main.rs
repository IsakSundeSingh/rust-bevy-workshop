use bevy::prelude::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Debug, Default, Clone, Hash, States, PartialEq, Eq)]
enum DebugMode {
    On,
    #[default]
    Off,
}

impl DebugMode {
    fn toggle(&self) -> Self {
        match self {
            DebugMode::On => DebugMode::Off,
            DebugMode::Off => DebugMode::On,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<DebugMode>()
        .add_plugins(WorldInspectorPlugin::new().run_if(in_state(DebugMode::On)))
        .add_systems(Startup, (spawn_camera, spawn_santa, spawn_elf))
        .add_systems(
            FixedUpdate,
            (
                toggle_debug_mode,
                move_santa,
                move_elf,
                throw_present,
                move_present,
            ),
        )
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Component)]
struct Santa;

fn spawn_santa(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_empty()
        .insert(Santa)
        .insert(Transform::from_xyz(0.0, 0.0, 0.0))
        .insert(Sprite::from_image(asset_server.load("santa.png")));
}

#[derive(Component)]
struct Elf;

fn spawn_elf(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_empty()
        .insert(Elf)
        .insert(Transform::from_xyz(100.0, 100.0, 0.0))
        .insert(Sprite::from_image(asset_server.load("elf.png")));
}

fn move_santa(mut query: Query<&mut Transform, With<Santa>>, time: Res<Time>) {
    let mut santa_transform = query.single_mut();

    let circle = FunctionCurve::new(Interval::EVERYWHERE, |t| Vec3::new(t.sin(), t.cos(), 0.0));

    santa_transform.translation = circle.sample(time.elapsed_secs_wrapped()).unwrap() * 250.0;
}

fn move_elf(
    mut query: Query<&mut Transform, With<Elf>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let mut elf_transform = query.single_mut();

    let move_left = keyboard_input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]);
    let move_right = keyboard_input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]);
    let move_up = keyboard_input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]);
    let move_down = keyboard_input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]);

    let mut delta = Vec2::ZERO;
    if move_left {
        delta.x -= 1.0;
    }
    if move_right {
        delta.x += 1.0;
    }
    if move_down {
        delta.y -= 1.0;
    }
    if move_up {
        delta.y += 1.0;
    }
    let delta = delta.normalize_or_zero();

    let z = elf_transform.translation.z;
    elf_transform.translation += Vec3::new(delta.x, delta.y, z) * 10.0;
}

#[derive(Component)]
struct Present;

fn throw_present(
    query: Query<&Transform, With<Elf>>,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    let elf_transform = query.single();

    if keyboard_input.just_pressed(KeyCode::Space) {
        commands
            .spawn_empty()
            .insert(Present)
            .insert(Transform {
                scale: elf_transform.scale / 10.0,
                ..*elf_transform
            })
            .insert(Sprite::from_image(asset_server.load("present.png")));
    }
}

fn move_present(mut query: Query<&mut Transform, With<Present>>) {
    for mut present_transform in query.iter_mut() {
        present_transform.translation += Vec3::new(10.0, 0.0, 0.0);
    }
}

fn toggle_debug_mode(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    debug_mode: Res<State<DebugMode>>,
    mut next_state: ResMut<NextState<DebugMode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyD) && keyboard_input.pressed(KeyCode::ControlLeft) {
        next_state.set(debug_mode.get().toggle());
    }
}
