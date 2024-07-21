use bevy::prelude::*;

pub mod buildings;
mod controls;
pub mod tiles;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((controls::plugin, tiles::plugin, buildings::plugin));
}
