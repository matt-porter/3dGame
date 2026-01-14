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

(Nothing currently in progress)

## Completed Recently

- [x] Visual effects and UI
  - Impact spark particles on hits (orange) and blocks (blue)
  - Player health bar HUD at top-left
  - Enemy health bars (floating billboards above enemies)
  - Debug logging for combat events
  - Enemies despawn 3 seconds after death

- [x] Health system and combat
  - Player and enemy health tracking
  - Hit detection (proximity-based during attacks)
  - Blocking with right mouse button
  - Hit_A animation on damage
  - Death_A animation when health depleted

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

### Combat Improvements
- [ ] Dodge roll with Q key (Dodge_Forward/Left/Right animations)
- [ ] Attack combos (chain multiple attack animations)
- [ ] Lock-on targeting for enemies

### Player Abilities
- [ ] Magic/ranged attacks
  - Spellcast animations with particle effects
  - Projectile spawning and physics
- [ ] Stamina system for attacks/dodge/sprint

### Visual & Audio
- [ ] Particle effects
  - Sword swing trails
  - Magic spell effects
- [ ] Sound effects
  - Footsteps, sword swings, impacts
  - Background ambient sounds
- [ ] Day/night cycle with dynamic lighting

### Camera & UI
- [ ] Camera collision to prevent clipping through walls
- [ ] Simple main menu (Play, Quit)
- [ ] Pause menu with ESC key

### World
- [ ] Collectible items (health potions, coins)
- [ ] Multiple areas/levels to explore
- [ ] Interactive objects (doors, chests)

### Bugs / Issues
- [ ] Player doesn't drop to floor properly - castle collision mesh may not be precise enough
