use bevy::prelude::*;

use crate::gameplay::combat::CombatStatus;

#[derive(Component)]
pub struct Stamina {
    pub current: f32,
    pub max: f32,
    pub recovery_rate: f32,
}

impl Default for Stamina {
    fn default() -> Self {
        Self {
            current: 100.0,
            max: 100.0,
            recovery_rate: 30.0,
        }
    }
}

pub const ATTACK_STAMINA_COST: f32 = 20.0;
pub const DODGE_STAMINA_COST: f32 = 25.0;
pub const BLOCK_STAMINA_DRAIN: f32 = 15.0;
pub const SPRINT_STAMINA_DRAIN: f32 = 10.0;

pub fn recover_stamina(
    time: Res<Time>,
    mut query: Query<(&mut Stamina, &CombatStatus)>,
) {
    for (mut stamina, combat_status) in query.iter_mut() {
        if !combat_status.is_blocking {
            stamina.current = (stamina.current + stamina.recovery_rate * time.delta_secs())
                .min(stamina.max);
        }
    }
}
