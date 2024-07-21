use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::InputManagerBundle;

use crate::{
    camera::CameraFollow,
    game::{
        assets::{HandleMap, ImageKey},
        beam::beam_blocker::BeamBlocker,
        map::tiles::TILE_SIZE,
        movement::Movement,
        physics::CollisionLayer,
    },
    palette,
    render_layer::{RENDER_LAYER_PLAYER, RENDER_LAYER_PLAYER_SPAWNER},
    screen::Screen,
};

use crate::game::player::Player;

use super::controls::player_action::PlayerAction;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_player_spawner);
    app.observe(spawn_player);
}

#[derive(Component)]
pub struct PlayerSpawner;

#[derive(Event)]
pub struct SpawnPlayerSpawner {
    pub position: UVec2,
}

#[derive(Event, Debug)]
pub struct SpawnPlayer;

fn spawn_player_spawner(
    trigger: Trigger<SpawnPlayerSpawner>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
) {
    commands.spawn((
        Name::new("Player Spawner"),
        PlayerSpawner,
        SpriteBundle {
            sprite: Sprite {
                color: palette::PLAYER_SPAWNER_COLOR,
                ..default()
            },
            texture: image_handles[&ImageKey::PlayerSpawner].clone_weak(),
            transform: Transform::from_translation(
                (trigger.event().position.as_vec2() * TILE_SIZE)
                    .extend(RENDER_LAYER_PLAYER_SPAWNER),
            ),
            ..Default::default()
        },
        RigidBody::Static,
        Collider::circle(32.),
        CollisionLayers::new([CollisionLayer::BeamBlocker], [CollisionLayer::Beam]),
        BeamBlocker,
    ));
}

fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    spawners_query: Query<&Transform, With<PlayerSpawner>>,
) {
    let position = spawners_query
        .get_single()
        .unwrap()
        .translation
        .with_z(RENDER_LAYER_PLAYER);

    commands.spawn((
        Name::new("Player"),
        Player,
        SpriteBundle {
            sprite: Sprite {
                color: palette::PLAYER_COLOR,
                ..default()
            },
            texture: image_handles[&ImageKey::Player].clone_weak(),
            transform: Transform::from_translation(position),
            ..Default::default()
        },
        InputManagerBundle::with_map(PlayerAction::default_input_map()),
        Movement {
            acceleration: 6000.,
        },
        CameraFollow,
        StateScoped(Screen::Playing),
        RigidBody::Dynamic,
        Collider::circle(24.),
        CollisionLayers::new(
            [CollisionLayer::Player],
            [CollisionLayer::Building, CollisionLayer::Beam],
        ),
    ));
}
