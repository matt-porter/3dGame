# Development Plan

## Completed

- [x] Basic 3D scene setup (ground plane, lighting, camera)
- [x] Load Knight character model
- [x] Load Castle model
- [x] Player movement with WASD controls
- [x] Player rotation to face movement direction
- [x] Walking animation (Walking_A) plays when moving
- [x] Sprint with Shift key (Running_B animation, faster speed)
- [x] Animation system using named animations from GLTF
- [x] Camera follows the player
- [x] Space bar attack with melee animation (1H_Melee_Attack_Chop)
- [x] Scale up castle (15x) so player can run around on top
- [x] Player spawns on top of castle
- [x] Floor collision prevents falling through

## In Progress

- [ ] Health system and combat
  - Player and enemy health tracking
  - Hit detection with rapier collision events
  - Blocking with right mouse button

## Completed Recently

- [x] Enemy spawning and basic AI
  - 3 enemy knights spawn on the castle
  - AI states: Idle, Patrol, Chase, Attack
  - Enemies detect player and chase/attack when in range
  - Idle animation added for standing still

- [x] Third-person camera that stays behind player
  - Camera rotates with player facing direction based on yaw
  - PlayerYaw resource tracks mouse-controlled rotation

- [x] Mouse-based controls overhaul
  - Mouse controls player/camera direction
  - A/D keys for strafe left/right
  - Left mouse button for attack
  - Space bar for jump with Jump_Full_Short animation

- [x] Add physics-based collision using bevy_rapier3d
  - Generate castle collider from mesh (trimesh via AsyncSceneCollider)
  - Add capsule collider + KinematicCharacterController to player
  - Removed manual floor height and boundary constants

## Planned

### Combat & Enemies
- [ ] Enemy spawning system
  - Spawn enemy knights that patrol the castle
  - Basic AI: idle, patrol, chase player, attack
  - Use same Knight.glb model with different color/material
- [ ] Health system
  - Player and enemy health bars (UI)
  - Hit detection using rapier collision events
  - Hit_A/Hit_B animations when taking damage
  - Death_A/Death_B animations on death
- [ ] Combat improvements
  - Block with right mouse button (Block/Blocking animations)
  - Dodge roll with Q key (Dodge_Forward/Left/Right animations)
  - Attack combos (chain multiple attack animations)
  - Lock-on targeting for enemies

### Player Abilities
- [ ] Idle animation when standing still
- [ ] Magic/ranged attacks
  - Spellcast animations with particle effects
  - Projectile spawning and physics
- [ ] Stamina system for attacks/dodge/sprint

### Visual & Audio
- [ ] Particle effects
  - Sword swing trails
  - Impact sparks on hit
  - Magic spell effects
- [ ] Sound effects
  - Footsteps, sword swings, impacts
  - Background ambient sounds
- [ ] Day/night cycle with dynamic lighting

### Camera & UI
- [ ] Camera collision to prevent clipping through walls
- [ ] Health/stamina bars HUD
- [ ] Simple main menu (Play, Quit)
- [ ] Pause menu with ESC key

### World
- [ ] Collectible items (health potions, coins)
- [ ] Multiple areas/levels to explore
- [ ] Interactive objects (doors, chests)
