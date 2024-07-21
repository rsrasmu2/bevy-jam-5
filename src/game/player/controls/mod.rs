use bevy::prelude::*;

pub mod player_action;
mod player_movement;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(player_action::plugin);
    app.add_plugins(player_movement::plugin);
}
