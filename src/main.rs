use bevy::{gltf::Gltf, prelude::*};

const WALK_SPEED: f32 = 5.0;
const RUN_SPEED: f32 = 10.0;
const WALK_ANIMATION: &str = "Walking_A";
const RUN_ANIMATION: &str = "Running_B";
const ATTACK_ANIMATION: &str = "1H_Melee_Attack_Chop";

// Camera offset from player (looking from behind and above)
const CAMERA_OFFSET: Vec3 = Vec3::new(0.0, 8.0, 15.0);

// Castle configuration
const CASTLE_SCALE: f32 = 15.0;
const CASTLE_FLOOR_HEIGHT: f32 = 7.5; // Height of the castle floor the player walks on
const PLAYER_START: Vec3 = Vec3::new(0.0, CASTLE_FLOOR_HEIGHT, 0.0);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct FollowCamera;

#[derive(Resource)]
struct KnightGltf(Handle<Gltf>);

#[derive(Resource)]
struct PlayerAnimations {
    graph: Handle<AnimationGraph>,
    walk_index: AnimationNodeIndex,
    run_index: AnimationNodeIndex,
    attack_index: AnimationNodeIndex,
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
        .add_systems(Startup, setup)
        .add_systems(Update, (load_animations, setup_player_animation, player_movement, camera_follow))
        .run();
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
        Transform::from_xyz(0.0, -5.0, 0.0),
    ));

    // Castle model (scaled up so player can run on top)
    commands.spawn((
        SceneRoot(asset_server.load("models/castle.glb#Scene0")),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(CASTLE_SCALE)),
    ));

    // Load the Knight GLTF (we'll extract named animations once it's loaded)
    let knight_gltf: Handle<Gltf> = asset_server.load("models/Knight.glb");
    commands.insert_resource(KnightGltf(knight_gltf.clone()));

    // Knight character (player) - spawns on top of the castle
    commands.spawn((
        SceneRoot(asset_server.load("models/Knight.glb#Scene0")),
        Transform::from_translation(PLAYER_START),
        Player,
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
        Transform::from_xyz(CAMERA_OFFSET.x, CAMERA_OFFSET.y, CAMERA_OFFSET.z).looking_at(Vec3::ZERO, Vec3::Y),
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

    // Create animation graph with all animations
    let mut graph = AnimationGraph::new();
    let walk_index = graph.add_clip(walk_clip.clone(), 1.0, graph.root);
    let run_index = graph.add_clip(run_clip.clone(), 1.0, graph.root);
    let attack_index = graph.add_clip(attack_clip.clone(), 1.0, graph.root);
    let graph_handle = graphs.add(graph);

    info!("Loaded animations from Knight.glb");

    commands.insert_resource(PlayerAnimations {
        graph: graph_handle,
        walk_index,
        run_index,
        attack_index,
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

fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    animations: Option<Res<PlayerAnimations>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    children: Query<&Children>,
    player_entity_query: Query<Entity, With<Player>>,
    mut anim_query: Query<(&mut AnimationPlayer, &mut CurrentAnimation)>,
) {
    let Ok(mut transform) = player_query.get_single_mut() else {
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

    // Check if currently attacking (attack animation is playing and not finished)
    let is_attacking = current_anim.0 == Some(animations.attack_index)
        && !anim_player.all_finished();

    // Handle attack input (space bar)
    if keyboard.just_pressed(KeyCode::Space) && !is_attacking {
        anim_player.stop_all();
        anim_player.play(animations.attack_index);
        current_anim.0 = Some(animations.attack_index);
        return; // Don't process movement this frame
    }

    // Don't allow movement while attacking
    if is_attacking {
        return;
    }

    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        direction.z -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction.z += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    let is_moving = direction != Vec3::ZERO;
    let is_running = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
    let speed = if is_running { RUN_SPEED } else { WALK_SPEED };

    if is_moving {
        direction = direction.normalize();
        transform.translation += direction * speed * time.delta_secs();

        // Rotate to face movement direction
        let target_rotation = Quat::from_rotation_y(direction.x.atan2(direction.z));
        transform.rotation = transform.rotation.slerp(target_rotation, 10.0 * time.delta_secs());
    }

    // Floor collision - keep player on the castle floor
    if transform.translation.y < CASTLE_FLOOR_HEIGHT {
        transform.translation.y = CASTLE_FLOOR_HEIGHT;
    }

    // Control animation based on movement and sprint state
    let desired_anim = if is_moving {
        Some(if is_running { animations.run_index } else { animations.walk_index })
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

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<FollowCamera>, Without<Player>)>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    let Ok(mut camera_transform) = camera_query.get_single_mut() else {
        return;
    };

    // Position camera at offset from player
    let target_position = player_transform.translation + CAMERA_OFFSET;
    camera_transform.translation = target_position;

    // Look at player
    camera_transform.look_at(player_transform.translation, Vec3::Y);
}
