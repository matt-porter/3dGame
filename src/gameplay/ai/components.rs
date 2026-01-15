use bevy::prelude::*;

pub const ENEMY_WALK_SPEED: f32 = 2.0;
pub const ENEMY_CHASE_SPEED: f32 = 4.0;
pub const ENEMY_DETECTION_RANGE: f32 = 8.0;
pub const ENEMY_ATTACK_RANGE: f32 = 2.0;
pub const ENEMY_PATROL_RANGE: f32 = 3.0;

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Default)]
pub struct EnemyAi {
    pub state: AiState,
    pub home_position: Vec3,
    pub patrol_target: Option<Vec3>,
    pub state_timer: f32,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum AiState {
    #[default]
    Idle,
    Patrol,
    Chase,
    Attack,
}
