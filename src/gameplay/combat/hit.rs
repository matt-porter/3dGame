use bevy::prelude::*;

#[derive(Event)]
pub struct HitEvent {
    pub position: Vec3,
    pub blocked: bool,
}
