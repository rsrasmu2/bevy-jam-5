use avian2d::prelude::*;
use bevy::{prelude::*, sprite::Anchor};

use crate::{palette, render_layer::RENDER_LAYER_BEAM, screen::Screen};

use super::{
    assets::{HandleMap, ImageKey},
    physics::CollisionLayer,
    player::{KillPlayer, Player},
    state::GameState,
};

const BEAM_IMAGE_WIDTH: f32 = 6.;
const BEAM_IMAGE_LENGTH: f32 = 32.;

pub mod beam_blocker;

pub(super) fn plugin(app: &mut App) {
    app.configure_sets(
        Update,
        (
            BeamSystemSet::SetBeamRotation,
            BeamSystemSet::SetBeamLength,
            BeamSystemSet::CheckPlayerCollision,
        ),
    );
    app.add_plugins(beam_blocker::plugin);
    app.observe(spawn_beam);
    app.observe(rotate_beam);
    app.observe(stop_rotate_beam);
    app.add_systems(
        Update,
        (
            player_collision
                .in_set(BeamSystemSet::CheckPlayerCollision)
                .run_if(in_state(GameState::Playing)),
            apply_rotation
                .in_set(BeamSystemSet::SetBeamRotation)
                .run_if(in_state(Screen::Playing)),
        ),
    );
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum BeamSystemSet {
    SetBeamRotation,
    SetBeamLength,
    CheckPlayerCollision,
}

#[derive(Component)]
pub struct Beam {
    pub length: f32,
    rotation: f32,
}

impl Beam {
    const DEFAULT_LENGTH: f32 = 512.;
}

impl Default for Beam {
    fn default() -> Self {
        Self {
            length: Self::DEFAULT_LENGTH,
            rotation: 0.,
        }
    }
}

#[derive(Event)]
pub struct SpawnBeam {
    pub position: Vec2,
    pub rotation: f32,
}

fn spawn_beam(
    trigger: Trigger<SpawnBeam>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
) {
    let c = trigger.event().rotation.cos();
    let s = trigger.event().rotation.sin();

    let up = Dir2::Y;

    let x = up.x * c + up.y * -s;
    let y = up.x * s + up.y * c;

    let dir = Dir2::new_unchecked(Vec2::new(x, y));

    commands.spawn((
        Name::new("Beam"),
        Beam::default(),
        SpriteBundle {
            sprite: Sprite {
                color: palette::BEAM_COLOR,
                anchor: Anchor::BottomCenter,
                ..default()
            },
            texture: image_handles[&ImageKey::Beam].clone_weak(),
            transform: Transform::from_translation(
                trigger.event().position.extend(RENDER_LAYER_BEAM),
            )
            .with_rotation(Quat::from_rotation_z(trigger.event().rotation))
            .with_scale(Vec3::new(1., Beam::DEFAULT_LENGTH / BEAM_IMAGE_LENGTH, 1.)),
            ..default()
        },
        ShapeCaster::new(Collider::circle(BEAM_IMAGE_WIDTH / 2.), Vec2::ZERO, 0., dir)
            .with_query_filter(SpatialQueryFilter::from_mask(
                CollisionLayer::Player.to_bits() | CollisionLayer::BeamBlocker.to_bits(),
            )),
    ));
}

fn player_collision(
    mut commands: Commands,
    shapecast_query: Query<(&Transform, &ShapeHits, &Beam)>,
    player: Query<&Transform, With<Player>>,
) {
    for (beam_transform, hits, beam) in &shapecast_query {
        for hit in hits.iter() {
            if let Ok(player_transform) = player.get(hit.entity) {
                let distance = (player_transform.translation - beam_transform.translation).length();
                if distance < beam.length {
                    commands.trigger(KillPlayer);
                }
            }
        }
    }
}

#[derive(Event)]
pub struct RotateBeam {
    pub direction: f32,
}

#[derive(Event)]
pub struct StopRotateBeam;

fn rotate_beam(trigger: Trigger<RotateBeam>, mut query: Query<&mut Beam>) {
    const ROTATION_SPEED: f32 = 1.5;

    let Ok(mut beam) = query.get_mut(trigger.entity()) else {
        error!("No beam found");
        return;
    };

    beam.rotation = trigger.event().direction * ROTATION_SPEED;
}

fn stop_rotate_beam(trigger: Trigger<StopRotateBeam>, mut query: Query<&mut Beam>) {
    let Ok(mut beam) = query.get_mut(trigger.entity()) else {
        error!("No beam found");
        return;
    };

    beam.rotation = 0.;
}

fn apply_rotation(mut query: Query<(&mut Transform, &Beam)>, time: Res<Time>) {
    for (mut transform, beam) in query.iter_mut() {
        let rot_amount = beam.rotation * time.delta_seconds();
        transform.rotate_z(rot_amount);
    }
}
