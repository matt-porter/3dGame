use bevy::prelude::*;

mod components;
mod hit;
mod stamina;
mod system;

pub use components::*;
pub use hit::*;
pub use stamina::*;
pub use system::*;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HitEvent>()
            .add_systems(Update, (combat_system, recover_stamina));
    }
}
