use bevy::prelude::*;

mod components;
mod state_machine;

pub use components::*;
pub use state_machine::*;

use crate::states::AppState;

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            enemy_ai.run_if(in_state(AppState::Playing)),
        );
    }
}
