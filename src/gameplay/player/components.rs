use bevy::prelude::*;

pub const WALK_SPEED: f32 = 1.0;
pub const RUN_SPEED: f32 = 2.0;
pub const GRAVITY: f32 = -4.0;
pub const JUMP_VELOCITY: f32 = 2.0;
pub const PLAYER_START: Vec3 = Vec3::new(0.0, 15.0, 0.0);

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct VerticalVelocity(pub f32);

#[derive(Component, Default)]
pub struct CurrentAnimation(pub Option<AnimationNodeIndex>);
