use bevy::{gltf::Gltf, input::mouse::MouseMotion, prelude::*, window::CursorGrabMode};
use bevy_rapier3d::prelude::*;

const WALK_SPEED: f32 = 5.0;
const RUN_SPEED: f32 = 10.0;
const GRAVITY: f32 = -20.0;
const JUMP_VELOCITY: f32 = 10.0;
const MOUSE_SENSITIVITY: f32 = 0.003;

// Enemy AI constants
const ENEMY_WALK_SPEED: f32 = 2.0;
const ENEMY_CHASE_SPEED: f32 = 4.0;
const ENEMY_DETECTION_RANGE: f32 = 8.0;
const ENEMY_ATTACK_RANGE: f32 = 2.0;
const ENEMY_PATROL_RANGE: f32 = 3.0;

const WALK_ANIMATION: &str = "Walking_A";
const RUN_ANIMATION: &str = "Running_B";
const ATTACK_ANIMATION: &str = "1H_Melee_Attack_Chop";
const JUMP_ANIMATION: &str = "Jump_Full_Short";
const IDLE_ANIMATION: &str = "Idle";
const HIT_ANIMATION: &str = "Hit_A";
const DEATH_ANIMATION: &str = "Death_A";
const BLOCK_ANIMATION: &str = "Blocking";

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
struct Enemy;

#[derive(Component, Default)]
struct EnemyAi {
    state: AiState,
    home_position: Vec3,
    patrol_target: Option<Vec3>,
    state_timer: f32,
}

#[derive(Default, Clone, Copy, PartialEq)]
enum AiState {
    #[default]
    Idle,
    Patrol,
    Chase,
    Attack,
}

#[derive(Component)]
struct Health {
    current: f32,
    max: f32,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            current: 100.0,
            max: 100.0,
        }
    }
}

#[derive(Component, Default)]
struct CombatState {
    is_blocking: bool,
    is_hit: bool,
    is_dead: bool,
    hit_timer: f32,
    death_timer: f32,
}

const DEATH_DESPAWN_TIME: f32 = 3.0; // Seconds after death before despawning

/// Marker for the attack hitbox sensor
#[derive(Component)]
struct AttackHitbox;

/// Event fired when a hit lands (for spawning impact effects)
#[derive(Event)]
struct HitEvent {
    position: Vec3,
    blocked: bool,
}

/// Marker for spark particles
#[derive(Component)]
struct SparkParticle {
    lifetime: f32,
    velocity: Vec3,
}

#[derive(Component)]
struct FollowCamera;

/// Marker for player health bar UI
#[derive(Component)]
struct PlayerHealthBar;

/// Marker for enemy health bar (world-space billboard)
#[derive(Component)]
struct EnemyHealthBar {
    enemy: Entity,
}

/// Tracks player's yaw rotation (controlled by mouse)
#[derive(Resource, Default)]
struct PlayerYaw(f32);

/// Tracks vertical velocity for jumping
#[derive(Component, Default)]
struct VerticalVelocity(f32);

#[derive(Resource)]
struct KnightGltf(Handle<Gltf>);

#[derive(Resource)]
struct GameAnimations {
    graph: Handle<AnimationGraph>,
    idle_index: AnimationNodeIndex,
    walk_index: AnimationNodeIndex,
    run_index: AnimationNodeIndex,
    attack_index: AnimationNodeIndex,
    jump_index: AnimationNodeIndex,
    hit_index: AnimationNodeIndex,
    death_index: AnimationNodeIndex,
    block_index: AnimationNodeIndex,
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
        .add_event::<HitEvent>()
        .add_systems(Startup, (setup, grab_cursor, setup_player_health_ui))
        .add_systems(
            Update,
            (
                load_animations,
                setup_character_animations,
                mouse_look,
                player_movement,
                enemy_ai,
                combat_system,
                spawn_impact_sparks,
                update_particles,
                update_player_health_bar,
                spawn_enemy_health_bars,
                update_enemy_health_bars,
                despawn_dead_enemies,
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
        Health { current: 200.0, max: 200.0 },
        CombatState::default(),
        VerticalVelocity::default(),
        RigidBody::KinematicPositionBased,
        Collider::capsule_y(0.5, 0.3),
        KinematicCharacterController {
            snap_to_ground: Some(CharacterLength::Absolute(0.5)),
            ..default()
        },
    ));

