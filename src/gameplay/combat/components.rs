use bevy::prelude::*;

pub const DEATH_DESPAWN_TIME: f32 = 3.0;

#[derive(Component, Default)]
pub struct CombatStatus {
    pub is_blocking: bool,
    pub is_hit: bool,
    pub is_dead: bool,
    pub hit_timer: f32,
    pub death_timer: f32,
}

#[derive(Component)]
pub struct AttackHitbox;
