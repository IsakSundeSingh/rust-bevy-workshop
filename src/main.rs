use bevy::{
    dev_tools::fps_overlay::FpsOverlayConfig,
    math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
    prelude::*,
};

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use noisy_bevy::simplex_noise_2d;

const PRESENT_SCALE: Vec3 = Vec3::new(0.1, 0.1, 0.1);

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

#[derive(Resource, Debug, Default)]
struct Score(u32);

fn main() {
    let mut fps_overlay_plugin = bevy::dev_tools::fps_overlay::FpsOverlayPlugin::default();
    fps_overlay_plugin.config.enabled = false;
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<DebugMode>()
        .add_plugins(fps_overlay_plugin)
        .init_resource::<CursorPosition>()
        .init_resource::<Score>()
        .add_plugins(WorldInspectorPlugin::new().run_if(in_state(DebugMode::On)))
        .add_systems(
            Startup,
            (spawn_camera, spawn_santa, spawn_elf, spawn_score_text),
        )
        .add_systems(
            FixedUpdate,
            (
                toggle_debug_mode,
                move_santa,
                move_elf,
                throw_present,
                move_present,
                handle_santa_present_collisions,
                draw_score,
            ),
        )
        .add_systems(Update, update_cursor_position)
        .run();
}

#[derive(Component)]
struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, MainCamera));
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

    let circle = FunctionCurve::new(Interval::EVERYWHERE, |t| {
        let point = Vec3::new(t.sin(), t.cos(), 0.0);
        point * simplex_noise_2d(point.truncate()) / 1.4
    });

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

#[derive(Component)]
struct Speed(Vec2);

fn throw_present(
    query: Query<&Transform, With<Elf>>,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    cursor_position: Res<CursorPosition>,
    mut commands: Commands,
) {
    let elf_transform = query.single();

    let throw_direction = (cursor_position.0 - elf_transform.translation.truncate()).normalize();

    if keyboard_input.just_pressed(KeyCode::Space) || mouse_input.just_pressed(MouseButton::Left) {
        commands
            .spawn_empty()
            .insert(Present)
            .insert(Transform {
                scale: PRESENT_SCALE,
                ..*elf_transform
            })
            .insert(Speed(throw_direction * 10.0))
            .insert(Sprite::from_image(asset_server.load("present.png")));
    }
}

fn move_present(mut query: Query<(&mut Transform, &Speed), With<Present>>) {
    for (mut present_transform, speed) in query.iter_mut() {
        present_transform.translation += Vec3::new(speed.0.x, speed.0.y, 0.0);
    }
}

/// The last known cursor position
///
/// Is not updated when the cursor is moved outside the main window
#[derive(Resource, Default)]
pub struct CursorPosition(Vec2);

fn update_cursor_position(
    mut cursor_position: ResMut<CursorPosition>,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if window_query.is_empty() || camera_query.is_empty() {
        return;
    }

    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();
    if let Some(position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
    {
        trace!("Cursor moved: {}", position);
        cursor_position.0 = position;
    }
}

fn handle_santa_present_collisions(
    santa: Query<&Transform, With<Santa>>,
    presents: Query<(&Transform, Entity), With<Present>>,
    assets: Res<Assets<Image>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
    mut commands: Commands,
    debug_mode: Res<State<DebugMode>>,
    mut gizmos: Gizmos,
) {
    if santa.is_empty() || presents.is_empty() {
        return;
    }

    let santa_transform = santa.single();
    let santa_bounding_box = {
        let handle = asset_server.load("santa.png");
        let image = assets.get(&handle).unwrap();

        let size = image.size();
        let half_size = size.as_vec2() / 2.0;
        Aabb2d::new(santa_transform.translation.truncate(), half_size)
    };

    let present_half_size = {
        let handle = asset_server.load("present.png");
        let image = assets.get(&handle).unwrap();
        let size = image.size();
        size.as_vec2() / 2.0 * PRESENT_SCALE.truncate()
    };

    if let DebugMode::On = debug_mode.get() {
        gizmos.rect_2d(
            Isometry2d::from_translation(santa_bounding_box.center()),
            santa_bounding_box.half_size() * 2.0,
            Color::srgb(1.0, 0.0, 0.0),
        );
    }

    for (present_transform, entity) in presents.iter() {
        let present_bounding_box =
            Aabb2d::new(present_transform.translation.truncate(), present_half_size);

        if let DebugMode::On = debug_mode.get() {
            gizmos.rect_2d(
                Isometry2d::from_translation(present_bounding_box.center()),
                present_bounding_box.half_size() * 2.0,
                Color::srgb(0.0, 1.0, 0.0),
            );
        }

        if present_bounding_box.intersects(&santa_bounding_box) {
            // Santa caught a present!

            // Try to get the present and despawn it
            if let Some(mut entity) = commands.get_entity(entity) {
                entity.despawn();
            }
            score.0 += 1;
            println!("Santa caught a present! Score: {}", score.0);
        }
    }
}

#[derive(Debug, Component)]
struct ScoreText;

fn spawn_score_text(score: Res<Score>, mut commands: Commands) {
    commands.spawn((Text(format!("Score: {}", score.0)), ScoreText));
}

fn draw_score(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    let mut score_text = query.single_mut();
    score_text.0 = format!("Score: {}", score.0)
}

fn toggle_debug_mode(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    debug_mode: Res<State<DebugMode>>,
    mut fps_overlay_config: ResMut<FpsOverlayConfig>,
    mut next_state: ResMut<NextState<DebugMode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyD) && keyboard_input.pressed(KeyCode::ControlLeft) {
        let debugging = debug_mode.get().toggle();
        fps_overlay_config.enabled = matches!(debugging, DebugMode::On);
        next_state.set(debugging);
    }
}
