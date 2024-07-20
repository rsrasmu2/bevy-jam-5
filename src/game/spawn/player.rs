//! Spawn the player.

use bevy::prelude::*;
use iyes_progress::prelude::*;
use leafwing_input_manager::InputManagerBundle;

use crate::{
    camera::CameraFollow,
    game::{
        assets::{HandleMap, ImageKey},
        controls::player_action::PlayerAction,
        map::tiles::TILE_SIZE,
        movement::Movement,
        state::GameState,
    },
    palette,
    screen::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_player);
    app.register_type::<Player>();
    app.add_systems(
        Update,
        player_spawn_progress
            .track_progress()
            .run_if(in_state(GameState::Setup)),
    );
}

#[derive(Event, Debug)]
pub struct SpawnPlayer {
    pub position: UVec2,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

fn spawn_player(
    trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    let position = trigger.event().position.as_vec2() * TILE_SIZE;

    commands.spawn((
        Name::new("Player"),
        Player,
        SpriteBundle {
            sprite: Sprite {
                color: palette::PLAYER_COLOR,
                ..default()
            },
            texture: image_handles[&ImageKey::Player].clone_weak(),
            transform: Transform::from_translation(position.extend(2.)),
            ..Default::default()
        },
        InputManagerBundle::with_map(PlayerAction::default_input_map()),
        Movement { speed: 400. },
        CameraFollow,
        StateScoped(Screen::Playing),
    ));

    let Ok(mut camera_transform) = camera_query.get_single_mut() else {
        error!("No camera found");
        return;
    };

    camera_transform.translation = position.extend(0.);
}

fn player_spawn_progress(player_query: Query<Entity, With<Player>>) -> Progress {
    (player_query.iter().len() == 1).into()
}
