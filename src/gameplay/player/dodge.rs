use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::core::camera::PlayerYaw;
use crate::core::input::PlayerInput;
use crate::gameplay::combat::{Stamina, DODGE_STAMINA_COST};

use super::{Player, VerticalVelocity};

pub const DODGE_SPEED: f32 = 3.0;
pub const DODGE_DURATION: f32 = 0.4;

pub const DODGE_FORWARD_ANIMATION: &str = "Dodge_Forward";
pub const DODGE_LEFT_ANIMATION: &str = "Dodge_Left";
pub const DODGE_RIGHT_ANIMATION: &str = "Dodge_Right";
pub const DODGE_BACKWARD_ANIMATION: &str = "Dodge_Backward";

#[derive(Component)]
pub struct Dodging {
    pub direction: Vec3,
    pub timer: f32,
}

pub fn handle_dodge(
    mut commands: Commands,
    input: Res<PlayerInput>,
    player_yaw: Res<PlayerYaw>,
    mut query: Query<(Entity, &mut Stamina, Option<&Dodging>), With<Player>>,
) {
    for (entity, mut stamina, maybe_dodging) in query.iter_mut() {
        if maybe_dodging.is_some() {
            continue;
        }

        if input.dodging && stamina.current >= DODGE_STAMINA_COST {
            stamina.current -= DODGE_STAMINA_COST;

            let yaw = player_yaw.0;
            let forward = Vec3::new(-yaw.sin(), 0.0, -yaw.cos());
            let right = Vec3::new(yaw.cos(), 0.0, -yaw.sin());

            let direction = if input.movement.length_squared() > 0.1 {
                (forward * -input.movement.z + right * input.movement.x).normalize()
            } else {
                forward
            };

            commands.entity(entity).insert(Dodging {
                direction,
                timer: DODGE_DURATION,
            });
        }
    }
}

pub fn update_dodge(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<
        (
            Entity,
            &mut Dodging,
            &mut KinematicCharacterController,
            &mut VerticalVelocity,
        ),
        With<Player>,
    >,
) {
    for (entity, mut dodging, mut controller, mut vertical_velocity) in query.iter_mut() {
        dodging.timer -= time.delta_secs();

        if dodging.timer <= 0.0 {
            commands.entity(entity).remove::<Dodging>();
        } else {
            let mut movement = dodging.direction * DODGE_SPEED * time.delta_secs();
            vertical_velocity.0 += super::components::GRAVITY * time.delta_secs();
            movement.y = vertical_velocity.0 * time.delta_secs();
            controller.translation = Some(movement);
        }
    }
}
