use bevy::prelude::*;

pub const ENEMY_WALK_SPEED: f32 = 0.4;
pub const ENEMY_CHASE_SPEED: f32 = 0.8;
pub const ENEMY_DETECTION_RANGE: f32 = 1.6;
pub const ENEMY_ATTACK_RANGE: f32 = 0.4;
pub const ENEMY_PATROL_RANGE: f32 = 0.6;

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
