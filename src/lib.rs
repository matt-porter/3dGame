pub mod core;
pub mod gameplay;
pub mod states;
pub mod visual;

pub use core::{CameraPlugin, InputPlugin};
pub use gameplay::{AIPlugin, CombatPlugin, HealthPlugin, PlayerPlugin};
pub use states::AppState;
pub use visual::{HealthBarPlugin, ParticlePlugin};

pub use gameplay::player::{KnightGltf, Player, PLAYER_START};
pub use gameplay::ai::{Enemy, EnemyAi};
pub use gameplay::health::Health;
pub use gameplay::combat::{CombatStatus, Stamina};
pub use core::camera::{FollowCamera, PlayerYaw};
