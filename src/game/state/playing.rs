use bevy::prelude::*;

use crate::game::{
    beam::{Beam, RotateBeam},
    player::player_spawner::SpawnPlayer,
};

use super::GameState;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), enter_playing);
    app.add_systems(OnExit(GameState::Playing), exit_playing);
}

fn enter_playing(mut commands: Commands, beam_query: Query<Entity, With<Beam>>) {
    commands.trigger(SpawnPlayer);

    for beam in beam_query.iter() {
        commands.trigger_targets(RotateBeam { direction: -1. }, beam);
    }
}

fn exit_playing() {}