    // Spawn enemies at various positions on the castle
    let enemy_positions = [
        Vec3::new(-3.0, 15.0, -3.0),
        Vec3::new(2.0, 15.0, -2.0),
        Vec3::new(-2.0, 15.0, 2.0),
    ];

    for pos in enemy_positions {
        commands.spawn((
            SceneRoot(asset_server.load("models/Knight.glb#Scene0")),
            Transform::from_translation(pos),
            Enemy,
            EnemyAi {
                home_position: pos,
                ..default()
            },
            Health::default(),
            CombatState::default(),
            RigidBody::KinematicPositionBased,
            Collider::capsule_y(0.5, 0.3),
            KinematicCharacterController {
                snap_to_ground: Some(CharacterLength::Absolute(0.5)),
                ..default()
            },
        ));
    }

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
    existing_animations: Option<Res<GameAnimations>>,
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

    // Helper to get animation clip or warn
    let get_clip = |name: &str| -> Option<Handle<AnimationClip>> {
        gltf.named_animations.get(name).cloned().or_else(|| {
            warn!(
                "Animation '{}' not found in Knight.glb. Available: {:?}",
                name,
                gltf.named_animations.keys().collect::<Vec<_>>()
            );
            None
        })
    };

    let Some(idle_clip) = get_clip(IDLE_ANIMATION) else { return };
    let Some(walk_clip) = get_clip(WALK_ANIMATION) else { return };
    let Some(run_clip) = get_clip(RUN_ANIMATION) else { return };
    let Some(attack_clip) = get_clip(ATTACK_ANIMATION) else { return };
    let Some(jump_clip) = get_clip(JUMP_ANIMATION) else { return };
    let Some(hit_clip) = get_clip(HIT_ANIMATION) else { return };
    let Some(death_clip) = get_clip(DEATH_ANIMATION) else { return };
    let Some(block_clip) = get_clip(BLOCK_ANIMATION) else { return };

    // Create animation graph with all animations
    let mut graph = AnimationGraph::new();
    let idle_index = graph.add_clip(idle_clip, 1.0, graph.root);
    let walk_index = graph.add_clip(walk_clip, 1.0, graph.root);
    let run_index = graph.add_clip(run_clip, 1.0, graph.root);
    let attack_index = graph.add_clip(attack_clip, 1.0, graph.root);
    let jump_index = graph.add_clip(jump_clip, 1.0, graph.root);
    let hit_index = graph.add_clip(hit_clip, 1.0, graph.root);
    let death_index = graph.add_clip(death_clip, 1.0, graph.root);
    let block_index = graph.add_clip(block_clip, 1.0, graph.root);
    let graph_handle = graphs.add(graph);

    info!("Loaded animations from Knight.glb");

    commands.insert_resource(GameAnimations {
        graph: graph_handle,
        idle_index,
        walk_index,
        run_index,
        attack_index,
        jump_index,
        hit_index,
        death_index,
        block_index,
    });
}

