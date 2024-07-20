use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::game::{movement::Movement, spawn::player::Player, state::GameState};

use super::player_action::PlayerAction;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, player_movement.run_if(in_state(GameState::Playing)));
}

fn player_movement(
    mut query: Query<(&mut Transform, &Movement, &ActionState<PlayerAction>), With<Player>>,
    time: Res<Time>,
) {
    let Ok((mut transform, movement, action_state)) = query.get_single_mut() else {
        error!("Player not found");
        return;
    };

    let axis = action_state.clamped_axis_pair(&PlayerAction::Move).xy();
    if axis.length_squared() < f32::EPSILON {
        return;
    }

    let Ok(direction) = Dir2::new(axis) else {
        error!("Invalid direction: {:?}", axis);
        return;
    };

    transform.translation +=
        (direction.as_vec2() * movement.speed * time.delta_seconds()).extend(0.);
}
