use bevy::prelude::*;

mod collision;
mod follow;

pub use collision::*;
pub use follow::*;

use crate::core::input::PlayerInput;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerYaw>()
            .add_systems(Update, (mouse_look_system, camera_follow_with_collision));
    }
}

#[derive(Component)]
pub struct FollowCamera;

#[derive(Resource, Default)]
pub struct PlayerYaw(pub f32);

fn mouse_look_system(
    input: Res<PlayerInput>,
    yaw: ResMut<PlayerYaw>,
    player_query: Query<&mut Transform, With<crate::gameplay::player::Player>>,
) {
    mouse_look(yaw, player_query, input.camera_delta);
}
