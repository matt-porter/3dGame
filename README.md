# 3D Game

A 3D game built with Rust and [Bevy](https://bevyengine.org/).

## Prerequisites

- [Rust](https://rustup.rs/) (latest stable)

## Getting Started

```bash
# Run the game
cargo run

# Run in release mode (better performance)
cargo run --release
```

The first build will take several minutes as Bevy compiles. Subsequent builds are much faster.

## Project Structure

```
3dGame/
├── src/
│   └── main.rs         # Application entry point
├── assets/
│   ├── models/         # 3D models (.gltf, .glb)
│   ├── textures/       # Textures (.png, .jpg)
│   ├── sounds/         # Audio files (.ogg, .wav)
│   ├── fonts/          # Font files (.ttf, .otf)
│   ├── shaders/        # Custom shaders (.wgsl)
│   └── scenes/         # Scene definitions
├── docs/               # Documentation
├── Cargo.toml          # Rust dependencies
└── README.md
```

## Asset Formats

| Type     | Formats          | Notes                                    |
|----------|------------------|------------------------------------------|
| Models   | `.gltf`, `.glb`  | Bevy's native 3D format, exported from Blender |
| Textures | `.png`, `.jpg`   | PNG recommended for transparency support |
| Sounds   | `.ogg`, `.wav`   | OGG for music, WAV for short effects     |
| Fonts    | `.ttf`, `.otf`   | TrueType or OpenType fonts               |
| Shaders  | `.wgsl`          | WebGPU Shading Language                  |

## Controls

*To be defined*

## License

*To be defined*
