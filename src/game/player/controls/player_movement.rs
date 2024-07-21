use avian2d::dynamics::rigid_body::LinearVelocity;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::game::{movement::Movement, player::Player, state::GameState};

use super::player_action::PlayerAction;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (player_movement, dampen_movement)
            .chain()
            .run_if(in_state(GameState::Playing)),
    );
}

fn player_movement(
    mut query: Query<(&mut LinearVelocity, &Movement, &ActionState<PlayerAction>), With<Player>>,
    time: Res<Time>,
) {
    let Ok((mut velocity, movement, action_state)) = query.get_single_mut() else {
        // Player does not exist
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

    velocity.0 += direction.as_vec2() * movement.acceleration * time.delta_seconds();
}

fn dampen_movement(mut query: Query<&mut LinearVelocity, With<Player>>) {
    const DAMPEN_FACTOR: f32 = 0.9;

    let Ok(mut velocity) = query.get_single_mut() else {
        // Player does not exist
        return;
    };

    velocity.0 *= DAMPEN_FACTOR;
}
