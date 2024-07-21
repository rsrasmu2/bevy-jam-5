use avian2d::math::PI;
use bevy::prelude::*;
use iyes_progress::prelude::*;

use crate::game::{
    beam::SpawnBeam,
    map::{
        buildings::building::SpawnBuilding,
        tiles::{SpawnTileMap, TileMap, TILE_SIZE},
    },
    player::player_spawner::SpawnPlayerSpawner,
    state::GameState,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Setup), enter_setup);
    app.add_systems(OnExit(GameState::Setup), exit_setup);
    app.add_systems(
        Update,
        track_progress
            .track_progress()
            .run_if(in_state(GameState::Setup).and_then(resource_exists::<TileMap>)),
    );
}

fn enter_setup(mut commands: Commands, mut camera_query: Query<&mut Transform, With<Camera>>) {
    commands.trigger(SpawnTileMap {
        size: UVec2::new(25, 25),
    });

    commands.trigger(SpawnPlayerSpawner {
        position: UVec2::new(12, 12),
    });

    commands.trigger(SpawnBuilding {
        position: UVec2::new(10, 12),
    });
    commands.trigger(SpawnBeam {
        position: Vec2::new(10., 12.) * TILE_SIZE,
        rotation: 0.,
    });

    let Ok(mut camera_transform) = camera_query.get_single_mut() else {
        error!("No camera found");
        return;
    };

    camera_transform.translation = Vec2::splat(12. * TILE_SIZE).extend(0.);
}

fn exit_setup() {}

fn track_progress() -> Progress {
    true.into()
}
