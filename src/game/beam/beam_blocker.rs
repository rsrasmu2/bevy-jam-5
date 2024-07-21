use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{game::beam::BEAM_IMAGE_LENGTH, screen::Screen};

use super::{Beam, BeamSystemSet};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        resize_beam
            .in_set(BeamSystemSet::SetBeamLength)
            .run_if(in_state(Screen::Playing)),
    );
}

#[derive(Component)]
pub struct BeamBlocker;

fn resize_beam(
    mut beam_query: Query<(&mut Transform, &ShapeHits, &mut Beam)>,
    blocker_query: Query<Entity, With<BeamBlocker>>,
) {
    for (mut transform, hits, mut beam) in beam_query.iter_mut() {
        let mut smallest_distance_sqr = Beam::DEFAULT_LENGTH * Beam::DEFAULT_LENGTH;
        for hit in hits.iter() {
            warn!("Hit: {:?}", hit);
            if blocker_query.contains(hit.entity) {
                warn!("Blocker found");
                let hit_dist_sqr = transform.translation.xy().distance_squared(hit.point1);
                if hit_dist_sqr < smallest_distance_sqr {
                    smallest_distance_sqr = hit_dist_sqr;
                }
            }
        }
        let smallest_distance = smallest_distance_sqr.sqrt();
        beam.length = smallest_distance;
        transform.scale.y = smallest_distance / BEAM_IMAGE_LENGTH;
    }
}
