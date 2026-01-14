# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
cargo run              # Development build (faster compile, slower runtime)
cargo run --release    # Release build (slower compile, optimized runtime)
cargo check            # Check for errors without building
cargo test             # Run tests
```

First build takes several minutes due to Bevy compilation. Subsequent builds are much faster.

## Architecture

This is a 3D game built with Rust and Bevy 0.15 with bevy_rapier3d for physics. Currently a single-file architecture in `src/main.rs`.

### Core Systems

The game uses Bevy's ECS (Entity Component System) with these main systems:

1. **setup** - Spawns scene: ground plane, castle model, player (Knight), lighting, camera
2. **grab_cursor** - Locks cursor to window for mouse look
3. **load_animations** - Loads named animations from Knight.glb GLTF file once asset is ready
4. **setup_player_animation** - Attaches animation graph to player's AnimationPlayer
5. **mouse_look** - Updates player yaw based on mouse movement (stored in `PlayerYaw` resource)
6. **player_movement** - Handles WASD movement (W/S forward/back, A/D strafe), sprint, jump, attack
7. **camera_follow** - Third-person camera stays behind player based on yaw

### Key Components

- `Player` - Marks the player entity
- `FollowCamera` - Marks the camera that follows player
- `AnimationSetupDone` - Prevents re-attaching animation graph
- `CurrentAnimation` - Tracks currently playing animation index

### Resources

- `KnightGltf` - Handle to loaded Knight.glb asset
- `PlayerAnimations` - Animation graph and node indices for walk/run/attack

### Animation System

Animations are loaded by name from the GLTF file (see `docs/assets.md` for full list):
- `Walking_A` - Walk animation
- `Running_B` - Sprint animation
- `1H_Melee_Attack_Chop` - Attack animation

The animation graph is created in `load_animations` and attached to the AnimationPlayer entity (nested in the GLTF scene hierarchy) in `setup_player_animation`.

### Physics (bevy_rapier3d)

- Castle uses `AsyncSceneCollider` with `ComputedColliderShape::TriMesh` to auto-generate colliders from the mesh
- Player uses `KinematicCharacterController` with a capsule collider for movement and collision
- Gravity is applied manually in `player_movement` when not grounded
- Ground plane uses a halfspace collider

### Constants

Movement speeds, gravity, camera offset, and castle scale are defined as constants at the top of `main.rs`.

## Controls

- Mouse - Look/aim direction
- W/S - Move forward/backward
- A/D - Strafe left/right
- Shift - Sprint (hold)
- Space - Jump
- Left Mouse Button - Attack

## Assets

Assets load from `assets/` directory. Key models:
- `models/Knight.glb` - Player character with animations
- `models/castle.glb` - Castle environment

See `docs/assets.md` for supported formats and available Knight animations.
