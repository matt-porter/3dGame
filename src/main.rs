use bevy::{gltf::Gltf, input::mouse::MouseMotion, prelude::*, window::CursorGrabMode};
use bevy_rapier3d::prelude::*;

const WALK_SPEED: f32 = 5.0;
const RUN_SPEED: f32 = 10.0;
const GRAVITY: f32 = -20.0;
const JUMP_VELOCITY: f32 = 10.0;
const MOUSE_SENSITIVITY: f32 = 0.003;

const WALK_ANIMATION: &str = "Walking_A";
const RUN_ANIMATION: &str = "Running_B";
const ATTACK_ANIMATION: &str = "1H_Melee_Attack_Chop";
const JUMP_ANIMATION: &str = "Jump_Full_Short";

// Camera distance and height behind player
const CAMERA_DISTANCE: f32 = 10.0;
const CAMERA_HEIGHT: f32 = 5.0;

// Castle configuration
const CASTLE_SCALE: f32 = 2.0;
// Player spawns above the castle and falls onto it
const PLAYER_START: Vec3 = Vec3::new(0.0, 15.0, 0.0);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct FollowCamera;

/// Tracks player's yaw rotation (controlled by mouse)
#[derive(Resource, Default)]
struct PlayerYaw(f32);

/// Tracks vertical velocity for jumping
#[derive(Component, Default)]
struct VerticalVelocity(f32);

#[derive(Resource)]
struct KnightGltf(Handle<Gltf>);

#[derive(Resource)]
struct PlayerAnimations {
    graph: Handle<AnimationGraph>,
    walk_index: AnimationNodeIndex,
    run_index: AnimationNodeIndex,
    attack_index: AnimationNodeIndex,
    jump_index: AnimationNodeIndex,
}

#[derive(Component)]
struct AnimationSetupDone;

#[derive(Component, Default)]
struct CurrentAnimation(Option<AnimationNodeIndex>);

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
        .init_resource::<PlayerYaw>()
        .add_systems(Startup, (setup, grab_cursor))
        .add_systems(
            Update,
            (
                load_animations,
                setup_player_animation,
                mouse_look,
                player_movement,
                camera_follow,
            ),
        )
        .run();
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
    // Ground plane (far below the castle)
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(500.0, 500.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.4, 0.2))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        RigidBody::Fixed,
        Collider::halfspace(Vec3::Y).unwrap(),
    ));

    // Castle model (scaled up so player can run on top)
    // AsyncSceneCollider generates trimesh colliders from the scene's meshes
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

    // Load the Knight GLTF (we'll extract named animations once it's loaded)
    let knight_gltf: Handle<Gltf> = asset_server.load("models/Knight.glb");
    commands.insert_resource(KnightGltf(knight_gltf.clone()));

    // Knight character (player) - spawns above the castle and falls onto it
    // Initial rotation faces away from camera (yaw=0 + PI)
    commands.spawn((
        SceneRoot(asset_server.load("models/Knight.glb#Scene0")),
        Transform::from_translation(PLAYER_START)
            .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
        Player,
        VerticalVelocity::default(),
        RigidBody::KinematicPositionBased,
        Collider::capsule_y(0.5, 0.3),
        KinematicCharacterController {
            snap_to_ground: Some(CharacterLength::Absolute(0.5)),
            ..default()
        },
    ));

    // Light (adjusted for larger scene)
    commands.spawn((
        DirectionalLight {
            illuminance: 15000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(20.0, 40.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Camera (follows player)
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, CAMERA_HEIGHT, CAMERA_DISTANCE).looking_at(Vec3::ZERO, Vec3::Y),
        FollowCamera,
    ));
}

