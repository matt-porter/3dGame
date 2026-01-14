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

(none)

## Completed Recently

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

(none)
