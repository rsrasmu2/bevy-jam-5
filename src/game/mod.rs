//! Game mechanics and content.

use bevy::prelude::*;

//mod animation;
pub mod assets;
pub mod audio;
pub mod controls;
pub mod map;
pub mod movement;
pub mod spawn;
pub mod state;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        //animation::plugin,
        state::plugin,
        audio::plugin,
        assets::plugin,
        controls::plugin,
        spawn::plugin,
        map::plugin,
    ));
}