// Loads animations by name once the GLTF asset is ready
fn load_animations(
    mut commands: Commands,
    knight_gltf: Option<Res<KnightGltf>>,
    gltf_assets: Res<Assets<Gltf>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    existing_animations: Option<Res<PlayerAnimations>>,
) {
    // Skip if animations already loaded
    if existing_animations.is_some() {
        return;
    }

    let Some(knight_gltf) = knight_gltf else {
        return;
    };

    // Wait for GLTF to load
    let Some(gltf) = gltf_assets.get(&knight_gltf.0) else {
        return;
    };

    // Get the walk animation by name
    let Some(walk_clip) = gltf.named_animations.get(WALK_ANIMATION) else {
        warn!(
            "Animation '{}' not found in Knight.glb. Available animations: {:?}",
            WALK_ANIMATION,
            gltf.named_animations.keys().collect::<Vec<_>>()
        );
        return;
    };

    // Get the run animation by name
    let Some(run_clip) = gltf.named_animations.get(RUN_ANIMATION) else {
        warn!(
            "Animation '{}' not found in Knight.glb. Available animations: {:?}",
            RUN_ANIMATION,
            gltf.named_animations.keys().collect::<Vec<_>>()
        );
        return;
    };

    // Get the attack animation by name
    let Some(attack_clip) = gltf.named_animations.get(ATTACK_ANIMATION) else {
        warn!(
            "Animation '{}' not found in Knight.glb. Available animations: {:?}",
            ATTACK_ANIMATION,
            gltf.named_animations.keys().collect::<Vec<_>>()
        );
        return;
    };

    // Get the jump animation by name
    let Some(jump_clip) = gltf.named_animations.get(JUMP_ANIMATION) else {
        warn!(
            "Animation '{}' not found in Knight.glb. Available animations: {:?}",
            JUMP_ANIMATION,
            gltf.named_animations.keys().collect::<Vec<_>>()
        );
        return;
    };

    // Create animation graph with all animations
    let mut graph = AnimationGraph::new();
    let walk_index = graph.add_clip(walk_clip.clone(), 1.0, graph.root);
    let run_index = graph.add_clip(run_clip.clone(), 1.0, graph.root);
    let attack_index = graph.add_clip(attack_clip.clone(), 1.0, graph.root);
    let jump_index = graph.add_clip(jump_clip.clone(), 1.0, graph.root);
    let graph_handle = graphs.add(graph);

    info!("Loaded animations from Knight.glb");

    commands.insert_resource(PlayerAnimations {
        graph: graph_handle,
        walk_index,
        run_index,
        attack_index,
        jump_index,
    });
}

// Attaches the animation graph to the player's AnimationPlayer once the model is loaded
fn setup_player_animation(
    mut commands: Commands,
    animations: Option<Res<PlayerAnimations>>,
    mut players: Query<(Entity, &mut AnimationPlayer), Without<AnimationSetupDone>>,
    player_query: Query<Entity, With<Player>>,
    children: Query<&Children>,
) {
    let Some(animations) = animations else {
        return;
    };

    // Find the player entity
    let Ok(player_entity) = player_query.get_single() else {
        return;
    };

    // Find the AnimationPlayer in the player's children (GLTF scenes nest the AnimationPlayer)
    for entity in std::iter::once(player_entity).chain(children.iter_descendants(player_entity)) {
        if let Ok((anim_entity, _)) = players.get_mut(entity) {
            // Attach the animation graph, current animation tracker, and mark as set up
            commands.entity(anim_entity).insert((
                AnimationGraphHandle(animations.graph.clone()),
                AnimationSetupDone,
                CurrentAnimation::default(),
            ));
            return;
        }
    }
}

/// Updates player yaw based on mouse movement
fn mouse_look(
    mut mouse_motion: EventReader<MouseMotion>,
    mut yaw: ResMut<PlayerYaw>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let mut delta_x = 0.0;
    for event in mouse_motion.read() {
        delta_x += event.delta.x;
    }

    if delta_x != 0.0 {
        yaw.0 -= delta_x * MOUSE_SENSITIVITY;

        // Update player rotation to face away from camera (add PI to face forward)
        if let Ok(mut transform) = player_query.get_single_mut() {
            transform.rotation = Quat::from_rotation_y(yaw.0 + std::f32::consts::PI);
        }
    }
}

fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    yaw: Res<PlayerYaw>,
    animations: Option<Res<PlayerAnimations>>,
    mut player_query: Query<
        (
            &Transform,
            &mut KinematicCharacterController,
            &mut VerticalVelocity,
            Option<&KinematicCharacterControllerOutput>,
        ),
        With<Player>,
    >,
    children: Query<&Children>,
    player_entity_query: Query<Entity, With<Player>>,
    mut anim_query: Query<(&mut AnimationPlayer, &mut CurrentAnimation)>,
) {
    let Ok((_transform, mut controller, mut vertical_velocity, controller_output)) =
        player_query.get_single_mut()
    else {
        return;
    };

    let Some(animations) = animations else {
        return;
    };
    let Ok(player_entity) = player_entity_query.get_single() else {
        return;
    };

    // Find the animation player
    let Some(anim_entity) = std::iter::once(player_entity)
        .chain(children.iter_descendants(player_entity))
        .find(|e| anim_query.get(*e).is_ok())
    else {
        return;
    };

    let Ok((mut anim_player, mut current_anim)) = anim_query.get_mut(anim_entity) else {
        return;
    };

    let grounded = controller_output.map(|o| o.grounded).unwrap_or(false);

    // Check if currently attacking (attack animation is playing and not finished)
    let is_attacking =
        current_anim.0 == Some(animations.attack_index) && !anim_player.all_finished();

    // Check if currently jumping (jump animation is playing and not finished)
    let is_jumping = current_anim.0 == Some(animations.jump_index) && !anim_player.all_finished();

    // Handle attack input (left mouse button)
    if mouse_button.just_pressed(MouseButton::Left) && !is_attacking && !is_jumping && grounded {
        anim_player.stop_all();
        anim_player.play(animations.attack_index);
        current_anim.0 = Some(animations.attack_index);
        return; // Don't process movement this frame
    }

    // Handle jump input (space bar)
    if keyboard.just_pressed(KeyCode::Space) && grounded && !is_attacking && !is_jumping {
        vertical_velocity.0 = JUMP_VELOCITY;
        anim_player.stop_all();
        anim_player.play(animations.jump_index);
        current_anim.0 = Some(animations.jump_index);
    }

    // Don't allow movement while attacking
    if is_attacking {
        return;
    }

    // Get forward and right vectors based on player yaw
    let forward = Vec3::new(-yaw.0.sin(), 0.0, -yaw.0.cos());
    let right = Vec3::new(forward.z, 0.0, -forward.x);

    // Build movement direction from input
    // W/S = forward/backward, A/D = strafe left/right
    let mut direction = Vec3::ZERO;
    if keyboard.pressed(KeyCode::KeyW) {
        direction += forward;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction -= forward;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction += right;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction -= right;
    }

    let is_moving = direction != Vec3::ZERO;
    let is_running = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
    let speed = if is_running { RUN_SPEED } else { WALK_SPEED };

    // Calculate horizontal movement
    let mut movement = Vec3::ZERO;
    if is_moving {
        direction = direction.normalize();
        movement = direction * speed * time.delta_secs();
    }

    // Apply gravity and vertical velocity
    if grounded && vertical_velocity.0 <= 0.0 {
        vertical_velocity.0 = 0.0;
    } else {
        vertical_velocity.0 += GRAVITY * time.delta_secs();
    }
    movement.y = vertical_velocity.0 * time.delta_secs();

    // Set the desired translation on the character controller
    controller.translation = Some(movement);

    // Control animation based on movement and sprint state (only when grounded and not jumping)
    if grounded && !is_jumping {
        let desired_anim = if is_moving {
            Some(if is_running {
                animations.run_index
            } else {
                animations.walk_index
            })
        } else {
            None
        };

        // Switch animation if needed
        if current_anim.0 != desired_anim {
            anim_player.stop_all();
            if let Some(anim_index) = desired_anim {
                anim_player.play(anim_index).repeat();
            }
            current_anim.0 = desired_anim;
        }
    }
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<FollowCamera>, Without<Player>)>,
    yaw: Res<PlayerYaw>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    let Ok(mut camera_transform) = camera_query.get_single_mut() else {
        return;
    };

    // Calculate camera position behind player based on yaw
    let offset = Vec3::new(
        yaw.0.sin() * CAMERA_DISTANCE,
        CAMERA_HEIGHT,
        yaw.0.cos() * CAMERA_DISTANCE,
    );

    let target_position = player_transform.translation + offset;
    camera_transform.translation = target_position;

    // Look at player
    camera_transform.look_at(player_transform.translation + Vec3::Y * 1.0, Vec3::Y);
}
