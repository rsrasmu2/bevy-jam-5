//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{dev_tools::states::log_transitions, prelude::*};

use crate::{game::state::GameState, screen::Screen};

pub(super) fn plugin(app: &mut App) {
    // Add bevy-inspector-egui
    // app.add_plugins(bevy_inspector_egui::WorldInspectorPlugin::new());

    // Print state transitions in dev builds
    app.add_systems(Update, log_transitions::<Screen>);
    app.add_systems(Update, log_transitions::<GameState>);
}
