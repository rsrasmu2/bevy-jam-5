use bevy::prelude::*;

use crate::{game::state::GameState, stable_interpolate::StableInterpolate};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, camera_follow.run_if(in_state(GameState::Playing)));
}

#[derive(Component)]
pub struct CameraFollow;

fn camera_follow(
    follow_query: Query<&Transform, (With<CameraFollow>, Without<Camera>)>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    const CAMERA_DECAY_RATE: f32 = 15.;

    let Ok(follow_transform) = follow_query.get_single() else {
        return;
    };

    let Ok(mut camera_transform) = camera_query.get_single_mut() else {
        error!("No camera found");
        return;
    };

    camera_transform.translation.smooth_nudge(
        &follow_transform.translation,
        CAMERA_DECAY_RATE,
        time.delta_seconds(),
    );
}
