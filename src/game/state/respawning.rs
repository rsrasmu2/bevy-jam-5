use bevy::prelude::*;

use super::GameState;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Respawning), enter_respawning);
    app.add_systems(OnExit(GameState::Respawning), exit_respawning);
    app.add_systems(Update, tick_timer.run_if(in_state(GameState::Respawning)));
}

#[derive(Resource, Default)]
struct PlayerRespawnTimer(Timer);

fn enter_respawning(mut commands: Commands) {
    const RESPAWN_DURATION: f32 = 1.5;

    commands.insert_resource(PlayerRespawnTimer(Timer::from_seconds(
        RESPAWN_DURATION,
        TimerMode::Once,
    )));
}

fn exit_respawning(mut commands: Commands) {
    commands.remove_resource::<PlayerRespawnTimer>();
}

fn tick_timer(
    mut timer: ResMut<PlayerRespawnTimer>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        next_state.set(GameState::Playing);
    }
}
