use bevy::{prelude::*, window::PrimaryWindow};

use crate::game::state::GameState;

use super::tiles::{HoveredTile, SetHoveredTile, TileMap, TILE_SIZE_HALF};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        set_hovered_tile.run_if(in_state(GameState::Playing)),
    );
}

fn set_hovered_tile(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
    tiles: Res<TileMap>,
    hovered_tile: Res<HoveredTile>,
) {
    let Ok((camera, camera_transform)) = camera_query.get_single() else {
        error!("No camera found");
        return;
    };

    let Ok(window) = window_query.get_single() else {
        error!("No window found");
        return;
    };

    let Some(mouse_position) = window.cursor_position() else {
        // Cursor is outside of the window
        return;
    };

    let Some(world_position) = camera.viewport_to_world_2d(camera_transform, mouse_position) else {
        error!("No world position found");
        return;
    };

    let hovered_tile_pos = tiles.world_to_tile(world_position + TILE_SIZE_HALF);
    if hovered_tile_pos != hovered_tile.position() {
        commands.trigger(SetHoveredTile {
            position: hovered_tile_pos,
        });
    }
}