// Attaches the animation graph to all characters (player and enemies) once their models are loaded
fn setup_character_animations(
    mut commands: Commands,
    animations: Option<Res<GameAnimations>>,
    mut anim_players: Query<(Entity, &mut AnimationPlayer), Without<AnimationSetupDone>>,
    characters: Query<Entity, Or<(With<Player>, With<Enemy>)>>,
    children: Query<&Children>,
) {
    let Some(animations) = animations else {
        return;
    };

    // Find all character entities (player and enemies)
    for character_entity in characters.iter() {
        // Find the AnimationPlayer in the character's children (GLTF scenes nest the AnimationPlayer)
        for entity in std::iter::once(character_entity).chain(children.iter_descendants(character_entity)) {
            if let Ok((anim_entity, _)) = anim_players.get_mut(entity) {
                // Attach the animation graph, current animation tracker, and mark as set up
                commands.entity(anim_entity).insert((
                    AnimationGraphHandle(animations.graph.clone()),
                    AnimationSetupDone,
                    CurrentAnimation::default(),
                ));
                break;
            }
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
    animations: Option<Res<GameAnimations>>,
    mut player_query: Query<
        (
            &Transform,
            &mut KinematicCharacterController,
            &mut VerticalVelocity,
            Option<&KinematicCharacterControllerOutput>,
            &CombatState,
        ),
        With<Player>,
    >,
    children: Query<&Children>,
    player_entity_query: Query<Entity, With<Player>>,
    mut anim_query: Query<(&mut AnimationPlayer, &mut CurrentAnimation)>,
) {
    let Ok((_transform, mut controller, mut vertical_velocity, controller_output, combat_state)) =
        player_query.get_single_mut()
    else {
        return;
    };

    // Don't process movement or animations if player is dead
    if combat_state.is_dead {
        return;
    }

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

fn enemy_ai(
    time: Res<Time>,
    animations: Option<Res<GameAnimations>>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<
        (
            Entity,
            &mut Transform,
            &mut EnemyAi,
            &mut KinematicCharacterController,
        ),
        (With<Enemy>, Without<Player>),
    >,
    children: Query<&Children>,
    mut anim_query: Query<(&mut AnimationPlayer, &mut CurrentAnimation)>,
) {
    let Some(animations) = animations else {
        return;
    };

    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    let player_pos = player_transform.translation;

    for (enemy_entity, mut transform, mut ai, mut controller) in enemy_query.iter_mut() {
        let enemy_pos = transform.translation;
        let distance_to_player = enemy_pos.distance(player_pos);
        let direction_to_player = (player_pos - enemy_pos).normalize_or_zero();

        // Update state timer
        ai.state_timer += time.delta_secs();

        // State transitions
        let new_state = match ai.state {
            AiState::Idle => {
                if distance_to_player < ENEMY_DETECTION_RANGE {
                    AiState::Chase
                } else if ai.state_timer > 3.0 {
                    ai.state_timer = 0.0;
                    AiState::Patrol
                } else {
                    AiState::Idle
                }
            }
            AiState::Patrol => {
                if distance_to_player < ENEMY_DETECTION_RANGE {
                    AiState::Chase
                } else if ai.patrol_target.is_none() || ai.state_timer > 5.0 {
                    ai.state_timer = 0.0;
                    AiState::Idle
                } else {
                    AiState::Patrol
                }
            }
            AiState::Chase => {
                if distance_to_player < ENEMY_ATTACK_RANGE {
                    ai.state_timer = 0.0;
                    AiState::Attack
                } else if distance_to_player > ENEMY_DETECTION_RANGE * 1.5 {
                    AiState::Idle
                } else {
                    AiState::Chase
                }
            }
            AiState::Attack => {
                if ai.state_timer > 1.0 {
                    ai.state_timer = 0.0;
                    if distance_to_player < ENEMY_ATTACK_RANGE {
                        AiState::Attack
                    } else {
                        AiState::Chase
                    }
                } else {
                    AiState::Attack
                }
            }
        };

        // Handle state change
        if new_state != ai.state {
            ai.state = new_state;
            if new_state == AiState::Patrol {
                // Pick a random patrol point near home
                let angle = ai.state_timer * 1000.0; // pseudo-random
                ai.patrol_target = Some(
                    ai.home_position
                        + Vec3::new(angle.cos() * ENEMY_PATROL_RANGE, 0.0, angle.sin() * ENEMY_PATROL_RANGE),
                );
            }
        }

        // Execute current state behavior
        let mut movement = Vec3::ZERO;
        let mut desired_anim = None;

        match ai.state {
            AiState::Idle => {
                desired_anim = Some(animations.idle_index);
            }
            AiState::Patrol => {
                if let Some(target) = ai.patrol_target {
                    let dir = (target - enemy_pos).normalize_or_zero();
                    movement = dir * ENEMY_WALK_SPEED * time.delta_secs();
                    movement.y = GRAVITY * time.delta_secs();

                    // Face movement direction
                    if dir.length_squared() > 0.01 {
                        transform.rotation = Quat::from_rotation_y(dir.x.atan2(dir.z));
                    }
                }
                desired_anim = Some(animations.walk_index);
            }
            AiState::Chase => {
                let dir = Vec3::new(direction_to_player.x, 0.0, direction_to_player.z).normalize_or_zero();
                movement = dir * ENEMY_CHASE_SPEED * time.delta_secs();
                movement.y = GRAVITY * time.delta_secs();

                // Face player
                if dir.length_squared() > 0.01 {
                    transform.rotation = Quat::from_rotation_y(dir.x.atan2(dir.z));
                }
                desired_anim = Some(animations.run_index);
            }
            AiState::Attack => {
                // Face player during attack
                let dir = Vec3::new(direction_to_player.x, 0.0, direction_to_player.z).normalize_or_zero();
                if dir.length_squared() > 0.01 {
                    transform.rotation = Quat::from_rotation_y(dir.x.atan2(dir.z));
                }
                movement.y = GRAVITY * time.delta_secs();
                desired_anim = Some(animations.attack_index);
            }
        }

        controller.translation = Some(movement);

        // Update animation
        if let Some(anim_entity) = std::iter::once(enemy_entity)
            .chain(children.iter_descendants(enemy_entity))
            .find(|e| anim_query.get(*e).is_ok())
        {
            if let Ok((mut anim_player, mut current_anim)) = anim_query.get_mut(anim_entity) {
                // For attack animation, play once; for others, loop
                if ai.state == AiState::Attack {
                    if current_anim.0 != desired_anim {
                        anim_player.stop_all();
                        anim_player.play(animations.attack_index);
                        current_anim.0 = desired_anim;
                    }
                } else if current_anim.0 != desired_anim {
                    anim_player.stop_all();
                    if let Some(anim_index) = desired_anim {
                        anim_player.play(anim_index).repeat();
                    }
                    current_anim.0 = desired_anim;
                }
            }
        }
    }
}

/// Handles combat: blocking, hit detection, damage, and death
fn combat_system(
    time: Res<Time>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    animations: Option<Res<GameAnimations>>,
    mut player_query: Query<
        (Entity, &Transform, &mut Health, &mut CombatState),
        (With<Player>, Without<Enemy>),
    >,
    mut enemy_query: Query<
        (Entity, &Transform, &mut Health, &mut CombatState, &EnemyAi),
        (With<Enemy>, Without<Player>),
    >,
    children: Query<&Children>,
    mut anim_query: Query<(&mut AnimationPlayer, &mut CurrentAnimation)>,
    mut hit_events: EventWriter<HitEvent>,
) {
    let Some(animations) = animations else {
        return;
    };

    // Get player info
    let Ok((player_entity, player_transform, mut player_health, mut player_combat)) =
        player_query.get_single_mut()
    else {
        return;
    };

    // Handle player blocking
    player_combat.is_blocking = mouse_button.pressed(MouseButton::Right) && !player_combat.is_dead;

    // Update hit timer
    if player_combat.is_hit {
        player_combat.hit_timer -= time.delta_secs();
        if player_combat.hit_timer <= 0.0 {
            player_combat.is_hit = false;
        }
    }

    let player_pos = player_transform.translation;

    // Check enemy attacks hitting player
    for (enemy_entity, enemy_transform, mut enemy_health, mut enemy_combat, enemy_ai) in
        enemy_query.iter_mut()
    {
        let enemy_pos = enemy_transform.translation;
        let distance = player_pos.distance(enemy_pos);

        // Update enemy hit timer
        if enemy_combat.is_hit {
            enemy_combat.hit_timer -= time.delta_secs();
            if enemy_combat.hit_timer <= 0.0 {
                enemy_combat.is_hit = false;
            }
        }

        // Enemy attack hits player
        if enemy_ai.state == AiState::Attack
            && distance < ENEMY_ATTACK_RANGE
            && !player_combat.is_hit
            && !player_combat.is_dead
        {
            // Calculate impact position between enemy and player
            let impact_pos = player_pos.lerp(enemy_pos, 0.3) + Vec3::Y * 1.0;

            if player_combat.is_blocking {
                // Blocked - play block animation, no damage
                info!("Player blocked attack!");
                hit_events.send(HitEvent {
                    position: impact_pos,
                    blocked: true,
                });
                if let Some(anim_entity) = find_animation_entity(player_entity, &children, &anim_query)
                {
                    if let Ok((mut anim_player, mut current_anim)) = anim_query.get_mut(anim_entity) {
                        if current_anim.0 != Some(animations.block_index) {
                            anim_player.stop_all();
                            anim_player.play(animations.block_index);
                            current_anim.0 = Some(animations.block_index);
                        }
                    }
                }
            } else {
                // Hit - take damage
                hit_events.send(HitEvent {
                    position: impact_pos,
                    blocked: false,
                });
                player_health.current -= 20.0;
                info!("Player hit! Health: {}/{}", player_health.current, player_health.max);
                player_combat.is_hit = true;
                player_combat.hit_timer = 0.5;

                if player_health.current <= 0.0 {
                    player_health.current = 0.0;
                    player_combat.is_dead = true;
                    info!("Player died!");
                    // Play death animation
                    if let Some(anim_entity) =
                        find_animation_entity(player_entity, &children, &anim_query)
                    {
                        if let Ok((mut anim_player, mut current_anim)) = anim_query.get_mut(anim_entity)
                        {
                            anim_player.stop_all();
                            anim_player.play(animations.death_index);
                            current_anim.0 = Some(animations.death_index);
                        }
                    }
                } else {
                    // Play hit animation
                    if let Some(anim_entity) =
                        find_animation_entity(player_entity, &children, &anim_query)
                    {
                        if let Ok((mut anim_player, mut current_anim)) = anim_query.get_mut(anim_entity)
                        {
                            anim_player.stop_all();
                            anim_player.play(animations.hit_index);
                            current_anim.0 = Some(animations.hit_index);
                        }
                    }
                }
            }
        }

        // Check if player attack hits this enemy
        // We check by looking at current animation of player
        if let Some(player_anim_entity) = find_animation_entity(player_entity, &children, &anim_query)
        {
            if let Ok((anim_player, current_anim)) = anim_query.get(player_anim_entity) {
                let is_player_attacking = current_anim.0 == Some(animations.attack_index)
                    && !anim_player.all_finished();

                if is_player_attacking
                    && distance < ENEMY_ATTACK_RANGE * 1.5
                    && !enemy_combat.is_hit
                    && !enemy_combat.is_dead
                {
                    // Player hits enemy - spawn sparks at enemy position
                    let impact_pos = enemy_pos.lerp(player_pos, 0.3) + Vec3::Y * 1.0;
                    hit_events.send(HitEvent {
                        position: impact_pos,
                        blocked: false,
                    });
                    enemy_health.current -= 25.0;
                    info!("Enemy hit! Health: {}/{}", enemy_health.current, enemy_health.max);
                    enemy_combat.is_hit = true;
                    enemy_combat.hit_timer = 0.5;

                    if enemy_health.current <= 0.0 {
                        enemy_health.current = 0.0;
                        enemy_combat.is_dead = true;
                        enemy_combat.death_timer = DEATH_DESPAWN_TIME;
                        info!("Enemy died!");
                        // Play death animation
                        if let Some(anim_entity) =
                            find_animation_entity(enemy_entity, &children, &anim_query)
                        {
                            if let Ok((mut anim_player, mut current_anim)) =
                                anim_query.get_mut(anim_entity)
                            {
                                anim_player.stop_all();
                                anim_player.play(animations.death_index);
                                current_anim.0 = Some(animations.death_index);
                            }
                        }
                    } else {
                        // Play hit animation
                        if let Some(anim_entity) =
                            find_animation_entity(enemy_entity, &children, &anim_query)
                        {
                            if let Ok((mut anim_player, mut current_anim)) =
                                anim_query.get_mut(anim_entity)
                            {
                                anim_player.stop_all();
                                anim_player.play(animations.hit_index);
                                current_anim.0 = Some(animations.hit_index);
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Helper to find the AnimationPlayer entity for a character
fn find_animation_entity(
    character: Entity,
    children: &Query<&Children>,
    anim_query: &Query<(&mut AnimationPlayer, &mut CurrentAnimation)>,
) -> Option<Entity> {
    std::iter::once(character)
        .chain(children.iter_descendants(character))
        .find(|e| anim_query.get(*e).is_ok())
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

/// Spawns spark particles when hits land
fn spawn_impact_sparks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut hit_events: EventReader<HitEvent>,
) {
    for event in hit_events.read() {
        let spark_count = if event.blocked { 8 } else { 12 };
        let color = if event.blocked {
            Color::srgb(0.7, 0.7, 1.0) // Blue-ish for blocked
        } else {
            Color::srgb(1.0, 0.6, 0.2) // Orange for hits
        };

        // Spawn multiple spark particles
        for _ in 0..spark_count {
            // Random direction for spark
            let angle = rand::random::<f32>() * std::f32::consts::TAU;
            let elevation = rand::random::<f32>() * 0.5 + 0.3;
            let speed = rand::random::<f32>() * 3.0 + 2.0;

            let velocity = Vec3::new(
                angle.cos() * speed * (1.0 - elevation),
                elevation * speed,
                angle.sin() * speed * (1.0 - elevation),
            );

            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.05, 0.05, 0.05))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: color,
                    emissive: color.into(),
                    ..default()
                })),
                Transform::from_translation(event.position),
                SparkParticle {
                    lifetime: 0.5,
                    velocity,
                },
            ));
        }
    }
}

/// Updates and despawns spark particles
fn update_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut particles: Query<(Entity, &mut Transform, &mut SparkParticle)>,
) {
    let dt = time.delta_secs();
    let gravity = Vec3::new(0.0, -15.0, 0.0);

    for (entity, mut transform, mut particle) in particles.iter_mut() {
        particle.lifetime -= dt;

        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        } else {
            // Update position with velocity and gravity
            particle.velocity += gravity * dt;
            transform.translation += particle.velocity * dt;

            // Scale down as lifetime decreases
            let scale = particle.lifetime * 2.0;
            transform.scale = Vec3::splat(scale.min(1.0));
        }
    }
}

/// Creates the player health bar UI at the top of the screen
fn setup_player_health_ui(mut commands: Commands) {
    // Root container at top-left
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            left: Val::Px(20.0),
            top: Val::Px(20.0),
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|parent| {
            // Health label
            parent.spawn((
                Text::new("Health"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Health bar background
            parent
                .spawn((
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(20.0),
                        margin: UiRect::top(Val::Px(5.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                ))
                .with_children(|bar_bg| {
                    // Health bar fill
                    bar_bg.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.8, 0.2, 0.2)),
                        PlayerHealthBar,
                    ));
                });
        });
}

/// Updates the player health bar width based on current health
fn update_player_health_bar(
    player_query: Query<&Health, With<Player>>,
    mut health_bar_query: Query<&mut Node, With<PlayerHealthBar>>,
) {
    let Ok(health) = player_query.get_single() else {
        return;
    };
    let Ok(mut bar_node) = health_bar_query.get_single_mut() else {
        return;
    };

    let health_percent = (health.current / health.max * 100.0).max(0.0);
    bar_node.width = Val::Percent(health_percent);
}

/// Spawns floating health bars above enemies that don't have one yet
fn spawn_enemy_health_bars(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    enemies: Query<Entity, (With<Enemy>, Without<EnemyHealthBar>)>,
    health_bars: Query<&EnemyHealthBar>,
) {
    for enemy_entity in enemies.iter() {
        // Check if this enemy already has a health bar
        let has_bar = health_bars.iter().any(|bar| bar.enemy == enemy_entity);
        if has_bar {
            continue;
        }

        // Create health bar background (dark)
        let bg_mesh = meshes.add(Cuboid::new(1.0, 0.1, 0.05));
        let bg_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.2, 0.2),
            unlit: true,
            ..default()
        });

        // Create health bar fill (red)
        let fill_mesh = meshes.add(Cuboid::new(1.0, 0.1, 0.05));
        let fill_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.2),
            unlit: true,
            ..default()
        });

        // Spawn background
        commands.spawn((
            Mesh3d(bg_mesh),
            MeshMaterial3d(bg_material),
            Transform::from_xyz(0.0, 2.5, 0.0),
            EnemyHealthBar { enemy: enemy_entity },
        ));

        // Spawn fill (slightly in front of background)
        commands.spawn((
            Mesh3d(fill_mesh),
            MeshMaterial3d(fill_material),
            Transform::from_xyz(0.0, 2.5, 0.03),
            EnemyHealthBarFill { enemy: enemy_entity },
        ));
    }
}

