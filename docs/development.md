# Development Guide

See also:
- [architecture.md](architecture.md) - Module structure and plugin design
- [plan.md](plan.md) - Development roadmap and task tracking
- [assets.md](assets.md) - Asset formats and Knight animations

## Build Commands

```bash
# Development build (faster compile, slower runtime)
cargo run

# Release build (slower compile, faster runtime)
cargo run --release

# Check for errors without building
cargo check

# Run tests
cargo test
```

## Performance Tips

### Faster Compilation

The `Cargo.toml` is already configured with:
- Optimized dependencies in dev mode (`opt-level = 3`)
- Link-time optimization for release builds

For even faster iteration, consider:

1. **Use `cargo watch`** for auto-rebuild:
   ```bash
   cargo install cargo-watch
   cargo watch -x run
   ```

2. **Enable Bevy's dynamic linking** (faster linking, dev only):
   ```toml
   # In Cargo.toml, add:
   [features]
   dev = ["bevy/dynamic_linking"]
   ```
   Then run with: `cargo run --features dev`

### Runtime Performance

- Use `cargo run --release` for performance testing
- Enable Bevy's frame diagnostics to monitor FPS:
  ```rust
  app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin);
  app.add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default());
  ```

## Project Architecture

### Recommended Module Structure

As the project grows, organize code into modules:

```
src/
├── main.rs           # App setup and plugin registration
├── lib.rs            # Optional: expose as library
├── player/
│   ├── mod.rs        # Player plugin
│   ├── movement.rs   # Movement systems
│   └── input.rs      # Input handling
├── world/
│   ├── mod.rs        # World plugin
│   └── terrain.rs    # Terrain generation
├── ui/
│   ├── mod.rs        # UI plugin
│   └── hud.rs        # HUD systems
└── components.rs     # Shared components
```

### Bevy Plugin Pattern

Organize features as plugins:

```rust
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
           .add_systems(Update, (move_player, handle_input));
    }
}

// In main.rs:
app.add_plugins(PlayerPlugin);
```

## Debugging

### Enable Bevy's Debug Tools

```rust
// Show entity inspector (requires bevy-inspector-egui crate)
// Show physics debug (if using rapier)
// Log system execution order
app.add_plugins(bevy::log::LogPlugin::default());
```

### Useful Debug Commands

```bash
# See expanded macros
cargo expand

# Check binary size
cargo bloat --release

# Profile compilation time
cargo build --timings
```

## Common Patterns

### State Management

```rust
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    Playing,
    Paused,
}

// In setup:
app.init_state::<GameState>()
   .add_systems(OnEnter(GameState::Playing), start_game)
   .add_systems(OnExit(GameState::Playing), cleanup_game);
```

### Resource Loading

```rust
#[derive(Resource)]
struct GameAssets {
    player_model: Handle<Scene>,
    jump_sound: Handle<AudioSource>,
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        player_model: asset_server.load("models/player.glb"),
        jump_sound: asset_server.load("sounds/jump.ogg"),
    });
}
```
