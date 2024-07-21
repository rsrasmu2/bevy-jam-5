//! Game mechanics and content.

use bevy::prelude::*;

//mod animation;
pub mod assets;
pub mod audio;
pub mod beam;
pub mod map;
pub mod movement;
mod physics;
pub mod player;
pub mod state;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        //animation::plugin,
        state::plugin,
        audio::plugin,
        assets::plugin,
        player::plugin,
        map::plugin,
        physics::plugin,
        beam::plugin,
    ));
}
