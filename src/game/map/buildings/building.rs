use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    game::{
        assets::{HandleMap, ImageKey},
        map::tiles::{Tile, TileMap},
        physics::CollisionLayer,
        state::GameState,
    },
    palette,
    render_layer::RENDER_LAYER_BUILDING,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Setup), spawn_buildings);
    app.observe(spawn_building);
}

#[derive(Component)]
pub struct Building;

#[derive(Event)]
pub struct SpawnBuilding {
    pub position: UVec2,
}

#[derive(Component)]
struct Buildings;

fn spawn_buildings(mut commands: Commands) {
    commands.spawn((Name::new("Buildings"), SpatialBundle::default(), Buildings));
}

fn spawn_building(
    trigger: Trigger<SpawnBuilding>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    tile_map: Res<TileMap>,
    mut tiles: Query<&mut Tile>,
    buildings: Query<Entity, With<Buildings>>,
) {
    let Ok(buildings_entity) = buildings.get_single() else {
        error!("No buildings entity found");
        return;
    };

    let position = trigger.event().position;

    let Some(&tile_entity) = tile_map.tile(position) else {
        error!("Invalid tile position: {position:?}");
        return;
    };

    let Ok(mut tile) = tiles.get_mut(tile_entity) else {
        error!("Invalid tile entity: {tile_entity}");
        return;
    };

    let building_entity = commands
        .spawn((
            Name::new(format!("Building ({}, {})", position.x, position.y)),
            Building,
            SpriteBundle {
                sprite: Sprite {
                    color: palette::BUILDING_COLOR,
                    ..default()
                },
                texture: image_handles[&ImageKey::Building].clone_weak(),
                transform: Transform::from_translation(
                    tile_map
                        .tile_to_world(position)
                        .extend(RENDER_LAYER_BUILDING),
                ),
                ..default()
            },
            RigidBody::Static,
            Collider::rectangle(48., 48.),
            CollisionLayers::new([CollisionLayer::Building], [CollisionLayer::Player]),
        ))
        .set_parent(buildings_entity)
        .id();

    tile.building = Some(building_entity);
}
