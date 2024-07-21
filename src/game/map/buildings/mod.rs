use bevy::prelude::*;

pub mod building;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(building::plugin);
}
