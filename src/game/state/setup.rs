use bevy::prelude::*;

use crate::game::{
    spawn::{level::SpawnLevel, player::SpawnPlayer},
    state::GameState,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Setup), enter_setup);
    app.add_systems(OnExit(GameState::Setup), exit_setup);
}

fn enter_setup(mut commands: Commands) {
    commands.trigger(SpawnLevel {
        size: UVec2::new(25, 25),
    });
    commands.trigger(SpawnPlayer {
        position: UVec2::new(12, 12),
    });
}

fn exit_setup() {}
