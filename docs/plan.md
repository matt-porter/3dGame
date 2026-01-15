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

- [x] Improve attack responsiveness
  - Added attack input buffer (0.15s window)
  - Allow attacks while airborne (removed grounded requirement)

- [x] Scale down characters and adjust world proportions
  - Reduced castle scale from 15x to 2x
  - Added CHARACTER_SCALE constant (0.2) for player and enemies
  - Adjusted physics colliders proportionally
  - Scaled camera distance/height/offsets for smaller characters
  - Adjusted enemy AI ranges (detection, attack, patrol speeds)
  - Adjusted player movement speeds, gravity, jump velocity
  - Scaled enemy health bar positions and mesh sizes
  - Scaled combat impact effect positions


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

## Priority 1: Code Refactoring ✅ COMPLETED

Refactored from ~1200 lines single file to modular plugin architecture (~140 lines in main.rs):

### Modular Structure ✅
- [x] Created plugin-based architecture following Bevy 0.15 best practices
  - `src/lib.rs` - Plugin registry and exports
  - `src/states/mod.rs` - AppState (Loading/Menu/Playing/Paused), CombatState substate
  - `src/core/` - InputPlugin, CameraPlugin (with collision)
  - `src/gameplay/` - PlayerPlugin, CombatPlugin, AIPlugin, HealthPlugin
  - `src/visual/` - ParticlePlugin, HealthBarPlugin

### State Management ✅
- [x] AppState enum (Loading, Menu, Playing, Paused)
- [x] CombatState as SubState (Idle, Attacking, Blocking, Dodging, Stunned)
- [x] run_if(in_state(AppState::Playing)) on all gameplay systems

### System Organization ✅
- [x] Centralized input handling into PlayerInput resource (core/input.rs)

## Priority 2: Camera & Core Improvements ✅ COMPLETED

- [x] Camera collision to prevent clipping through walls
  - Rapier raycast from player head to desired camera position
  - Camera pulls closer when obstacles detected
  - Smooth interpolation to avoid jitter (CAMERA_SMOOTHING = 10.0)

## Priority 3: Combat Improvements

### Stamina System ✅ COMPLETED
- [x] Stamina component (current, max, recovery_rate)
- [x] Stamina recovery system (recovers when not blocking)
- [x] Stamina bar UI below health bar (gold color)
- [x] Stamina costs defined (attack: 20, dodge: 25, block drain: 15/s)
- [ ] Integrate stamina cost checks into combat (attacks, blocking)

### Dodge Roll ✅ COMPLETED
- [x] Q key triggers dodge roll
- [x] Direction based on movement input (or forward if none)
- [x] Dodge costs stamina (25)
- [x] Movement system skips while dodging
- [x] Dodge animation constants defined
- [ ] Play appropriate dodge animation (Forward/Left/Right/Backward)
- [ ] Brief invincibility frames during dodge

### Attack Combos
- [ ] Chain multiple attack animations on repeated input
- [ ] Timing window for combo continuation
- [ ] Different animations per combo hit (1H_Melee_Attack_Chop → Stab → Slice)

### Lock-on Targeting
- [ ] Tab key to lock onto nearest enemy
- [ ] Visual indicator on locked target
- [ ] Camera focuses on target
- [ ] Attacks auto-orient toward target

## Priority 4: UI & Menus

- [ ] Simple main menu (Play, Quit)
  - Menu state with UI buttons
  - Transition to Playing state on Play
- [ ] Pause menu with ESC key
  - PauseState substate
  - Resume, Quit to Menu options
  - Time scale = 0 while paused
- [ ] Stamina bar UI

## Priority 5: Visual & Audio

### Particle Effects
- [ ] Sword swing trails
- [ ] Magic spell effects (for future ranged attacks)

### Sound Effects
- [ ] Footsteps (walk/run variations)
- [ ] Sword swings and impacts
- [ ] Block sounds
- [ ] Background ambient sounds

### Lighting
- [ ] Day/night cycle with dynamic lighting (stretch goal)

## Priority 6: Player Abilities (Future)

- [ ] Magic/ranged attacks
  - Spellcast animations with particle effects
  - Projectile spawning and physics

## Priority 7: World (Future)

- [ ] Collectible items (health potions, coins)
- [ ] Multiple areas/levels to explore
- [ ] Interactive objects (doors, chests)

## Bugs / Issues

- [ ] Player doesn't drop to floor properly - castle collision mesh may not be precise enough

## Architecture Notes

### Recommended Module Structure
```
src/
├── main.rs              # Entry point (minimal)
├── lib.rs               # Plugin registry + state definitions
├── states/
│   ├── mod.rs           # AppState, GameState enums
│   ├── menu.rs          # Menu plugin
│   └── gameplay.rs      # Gameplay plugin
├── core/
│   ├── mod.rs
│   ├── input.rs         # Unified input handling
│   ├── camera/
│   │   ├── mod.rs
│   │   └── collision.rs # Camera collision
│   └── physics.rs       # Rapier wrapper
├── gameplay/
│   ├── mod.rs
│   ├── player/
│   │   ├── mod.rs
│   │   ├── movement.rs
│   │   └── animation.rs
│   ├── combat/
│   │   ├── mod.rs
│   │   ├── stamina.rs
│   │   └── hit.rs
│   ├── ai/
│   │   ├── mod.rs
│   │   └── state_machine.rs
│   └── health.rs
└── visual/
    ├── mod.rs
    ├── particles.rs
    └── health_bar.rs
```

### Key Bevy 0.15 Patterns
- Use `States` for main flow (Menu → Playing → Paused)
- Use `SubStates` for nested state (CombatState only while Playing)
- Use `OnEnter`/`OnExit` for state transition setup/cleanup
- Use `run_if(in_state(...))` for conditional systems
- Use `SystemSets` and `.chain()` for system ordering
