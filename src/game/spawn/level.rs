//! Spawn the main level by triggering other observers.

use bevy::prelude::*;
use iyes_progress::prelude::*;

use crate::game::{
    map::tiles::{SpawnTileMap, TileMap},
    state::GameState,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
    app.add_systems(
        Update,
        track_progress
            .track_progress()
            .run_if(in_state(GameState::Setup).and_then(resource_exists::<TileMap>)),
    );
}

#[derive(Event, Debug)]
pub struct SpawnLevel {
    pub size: UVec2,
}

fn spawn_level(trigger: Trigger<SpawnLevel>, mut commands: Commands) {
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    commands.trigger(SpawnTileMap {
        size: trigger.event().size,
    });
}

fn track_progress() -> Progress {
    true.into()
}
