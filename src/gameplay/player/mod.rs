use bevy::prelude::*;

mod animation;
mod components;
mod dodge;
mod movement;

pub use animation::*;
pub use components::*;
pub use dodge::*;
pub use movement::*;

use crate::states::AppState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                load_animations,
                setup_character_animations,
                (handle_dodge, update_dodge, player_movement)
                    .run_if(in_state(AppState::Playing)),
            ),
        );
    }
}
