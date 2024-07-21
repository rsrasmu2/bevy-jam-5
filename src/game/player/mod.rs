use bevy::prelude::*;

use crate::game::state::GameState;

mod controls;
pub mod player_spawner;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((controls::plugin, player_spawner::plugin));
    app.observe(kill_player);
    app.register_type::<Player>();
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Event)]
pub struct KillPlayer;

fn kill_player(
    _trigger: Trigger<KillPlayer>,
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok(player) = player_query.get_single() else {
        error!("No player found");
        return;
    };

    commands.entity(player).despawn_recursive();
    warn!("Player killed");

    next_state.set(GameState::Respawning);
}
