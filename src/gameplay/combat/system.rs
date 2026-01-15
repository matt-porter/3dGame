use bevy::prelude::*;

use super::components::{CombatStatus, DEATH_DESPAWN_TIME};
use super::hit::HitEvent;
use super::stamina::{Stamina, ATTACK_STAMINA_COST, BLOCK_STAMINA_DRAIN};
use crate::gameplay::ai::{AiState, Enemy, EnemyAi, ENEMY_ATTACK_RANGE};
use crate::gameplay::health::Health;
use crate::gameplay::player::{CurrentAnimation, GameAnimations, Player};

pub fn combat_system(
    time: Res<Time>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    animations: Option<Res<GameAnimations>>,
    mut player_query: Query<
        (Entity, &Transform, &mut Health, &mut CombatStatus, &mut Stamina),
        (With<Player>, Without<Enemy>),
    >,
    mut enemy_query: Query<
        (Entity, &Transform, &mut Health, &mut CombatStatus, &EnemyAi),
        (With<Enemy>, Without<Player>),
    >,
    children: Query<&Children>,
    mut anim_query: Query<(&mut AnimationPlayer, &mut CurrentAnimation)>,
    mut hit_events: EventWriter<HitEvent>,
) {
    let Some(animations) = animations else {
        return;
    };

    let Ok((player_entity, player_transform, mut player_health, mut player_combat, mut player_stamina)) =
        player_query.get_single_mut()
    else {
        return;
    };

    let wants_to_block = mouse_button.pressed(MouseButton::Right) && !player_combat.is_dead;
    
    if wants_to_block && player_stamina.current > 0.0 {
        player_combat.is_blocking = true;
        player_stamina.current = (player_stamina.current - BLOCK_STAMINA_DRAIN * time.delta_secs()).max(0.0);
        
        if player_stamina.current <= 0.0 {
            player_combat.is_blocking = false;
        }
    } else {
        player_combat.is_blocking = false;
    }

    if player_combat.is_hit {
        player_combat.hit_timer -= time.delta_secs();
        if player_combat.hit_timer <= 0.0 {
            player_combat.is_hit = false;
        }
    }

    let player_pos = player_transform.translation;

    for (enemy_entity, enemy_transform, mut enemy_health, mut enemy_combat, enemy_ai) in
        enemy_query.iter_mut()
    {
        let enemy_pos = enemy_transform.translation;
        let distance = player_pos.distance(enemy_pos);

        if enemy_combat.is_hit {
            enemy_combat.hit_timer -= time.delta_secs();
            if enemy_combat.hit_timer <= 0.0 {
                enemy_combat.is_hit = false;
            }
        }

        if enemy_ai.state == AiState::Attack
            && distance < ENEMY_ATTACK_RANGE
            && !player_combat.is_hit
            && !player_combat.is_dead
        {
            let impact_pos = player_pos.lerp(enemy_pos, 0.3) + Vec3::Y * 1.0;

            if player_combat.is_blocking {
                info!("Player blocked attack!");
                hit_events.send(HitEvent {
                    position: impact_pos,
                    blocked: true,
                });
                if let Some(anim_entity) = find_animation_entity(player_entity, &children, &anim_query)
                {
                    if let Ok((mut anim_player, mut current_anim)) = anim_query.get_mut(anim_entity) {
                        if current_anim.0 != Some(animations.block_index) {
                            anim_player.stop_all();
                            anim_player.play(animations.block_index);
                            current_anim.0 = Some(animations.block_index);
                        }
                    }
                }
            } else {
                hit_events.send(HitEvent {
                    position: impact_pos,
                    blocked: false,
                });
                player_health.current -= 20.0;
                info!("Player hit! Health: {}/{}", player_health.current, player_health.max);
                player_combat.is_hit = true;
                player_combat.hit_timer = 0.5;

                if player_health.current <= 0.0 {
                    player_health.current = 0.0;
                    player_combat.is_dead = true;
                    info!("Player died!");
                    if let Some(anim_entity) =
                        find_animation_entity(player_entity, &children, &anim_query)
                    {
                        if let Ok((mut anim_player, mut current_anim)) = anim_query.get_mut(anim_entity)
                        {
                            anim_player.stop_all();
                            anim_player.play(animations.death_index);
                            current_anim.0 = Some(animations.death_index);
                        }
                    }
                } else {
                    if let Some(anim_entity) =
                        find_animation_entity(player_entity, &children, &anim_query)
                    {
                        if let Ok((mut anim_player, mut current_anim)) = anim_query.get_mut(anim_entity)
                        {
                            anim_player.stop_all();
                            anim_player.play(animations.hit_index);
                            current_anim.0 = Some(animations.hit_index);
                        }
                    }
                }
            }
        }

        if let Some(player_anim_entity) = find_animation_entity(player_entity, &children, &anim_query)
        {
            if let Ok((anim_player, current_anim)) = anim_query.get(player_anim_entity) {
                let is_player_attacking = current_anim.0 == Some(animations.attack_index)
                    && !anim_player.all_finished();

                if is_player_attacking
                    && distance < ENEMY_ATTACK_RANGE * 1.5
                    && !enemy_combat.is_hit
                    && !enemy_combat.is_dead
                {
                    let impact_pos = enemy_pos.lerp(player_pos, 0.3) + Vec3::Y * 1.0;
                    hit_events.send(HitEvent {
                        position: impact_pos,
                        blocked: false,
                    });
                    enemy_health.current -= 25.0;
                    info!("Enemy hit! Health: {}/{}", enemy_health.current, enemy_health.max);
                    enemy_combat.is_hit = true;
                    enemy_combat.hit_timer = 0.5;

                    if enemy_health.current <= 0.0 {
                        enemy_health.current = 0.0;
                        enemy_combat.is_dead = true;
                        enemy_combat.death_timer = DEATH_DESPAWN_TIME;
                        info!("Enemy died!");
                        if let Some(anim_entity) =
                            find_animation_entity(enemy_entity, &children, &anim_query)
                        {
                            if let Ok((mut anim_player, mut current_anim)) =
                                anim_query.get_mut(anim_entity)
                            {
                                anim_player.stop_all();
                                anim_player.play(animations.death_index);
                                current_anim.0 = Some(animations.death_index);
                            }
                        }
                    } else {
                        if let Some(anim_entity) =
                            find_animation_entity(enemy_entity, &children, &anim_query)
                        {
                            if let Ok((mut anim_player, mut current_anim)) =
                                anim_query.get_mut(anim_entity)
                            {
                                anim_player.stop_all();
                                anim_player.play(animations.hit_index);
                                current_anim.0 = Some(animations.hit_index);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn find_animation_entity(
    character: Entity,
    children: &Query<&Children>,
    anim_query: &Query<(&mut AnimationPlayer, &mut CurrentAnimation)>,
) -> Option<Entity> {
    std::iter::once(character)
        .chain(children.iter_descendants(character))
        .find(|e| anim_query.get(*e).is_ok())
}
