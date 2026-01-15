use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::{
    AiState, Enemy, EnemyAi, ENEMY_ATTACK_RANGE, ENEMY_CHASE_SPEED, ENEMY_DETECTION_RANGE,
    ENEMY_PATROL_RANGE, ENEMY_WALK_SPEED,
};
use crate::gameplay::combat::CombatStatus;
use crate::gameplay::player::{CurrentAnimation, GameAnimations, Player};

const GRAVITY: f32 = -20.0;

pub fn enemy_ai(
    time: Res<Time>,
    animations: Option<Res<GameAnimations>>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<
        (
            Entity,
            &mut Transform,
            &mut EnemyAi,
            &mut KinematicCharacterController,
            &CombatStatus,
        ),
        (With<Enemy>, Without<Player>),
    >,
    children: Query<&Children>,
    mut anim_query: Query<(&mut AnimationPlayer, &mut CurrentAnimation)>,
) {
    let Some(animations) = animations else {
        return;
    };

    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    let player_pos = player_transform.translation;

    for (enemy_entity, mut transform, mut ai, mut controller, combat_state) in enemy_query.iter_mut()
    {
        if combat_state.is_dead {
            continue;
        }
        let enemy_pos = transform.translation;
        let distance_to_player = enemy_pos.distance(player_pos);
        let direction_to_player = (player_pos - enemy_pos).normalize_or_zero();

        ai.state_timer += time.delta_secs();

        let new_state = match ai.state {
            AiState::Idle => {
                if distance_to_player < ENEMY_DETECTION_RANGE {
                    AiState::Chase
                } else if ai.state_timer > 3.0 {
                    ai.state_timer = 0.0;
                    AiState::Patrol
                } else {
                    AiState::Idle
                }
            }
            AiState::Patrol => {
                if distance_to_player < ENEMY_DETECTION_RANGE {
                    AiState::Chase
                } else if ai.patrol_target.is_none() || ai.state_timer > 5.0 {
                    ai.state_timer = 0.0;
                    AiState::Idle
                } else {
                    AiState::Patrol
                }
            }
            AiState::Chase => {
                if distance_to_player < ENEMY_ATTACK_RANGE {
                    ai.state_timer = 0.0;
                    AiState::Attack
                } else if distance_to_player > ENEMY_DETECTION_RANGE * 1.5 {
                    AiState::Idle
                } else {
                    AiState::Chase
                }
            }
            AiState::Attack => {
                if ai.state_timer > 1.0 {
                    ai.state_timer = 0.0;
                    if distance_to_player < ENEMY_ATTACK_RANGE {
                        AiState::Attack
                    } else {
                        AiState::Chase
                    }
                } else {
                    AiState::Attack
                }
            }
        };

        if new_state != ai.state {
            ai.state = new_state;
            if new_state == AiState::Patrol {
                let angle = ai.state_timer * 1000.0;
                ai.patrol_target = Some(
                    ai.home_position
                        + Vec3::new(
                            angle.cos() * ENEMY_PATROL_RANGE,
                            0.0,
                            angle.sin() * ENEMY_PATROL_RANGE,
                        ),
                );
            }
        }

        let mut movement = Vec3::ZERO;
        let desired_anim;

        match ai.state {
            AiState::Idle => {
                desired_anim = Some(animations.idle_index);
            }
            AiState::Patrol => {
                if let Some(target) = ai.patrol_target {
                    let dir = (target - enemy_pos).normalize_or_zero();
                    movement = dir * ENEMY_WALK_SPEED * time.delta_secs();
                    movement.y = GRAVITY * time.delta_secs();

                    if dir.length_squared() > 0.01 {
                        transform.rotation = Quat::from_rotation_y(dir.x.atan2(dir.z));
                    }
                }
                desired_anim = Some(animations.walk_index);
            }
            AiState::Chase => {
                let dir =
                    Vec3::new(direction_to_player.x, 0.0, direction_to_player.z).normalize_or_zero();
                movement = dir * ENEMY_CHASE_SPEED * time.delta_secs();
                movement.y = GRAVITY * time.delta_secs();

                if dir.length_squared() > 0.01 {
                    transform.rotation = Quat::from_rotation_y(dir.x.atan2(dir.z));
                }
                desired_anim = Some(animations.run_index);
            }
            AiState::Attack => {
                let dir =
                    Vec3::new(direction_to_player.x, 0.0, direction_to_player.z).normalize_or_zero();
                if dir.length_squared() > 0.01 {
                    transform.rotation = Quat::from_rotation_y(dir.x.atan2(dir.z));
                }
                movement.y = GRAVITY * time.delta_secs();
                desired_anim = Some(animations.attack_index);
            }
        }

        controller.translation = Some(movement);

        if let Some(anim_entity) = std::iter::once(enemy_entity)
            .chain(children.iter_descendants(enemy_entity))
            .find(|e| anim_query.get(*e).is_ok())
        {
            if let Ok((mut anim_player, mut current_anim)) = anim_query.get_mut(anim_entity) {
                if ai.state == AiState::Attack {
                    if current_anim.0 != desired_anim {
                        anim_player.stop_all();
                        anim_player.play(animations.attack_index);
                        current_anim.0 = desired_anim;
                    }
                } else if current_anim.0 != desired_anim {
                    anim_player.stop_all();
                    if let Some(anim_index) = desired_anim {
                        anim_player.play(anim_index).repeat();
                    }
                    current_anim.0 = desired_anim;
                }
            }
        }
    }
}
