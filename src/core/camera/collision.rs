use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::{FollowCamera, PlayerYaw};
use crate::gameplay::player::Player;

pub const CAMERA_DISTANCE: f32 = 2.0;
pub const CAMERA_HEIGHT: f32 = 1.0;
pub const CAMERA_COLLISION_OFFSET: f32 = 0.1;
pub const CAMERA_SMOOTHING: f32 = 10.0;

pub fn camera_follow_with_collision(
    time: Res<Time>,
    player_yaw: Res<PlayerYaw>,
    rapier_context: Query<&RapierContext>,
    player_query: Query<&Transform, (With<Player>, Without<FollowCamera>)>,
    mut camera_query: Query<&mut Transform, With<FollowCamera>>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    let Ok(mut camera_transform) = camera_query.get_single_mut() else {
        return;
    };
    let Ok(rapier_context) = rapier_context.get_single() else {
        return;
    };

    let player_pos = player_transform.translation;
    let yaw = player_yaw.0;

    let offset = Vec3::new(
        yaw.sin() * CAMERA_DISTANCE,
        CAMERA_HEIGHT,
        yaw.cos() * CAMERA_DISTANCE,
    );
    let desired_pos = player_pos + offset;

    let ray_origin = player_pos + Vec3::Y * 0.3;
    let ray_dir = (desired_pos - ray_origin).normalize();
    let ray_length = (desired_pos - ray_origin).length();

    let mut final_pos = desired_pos;

    if let Some((_, toi)) = rapier_context.cast_ray(
        ray_origin,
        ray_dir,
        ray_length,
        true,
        QueryFilter::default().exclude_sensors(),
    ) {
        let collision_distance = toi - CAMERA_COLLISION_OFFSET;
        if collision_distance > 0.5 {
            final_pos = ray_origin + ray_dir * collision_distance;
        } else {
            final_pos = ray_origin + ray_dir * 0.5;
        }
    }

    let smoothing = CAMERA_SMOOTHING * time.delta_secs();
    camera_transform.translation = camera_transform
        .translation
        .lerp(final_pos, smoothing.min(1.0));

    camera_transform.look_at(player_pos + Vec3::Y * 0.2, Vec3::Y);
}
