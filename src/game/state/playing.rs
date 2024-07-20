use bevy::prelude::*;

use crate::game::{assets::SoundtrackKey, audio::soundtrack::PlaySoundtrack, state::GameState};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), enter_playing);
    app.add_systems(OnExit(GameState::Playing), exit_playing);
}

fn enter_playing(mut commands: Commands) {
    //commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Gameplay));
}

fn exit_playing() {}
