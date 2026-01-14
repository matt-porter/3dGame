# Model Export Guide for Bevy

This guide explains how to export 3D models in GLB/GLTF format for use with Bevy.

## Supported Format

Bevy uses the GLTF 2.0 format. GLB is the binary version (single file), GLTF is the JSON version (may have separate files for textures). GLB is recommended for simplicity.

## Exporting from Blender

### Basic Export

1. Select the objects you want to export (or export all)
2. Go to **File > Export > glTF 2.0 (.glb/.gltf)**
3. Set the export settings:
   - **Format**: glTF Binary (.glb)
   - **Include**: Selected Objects (or Visible Objects)

### Export Settings

Under **Include**:
- Check **Custom Properties** if you have custom data
- Check **Cameras** and **Lights** if needed (usually not for game assets)

Under **Transform**:
- **+Y Up**: Enable this (Bevy uses Y-up coordinate system)

Under **Mesh**:
- **Apply Modifiers**: Enable to bake modifiers
- **UVs**: Enable for texture mapping
- **Normals**: Enable for lighting
- **Tangents**: Enable if using normal maps
- **Vertex Colors**: Enable if your model uses them

Under **Materials**:
- **Materials**: Export
- **Images**: Automatic (embeds textures in GLB)

### Textures

For textures to work correctly:
1. Use **Principled BSDF** shader in Blender
2. Connect textures to appropriate inputs:
   - Base Color texture -> Base Color
   - Normal map -> Normal (use Normal Map node)
   - Metallic/Roughness -> Metallic/Roughness
3. Ensure textures are packed into the .blend file or in the same folder

Bevy supports these texture types from GLTF:
- Base color (albedo)
- Metallic-roughness (combined in one texture, R=unused, G=roughness, B=metallic)
- Normal maps
- Occlusion maps
- Emissive maps

### Animations

For animations to export correctly:

1. **Action Setup**:
   - Each animation should be a separate Action in Blender
   - Name actions clearly (e.g., "Walking_A", "Attack_Slash", "Idle")
   - Push actions to NLA tracks if using multiple animations

2. **Export Settings** (under Animation):
   - **Animation mode**: Actions or NLA Tracks
   - **Shape Keys**: Enable if using blend shapes
   - **Skinning**: Enable for skeletal animations

3. **Armature Requirements**:
   - Armature should be parent of the mesh
   - Mesh needs Armature modifier with proper vertex groups
   - Each bone should have matching vertex group

4. **Accessing Animations in Bevy**:
   ```rust
   // Load the GLTF
   let gltf_handle: Handle<Gltf> = asset_server.load("models/character.glb");

   // Access named animations from the Gltf asset
   let gltf = gltfs.get(&gltf_handle).unwrap();
   let walk_clip = gltf.named_animations.get("Walking_A").unwrap();
   ```

### Common Issues

**Model is too small/large**:
- Apply scale in Blender (Ctrl+A > Scale) before export
- Or adjust Transform scale in export settings

**Textures not showing**:
- Ensure textures are embedded (Images: Automatic)
- Check UV mapping is correct
- Use Principled BSDF shader

**Animations not playing**:
- Check animation is in Actions list
- Ensure armature modifier is applied correctly
- Verify bone names match vertex groups

**Model is rotated wrong**:
- Enable +Y Up in export settings
- Apply rotation in Blender before export (Ctrl+A > Rotation)

## Exporting from Other Software

### Maya

1. Install the official Autodesk GLTF exporter plugin
2. File > Export All > GLTF
3. Enable "Embed textures" for GLB format

### 3ds Max

1. Use the Babylon.js exporter plugin
2. File > Export > Export to GLB

### Mixamo

1. Download character in FBX format
2. Import to Blender
3. Export as GLB following Blender instructions above

## File Organization

Place exported models in:
```
assets/
  models/
    character.glb
    enemy.glb
    environment/
      castle.glb
      props.glb
```

Load in Bevy:
```rust
asset_server.load("models/character.glb#Scene0")
```

## Validation

Use the [glTF Validator](https://github.khronos.org/glTF-Validator/) to check your exported files for issues before importing into Bevy.
