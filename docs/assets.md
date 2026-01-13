# Asset Guide

## Overview

Assets are loaded from the `assets/` directory at runtime. Bevy's asset system handles loading and caching automatically.

## 3D Models

### Supported Formats
- **glTF 2.0** (`.gltf`, `.glb`) - Recommended format

### Creating Models
1. Create your model in Blender (or other 3D software)
2. Export as glTF 2.0:
   - File → Export → glTF 2.0
   - Use `.glb` for single-file exports (embedded textures)
   - Use `.gltf` for separate files (external textures)

### Loading Models in Code
```rust
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SceneRoot(asset_server.load("models/my_model.glb")));
}
```

## Textures

### Supported Formats
- **PNG** - Recommended (supports transparency)
- **JPEG** - Good for photos/backgrounds without transparency

### Texture Guidelines
- Use power-of-two dimensions when possible (512x512, 1024x1024, etc.)
- Keep textures as small as practical for performance
- Use PNG for UI elements and sprites with transparency

### Loading Textures
```rust
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("textures/my_texture.png");
}
```

## Audio

### Supported Formats
- **OGG Vorbis** (`.ogg`) - Recommended for music (compressed)
- **WAV** - Good for short sound effects (uncompressed)

### Audio Guidelines
- Use OGG for background music and long audio
- Use WAV for short sound effects that need low latency
- Sample rate: 44100 Hz is standard

### Loading Audio
```rust
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(AudioPlayer::new(asset_server.load("sounds/music.ogg")));
}
```

## Fonts

### Supported Formats
- **TrueType** (`.ttf`)
- **OpenType** (`.otf`)

### Loading Fonts
```rust
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/my_font.ttf");
}
```

## Shaders

### Format
- **WGSL** (`.wgsl`) - WebGPU Shading Language

Bevy uses WGSL for custom shaders. Place shader files in `assets/shaders/`.

## Knight Model Animations

The `Knight.glb` model includes the following animations:

### Movement
- `Walking_A`, `Walking_B`, `Walking_C` - Walking variations
- `Walking_Backwards` - Walking backward
- `Running_A`, `Running_B` - Running variations
- `Running_Strafe_Left`, `Running_Strafe_Right` - Strafing while running
- `Idle` - Standing idle
- `Jump_Start`, `Jump_Idle`, `Jump_Land` - Jump phases
- `Jump_Full_Short`, `Jump_Full_Long` - Complete jump animations

### Combat - Melee
- `1H_Melee_Attack_Chop`, `1H_Melee_Attack_Stab`, `1H_Melee_Attack_Slice_Diagonal`, `1H_Melee_Attack_Slice_Horizontal` - One-handed attacks
- `2H_Melee_Attack_Chop`, `2H_Melee_Attack_Slice`, `2H_Melee_Attack_Stab`, `2H_Melee_Attack_Spin`, `2H_Melee_Attack_Spinning` - Two-handed attacks
- `Dualwield_Melee_Attack_Chop`, `Dualwield_Melee_Attack_Slice`, `Dualwield_Melee_Attack_Stab` - Dual wield attacks
- `Unarmed_Melee_Attack_Punch_A`, `Unarmed_Melee_Attack_Punch_B`, `Unarmed_Melee_Attack_Kick` - Unarmed attacks

### Combat - Ranged
- `1H_Ranged_Aiming`, `1H_Ranged_Shoot`, `1H_Ranged_Shooting`, `1H_Ranged_Reload` - One-handed ranged
- `2H_Ranged_Aiming`, `2H_Ranged_Shoot`, `2H_Ranged_Shooting`, `2H_Ranged_Reload` - Two-handed ranged
- `Throw` - Throwing attack

### Combat - Defense
- `Block`, `Blocking`, `Block_Attack`, `Block_Hit` - Blocking
- `Dodge_Forward`, `Dodge_Backward`, `Dodge_Left`, `Dodge_Right` - Dodging
- `Hit_A`, `Hit_B` - Taking damage

### Magic
- `Spellcasting`, `Spellcast_Long`, `Spellcast_Raise`, `Spellcast_Shoot` - Spellcasting

### Actions
- `Interact`, `PickUp`, `Use_Item`, `Cheer` - Interactions
- `Sit_Chair_Down`, `Sit_Chair_Idle`, `Sit_Chair_Pose`, `Sit_Chair_StandUp` - Chair sitting
- `Sit_Floor_Down`, `Sit_Floor_Idle`, `Sit_Floor_Pose`, `Sit_Floor_StandUp` - Floor sitting
- `Lie_Down`, `Lie_Idle`, `Lie_Pose`, `Lie_StandUp` - Lying down

### State
- `Death_A`, `Death_A_Pose`, `Death_B`, `Death_B_Pose` - Death animations
- `T-Pose` - Default T-pose
- `Unarmed_Idle`, `Unarmed_Pose` - Unarmed stances
- `2H_Melee_Idle` - Two-handed weapon idle

## Asset Organization Tips

1. Use descriptive names: `player_idle.png` not `sprite1.png`
2. Group related assets in subdirectories: `models/characters/`, `models/environment/`
3. Keep source files (`.blend`, `.psd`) in a separate `source_assets/` folder outside the project
