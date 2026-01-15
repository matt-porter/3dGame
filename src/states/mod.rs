use bevy::prelude::*;

#[derive(States, Default, Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    #[default]
    Loading,
    Menu,
    Playing,
    Paused,
}

#[derive(SubStates, Default, Clone, Copy, Eq, PartialEq, Debug, Hash)]
#[source(AppState = AppState::Playing)]
pub enum CombatState {
    #[default]
    Idle,
    Attacking,
    Blocking,
    Dodging,
    Stunned,
}
