use bevy::prelude::*;

use crate::gameplay::ai::Enemy;
use crate::gameplay::combat::CombatStatus;
use crate::visual::health_bar::{EnemyHealthBar, EnemyHealthBarFill};

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            current: 100.0,
            max: 100.0,
        }
    }
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_dead_enemies);
    }
}

fn despawn_dead_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut enemy_query: Query<(Entity, &mut CombatStatus), With<Enemy>>,
    health_bars: Query<(Entity, &EnemyHealthBar)>,
    health_bar_fills: Query<(Entity, &EnemyHealthBarFill)>,
) {
    for (enemy_entity, mut combat_state) in enemy_query.iter_mut() {
        if combat_state.is_dead {
            combat_state.death_timer -= time.delta_secs();

            if combat_state.death_timer <= 0.0 {
                info!("Despawning enemy");
                commands.entity(enemy_entity).despawn_recursive();

                for (bar_entity, bar) in health_bars.iter() {
                    if bar.enemy == enemy_entity {
                        commands.entity(bar_entity).despawn();
                    }
                }
                for (fill_entity, fill) in health_bar_fills.iter() {
                    if fill.enemy == enemy_entity {
                        commands.entity(fill_entity).despawn();
                    }
                }
            }
        }
    }
}
