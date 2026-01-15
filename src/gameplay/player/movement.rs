use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::components::{Player, VerticalVelocity, GRAVITY, JUMP_VELOCITY, RUN_SPEED, WALK_SPEED};
use super::animation::GameAnimations;
use super::dodge::Dodging;
use super::CurrentAnimation;
use crate::core::camera::PlayerYaw;
use crate::gameplay::combat::CombatStatus;

pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    player_yaw: Res<PlayerYaw>,
    animations: Option<Res<GameAnimations>>,
    mut player_query: Query<
        (
            &Transform,
            &mut KinematicCharacterController,
            &mut VerticalVelocity,
            Option<&KinematicCharacterControllerOutput>,
            &CombatStatus,
            Option<&Dodging>,
        ),
        With<Player>,
    >,
    children: Query<&Children>,
    player_entity_query: Query<Entity, With<Player>>,
    mut anim_query: Query<(&mut AnimationPlayer, &mut CurrentAnimation)>,
) {
    let Ok((_transform, mut controller, mut vertical_velocity, controller_output, combat_state, maybe_dodging)) =
        player_query.get_single_mut()
    else {
        return;
    };

    if combat_state.is_dead {
        return;
    }

    // Skip movement while dodging (dodge system handles movement)
    if maybe_dodging.is_some() {
        return;
    }

    let Some(animations) = animations else {
        return;
    };
    let Ok(player_entity) = player_entity_query.get_single() else {
        return;
    };

    let Some(anim_entity) = std::iter::once(player_entity)
        .chain(children.iter_descendants(player_entity))
        .find(|e| anim_query.get(*e).is_ok())
    else {
        return;
    };

    let Ok((mut anim_player, mut current_anim)) = anim_query.get_mut(anim_entity) else {
        return;
    };

    let grounded = controller_output.map(|o| o.grounded).unwrap_or(false);

    let is_attacking =
        current_anim.0 == Some(animations.attack_index) && !anim_player.all_finished();

    let is_jumping = current_anim.0 == Some(animations.jump_index) && !anim_player.all_finished();

    if mouse_button.just_pressed(MouseButton::Left) && !is_attacking && !is_jumping && grounded {
        anim_player.stop_all();
        anim_player.play(animations.attack_index);
        current_anim.0 = Some(animations.attack_index);
        return;
    }

    if keyboard.just_pressed(KeyCode::Space) && grounded && !is_attacking && !is_jumping {
        vertical_velocity.0 = JUMP_VELOCITY;
        anim_player.stop_all();
        anim_player.play(animations.jump_index);
        current_anim.0 = Some(animations.jump_index);
    }

    if is_attacking {
        return;
    }

    let yaw = player_yaw.0;
    let forward = Vec3::new(-yaw.sin(), 0.0, -yaw.cos());
    let right = Vec3::new(forward.z, 0.0, -forward.x);

    let mut direction = Vec3::ZERO;
    if keyboard.pressed(KeyCode::KeyW) {
        direction += forward;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction -= forward;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction += right;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction -= right;
    }

    let is_moving = direction != Vec3::ZERO;
    let is_running = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
    let speed = if is_running { RUN_SPEED } else { WALK_SPEED };

    let mut movement = Vec3::ZERO;
    if is_moving {
        direction = direction.normalize();
        movement = direction * speed * time.delta_secs();
    }

    if grounded && vertical_velocity.0 <= 0.0 {
        vertical_velocity.0 = 0.0;
    } else {
        vertical_velocity.0 += GRAVITY * time.delta_secs();
    }
    movement.y = vertical_velocity.0 * time.delta_secs();

    controller.translation = Some(movement);

    if grounded && !is_jumping {
        let desired_anim = if is_moving {
            Some(if is_running {
                animations.run_index
            } else {
                animations.walk_index
            })
        } else {
            None
        };

        if current_anim.0 != desired_anim {
            anim_player.stop_all();
            if let Some(anim_index) = desired_anim {
                anim_player.play(anim_index).repeat();
            }
            current_anim.0 = desired_anim;
        }
    }
}