/// Marker for the fill portion of enemy health bar
#[derive(Component)]
struct EnemyHealthBarFill {
    enemy: Entity,
}

/// Updates enemy health bar positions and scales
fn update_enemy_health_bars(
    mut bar_query: Query<(&mut Transform, &EnemyHealthBar), Without<EnemyHealthBarFill>>,
    mut fill_query: Query<(&mut Transform, &EnemyHealthBarFill), Without<EnemyHealthBar>>,
    enemy_query: Query<(&Transform, &Health), (With<Enemy>, Without<EnemyHealthBar>, Without<EnemyHealthBarFill>)>,
    camera_query: Query<&Transform, (With<FollowCamera>, Without<Enemy>, Without<EnemyHealthBar>, Without<EnemyHealthBarFill>)>,
) {
    let Ok(camera_transform) = camera_query.get_single() else {
        return;
    };

    // Update background bars
    for (mut bar_transform, health_bar) in bar_query.iter_mut() {
        if let Ok((enemy_transform, _health)) = enemy_query.get(health_bar.enemy) {
            // Position above enemy
            bar_transform.translation = enemy_transform.translation + Vec3::Y * 2.5;
            // Face camera (billboard)
            bar_transform.look_at(camera_transform.translation, Vec3::Y);
        }
    }

    // Update fill bars
    for (mut fill_transform, health_bar_fill) in fill_query.iter_mut() {
        if let Ok((enemy_transform, health)) = enemy_query.get(health_bar_fill.enemy) {
            let health_percent = health.current / health.max;
            // Position above enemy, slightly in front of background
            fill_transform.translation = enemy_transform.translation + Vec3::Y * 2.5;
            // Face camera
            fill_transform.look_at(camera_transform.translation, Vec3::Y);
            // Scale based on health (scale X axis)
            fill_transform.scale.x = health_percent.max(0.0);
            // Offset to align left edge
            let local_forward = fill_transform.forward();
            let local_right = local_forward.cross(Vec3::Y).normalize();
            fill_transform.translation -= local_right * (1.0 - health_percent) * 0.5;
        }
    }
}

/// Despawns enemies after their death animation finishes
fn despawn_dead_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut enemy_query: Query<(Entity, &mut CombatState), With<Enemy>>,
    health_bars: Query<(Entity, &EnemyHealthBar)>,
    health_bar_fills: Query<(Entity, &EnemyHealthBarFill)>,
) {
    for (enemy_entity, mut combat_state) in enemy_query.iter_mut() {
        if combat_state.is_dead {
            combat_state.death_timer -= time.delta_secs();

            if combat_state.death_timer <= 0.0 {
                info!("Despawning enemy");
                // Despawn the enemy
                commands.entity(enemy_entity).despawn_recursive();

                // Despawn associated health bars
                for (bar_entity, bar) in health_bars.iter() {
                    if bar.enemy == enemy_entity {
                        commands.entity(bar_entity).despawn();
                    }
                }
                for (fill_entity, fill) in health_bar_fills.iter() {
                    if fill.enemy == enemy_entity {
                        commands.entity(fill_entity).despawn();
                    }
                }
            }
        }
    }
}
