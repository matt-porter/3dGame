use bevy::prelude::*;

use super::PlayerYaw;
use crate::gameplay::player::Player;

pub fn mouse_look(
    mut yaw: ResMut<PlayerYaw>,
    mut player_query: Query<&mut Transform, With<Player>>,
    camera_delta: Vec2,
) {
    let delta_x = camera_delta.x;

    if delta_x != 0.0 {
        yaw.0 -= delta_x * crate::core::input::MOUSE_SENSITIVITY;

        if let Ok(mut transform) = player_query.get_single_mut() {
            transform.rotation = Quat::from_rotation_y(yaw.0 + std::f32::consts::PI);
        }
    }
}
