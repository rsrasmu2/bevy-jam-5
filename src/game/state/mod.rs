use bevy::prelude::*;
use iyes_progress::prelude::*;

use crate::screen::Screen;

mod playing;
mod respawning;
mod setup;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((setup::plugin, playing::plugin, respawning::plugin));
    app.add_plugins(ProgressPlugin::new(GameState::Setup).continue_to(GameState::Playing));
    app.add_sub_state::<GameState>();
}

#[derive(SubStates, Debug, Clone, Copy, Eq, PartialEq, Hash, Default, Reflect)]
#[source(Screen = Screen::Playing)]
pub enum GameState {
    #[default]
    Setup,
    Playing,
    Respawning,
}
