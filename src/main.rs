use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy_rapier3d::prelude::*;

use threegame::{
    states::AppState,
    core::{InputPlugin, CameraPlugin},
    gameplay::{PlayerPlugin, CombatPlugin, AIPlugin, HealthPlugin},
    visual::{ParticlePlugin, HealthBarPlugin},
    Player, Enemy, EnemyAi, Health, CombatStatus, Stamina, FollowCamera,
    KnightGltf, PLAYER_START,
};

const CASTLE_SCALE: f32 = 2.0;
const CHARACTER_SCALE: f32 = 0.2;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "3D Game".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .init_state::<AppState>()
        .add_plugins((
            InputPlugin,
            CameraPlugin,
            PlayerPlugin,
            CombatPlugin,
            AIPlugin,
            HealthPlugin,
            ParticlePlugin,
            HealthBarPlugin,
        ))
        .add_systems(Startup, (setup, grab_cursor))
        .add_systems(OnEnter(AppState::Loading), transition_to_playing)
        .run();
}

fn transition_to_playing(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::Playing);
}

fn grab_cursor(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.cursor_options.grab_mode = CursorGrabMode::Locked;
    window.cursor_options.visible = false;
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let ground_texture: Handle<Image> = asset_server.load("textures/ground.png");
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(500.0, 500.0).build())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(ground_texture),
            uv_transform: bevy::math::Affine2::from_scale(Vec2::splat(100.0)),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        RigidBody::Fixed,
        Collider::halfspace(Vec3::Y).unwrap(),
    ));

    commands.spawn((
        SceneRoot(asset_server.load("models/castle.glb#Scene0")),
        Transform::from_xyz(2.0 * CASTLE_SCALE, 0.0, 7.0 * CASTLE_SCALE)
            .with_scale(Vec3::splat(CASTLE_SCALE)),
        AsyncSceneCollider {
            shape: Some(ComputedColliderShape::TriMesh(TriMeshFlags::default())),
            named_shapes: default(),
        },
        RigidBody::Fixed,
    ));

    let knight_gltf: Handle<bevy::gltf::Gltf> = asset_server.load("models/Knight.glb");
    commands.insert_resource(KnightGltf(knight_gltf.clone()));

    commands.spawn((
        SceneRoot(asset_server.load("models/Knight.glb#Scene0")),
        Transform::from_translation(PLAYER_START)
            .with_rotation(Quat::from_rotation_y(std::f32::consts::PI))
            .with_scale(Vec3::splat(CHARACTER_SCALE)),
        Player,
        Health { current: 200.0, max: 200.0 },
        Stamina::default(),
        CombatStatus::default(),
        threegame::gameplay::player::VerticalVelocity::default(),
        RigidBody::KinematicPositionBased,
        Collider::capsule_y(0.5 * CHARACTER_SCALE, 0.3 * CHARACTER_SCALE),
        KinematicCharacterController {
            snap_to_ground: Some(CharacterLength::Absolute(0.1)),
            ..default()
        },
    ));

    let enemy_positions = [
        Vec3::new(-3.0, 15.0, -3.0),
        Vec3::new(2.0, 15.0, -2.0),
        Vec3::new(-2.0, 15.0, 2.0),
    ];

    for pos in enemy_positions {
        commands.spawn((
            SceneRoot(asset_server.load("models/Knight.glb#Scene0")),
            Transform::from_translation(pos)
                .with_scale(Vec3::splat(CHARACTER_SCALE)),
            Enemy,
            EnemyAi {
                home_position: pos,
                ..default()
            },
            Health::default(),
            CombatStatus::default(),
            RigidBody::KinematicPositionBased,
            Collider::capsule_y(0.5 * CHARACTER_SCALE, 0.3 * CHARACTER_SCALE),
            KinematicCharacterController {
                snap_to_ground: Some(CharacterLength::Absolute(0.1)),
                ..default()
            },
        ));
    }

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, PLAYER_START.y + 1.0, 2.0).looking_at(PLAYER_START, Vec3::Y),
        FollowCamera,
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(50.0, 100.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 300.0,
    });
}
