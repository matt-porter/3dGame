use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::states::AppState;

pub const MOUSE_SENSITIVITY: f32 = 0.003;

#[derive(Resource, Default)]
pub struct PlayerInput {
    pub movement: Vec3,
    pub attacking: bool,
    pub blocking: bool,
    pub dodging: bool,
    pub jumping: bool,
    pub sprinting: bool,
    pub camera_delta: Vec2,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerInput>()
            .add_systems(Update, read_input.run_if(in_state(AppState::Playing)));
    }
}

fn read_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut input: ResMut<PlayerInput>,
) {
    let mut camera_delta = Vec2::ZERO;
    for event in mouse_motion.read() {
        camera_delta += event.delta;
    }
    input.camera_delta = camera_delta;

    let mut movement = Vec3::ZERO;
    if keyboard.pressed(KeyCode::KeyW) {
        movement.z -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        movement.z += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        movement.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        movement.x += 1.0;
    }
    if movement != Vec3::ZERO {
        movement = movement.normalize();
    }
    input.movement = movement;

    input.attacking = mouse_button.just_pressed(MouseButton::Left);
    input.blocking = mouse_button.pressed(MouseButton::Right);
    input.jumping = keyboard.just_pressed(KeyCode::Space);
    input.sprinting = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
    input.dodging = keyboard.just_pressed(KeyCode::KeyQ);
}
