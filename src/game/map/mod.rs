use bevy::prelude::*;

mod controls;
pub mod tiles;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((controls::plugin, tiles::plugin));
}
