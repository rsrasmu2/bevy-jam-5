use bevy::{prelude::*, utils::HashMap};

use crate::{
    game::assets::{HandleMap, ImageKey},
    palette,
    render_layer::{RENDER_LAYER_GRID, RENDER_LAYER_TILE},
};

pub const TILE_SIZE: f32 = 64.;
pub const TILE_SIZE_HALF: f32 = TILE_SIZE / 2.;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_tile_map);
    app.observe(despawn_tile_map);
    app.observe(set_hovered_tile);
    app.register_type::<Tile>();
    app.register_type::<TileMap>();
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub struct Tile {
    position: UVec2,
    grid: Entity,
    pub building: Option<Entity>,
}

impl Tile {
    pub fn new(position: UVec2, grid: Entity) -> Self {
        Self {
            position,
            grid,
            building: None,
        }
    }

    #[allow(dead_code)]
    pub fn position(&self) -> UVec2 {
        self.position
    }

    pub fn world_position(&self) -> Vec2 {
        self.position.as_vec2() * TILE_SIZE
    }
}

#[derive(Resource, Reflect)]
pub struct TileMap {
    size: UVec2,
    tile_entities: HashMap<UVec2, Entity>,
}

impl TileMap {
    #[allow(dead_code)]
    pub fn new(size: UVec2) -> Self {
        Self {
            size,
            tile_entities: HashMap::with_capacity((size.x * size.y) as usize),
        }
    }

    #[allow(dead_code)]
    pub fn size(&self) -> UVec2 {
        self.size
    }

    pub fn tile(&self, position: UVec2) -> Option<&Entity> {
        self.tile_entities.get(&position)
    }

    pub fn world_to_tile(&self, position: Vec2) -> Option<UVec2> {
        let tile_position = position / TILE_SIZE;
        if (0. ..self.size.x as f32).contains(&tile_position.x)
            && (0. ..self.size.y as f32).contains(&tile_position.y)
        {
            Some(tile_position.as_uvec2())
        } else {
            None
        }
    }

    pub fn tile_to_world(&self, position: UVec2) -> Vec2 {
        position.as_vec2() * TILE_SIZE
    }
}

#[derive(Event)]
pub struct SetHoveredTile {
    pub position: Option<UVec2>,
}

#[derive(Resource, Debug, Default)]
pub struct HoveredTile {
    position: Option<UVec2>,
}

impl HoveredTile {
    pub fn position(&self) -> Option<UVec2> {
        self.position
    }
}

fn set_hovered_tile(
    trigger: Trigger<SetHoveredTile>,
    mut hovered_tile: ResMut<HoveredTile>,
    tile_map: Res<TileMap>,
    tile_query: Query<&Tile>,
    mut grid_query: Query<&mut Sprite, With<TileGrid>>,
) {
    if hovered_tile.position == trigger.event().position {
        error!("Hovered tile is already set to {:?}", hovered_tile.position);
        return;
    }

    if let Some(prev_tile_pos) = hovered_tile.position {
        set_grid_color(
            &tile_query,
            &mut grid_query,
            &tile_map,
            prev_tile_pos,
            palette::GRID_COLOR,
        );
    };

    if let Some(new_tile_pos) = trigger.event().position {
        set_grid_color(
            &tile_query,
            &mut grid_query,
            &tile_map,
            new_tile_pos,
            palette::GRID_HOVERED_COLOR,
        );
    }

    hovered_tile.position = trigger.event().position;
}

fn set_grid_color(
    tile_query: &Query<&Tile>,
    grid_query: &mut Query<&mut Sprite, With<TileGrid>>,
    tile_map: &TileMap,
    tile_pos: UVec2,
    color: Color,
) {
    let Some(tile_entity) = tile_map.tile(tile_pos) else {
        error!("Invalid tile position: {tile_pos:?}");
        return;
    };

    let Ok(tile) = tile_query.get(*tile_entity) else {
        error!("Invalid tile entity: {tile_entity:?}");
        return;
    };

    let Ok(mut grid_sprite) = grid_query.get_mut(tile.grid) else {
        error!("Invalid grid entity for tile: {tile:?}");
        return;
    };

    grid_sprite.color = color;
}

#[derive(Event, Debug)]
pub struct SpawnTileMap {
    pub size: UVec2,
}

#[derive(Event, Debug)]
pub struct DespawnTileMap;

#[derive(Component)]
pub struct TileGrid;

#[derive(Component)]
struct Tiles;

fn spawn_tile_map(
    trigger: Trigger<SpawnTileMap>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
) {
    let size = trigger.event().size;
    let mut tile_entities = HashMap::with_capacity((size.x * size.y) as usize);

    commands.insert_resource(HoveredTile::default());

    let parent = commands
        .spawn((Name::new("Tiles"), SpatialBundle::default(), Tiles))
        .id();

    let tile_sprite = Sprite {
        color: palette::TILE_COLOR,
        ..default()
    };

    let grid_sprite = Sprite {
        color: palette::GRID_COLOR,
        ..default()
    };

    for x in 0..size.x {
        for y in 0..size.y {
            let tile_pos = UVec2::new(x, y);

            let grid = commands
                .spawn((
                    Name::new(format!("Grid ({x}, {y})")),
                    TileGrid,
                    SpriteBundle {
                        sprite: grid_sprite.clone(),
                        texture: image_handles[&ImageKey::TileGrid].clone_weak(),
                        transform: Transform::from_translation(
                            Vec2::splat(0.).extend(RENDER_LAYER_GRID),
                        ),
                        ..default()
                    },
                ))
                .id();

            let tile = Tile::new(tile_pos, grid);
            let tile_entity = commands
                .spawn((
                    Name::new(format!("Tile ({x}, {y})")),
                    tile,
                    SpriteBundle {
                        sprite: tile_sprite.clone(),
                        texture: image_handles[&ImageKey::Tile].clone_weak(),
                        transform: Transform::from_translation(
                            tile.world_position().extend(RENDER_LAYER_TILE),
                        ),
                        ..default()
                    },
                ))
                .set_parent(parent)
                .id();

            commands.entity(grid).set_parent(tile_entity);

            tile_entities.insert(tile_pos, tile_entity);
        }
    }

    commands.insert_resource(TileMap {
        size,
        tile_entities,
    });
}

fn despawn_tile_map(
    _trigger: Trigger<DespawnTileMap>,
    mut commands: Commands,
    tiles_query: Query<Entity, With<Tiles>>,
) {
    commands.remove_resource::<TileMap>();
    commands.remove_resource::<HoveredTile>();
    for tiles in tiles_query.iter() {
        commands.entity(tiles).despawn_recursive();
    }
}
