mod map_colliders;

use crate::map::map_colliders::generate_colliders_for_map_tiles;
use crate::tags::{world_type_from_str, WorldType};
use bevy::prelude::*;
use ldtk_rust::RenderMode::Tile;
use ldtk_rust::{EntityInstance, Project};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

pub struct MapPlugin;

pub struct MapLocation(pub String);
pub struct MapScale(pub f32);

struct MapDoneLoading;

pub struct MapTile {
    size: Vec2,
    depth: f32,
    world_type: WorldType,
}

pub struct Map {
    pub ldtk_map: Project,
    current_level: usize,
    reload: bool,
}

struct MapAssets {
    sprite_sheets: HashMap<i32, Handle<TextureAtlas>>,
    entity_materials: HashMap<i32, Handle<ColorMaterial>>,
    // tilemap_enum_defs: HashMap<i32, HashMap<i64, Vec<WorldType>>>,
    tilemap_custom_data: HashMap<i32, HashMap<i64, String>>,
}

pub struct MapEntity {
    pub name: String,
    pub fields: HashMap<String, Option<Value>>,
}

struct MapLayerInfo {
    grid_width: i32,
    grid_height: i32,
    grid_size: i32,
    depth: i32,
    px_width: f32,
    px_height: f32,
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(init_map.system())
            .add_system(update_map.system())
            .add_system(generate_colliders_for_map_tiles.system());
    }
}

#[derive(Deserialize, Debug)]
struct MapEnumMapping {
    #[serde(rename(deserialize = "enumValueId"))]
    enum_value_id: String,

    #[serde(rename(deserialize = "tileIds"))]
    tile_ids: Vec<i64>,
}

#[derive(Deserialize, Debug)]
struct MapEnumTileIds {
    tile_ids: Vec<i32>,
}

fn init_map(
    mut c: Commands,
    map_path: Res<MapLocation>,
    asset_server: Res<AssetServer>,
    mut textures_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // TODO use bevy asset system to load the path
    let map = Map {
        ldtk_map: ldtk_rust::Project::new(map_path.0.clone()),
        current_level: 0,
        reload: true,
    };

    let mut map_assets = MapAssets {
        sprite_sheets: HashMap::new(),
        entity_materials: HashMap::new(),
        tilemap_custom_data: HashMap::new(),
    };

    // Load all tilesets
    for tileset in map.ldtk_map.defs.tilesets.iter() {
        let id = tileset.uid;
        let name = &tileset.identifier;
        let sprite_path = &tileset.rel_path;
        let tile_size = Vec2::new(tileset.px_wid as f32, tileset.px_hei as f32);
        let texture_handle = asset_server.load(&sprite_path[..]);
        let atlas = TextureAtlas::from_grid(
            texture_handle,
            tile_size,
            tileset.c_wid as usize,
            tileset.c_hei as usize,
        );
        let texture_atlas_handle = textures_atlases.add(atlas);

        // Convert enum data to []id -> enum[] instead of []enum -> []id

        let mut data_map = HashMap::new();
        for enum_map in tileset.custom_data.iter() {
            let enum_label = enum_map.get("tileId").unwrap().as_ref().unwrap();
            let enum_value = enum_map.get("data").unwrap().as_ref().unwrap();

            let tile_id = match enum_label {
                Value::Number(s) => s.as_i64().unwrap(),
                v => panic!("Can not parse {}", v),
            };
            let value = match enum_value {
                Value::String(s) => s,
                v => panic!("Can not parse {}", v),
            };

            data_map.insert(tile_id, value.clone());
        }
        map_assets.tilemap_custom_data.insert(id as i32, data_map);

        info!("Loading tileset {} from {}...", name, sprite_path);
        map_assets
            .sprite_sheets
            .insert(id as i32, texture_atlas_handle);
    }

    // Load all entity assets
    for entity in map.ldtk_map.defs.entities.iter() {
        let id = entity.uid;
        let name = &entity.identifier;
        let size = Vec3::new(entity.width as f32, entity.height as f32, 0.0);
        // Skip first char, since its the '#'
        let color_code = &entity.color.clone()[1..];
        let color = match Color::hex(&color_code) {
            Ok(t) => t,
            Err(e) => {
                error!("Failed to parse color, defaulting to blue, {:?}", e);
                Color::BLUE
            }
        };
        let material = materials.add(ColorMaterial::from(color));

        info!("Loading entity {}..", name);
        map_assets.entity_materials.insert(id as i32, material);
    }

    c.insert_resource(map);
    c.insert_resource(map_assets)
}

fn update_map(mut c: Commands, mut map: ResMut<Map>, assets: Res<MapAssets>, scale: Res<MapScale>) {
    // Only run if work needs to be done
    if !map.reload {
        return;
    }

    for (layer_z, layer) in map.ldtk_map.levels[map.current_level]
        .layer_instances
        .as_ref()
        .unwrap()
        .iter()
        .enumerate()
        .rev()
    {
        let tileset_uid = layer.tileset_def_uid.unwrap_or(-1) as i32;
        let layer_uid = layer.layer_def_uid as i32;
        let layer_name = &layer.identifier;
        let layer_type = &layer.layer_instance_type[..];

        info!("Spawning Layer {} of type {}", layer_name, layer_type);

        let layer_info = MapLayerInfo {
            grid_width: layer.c_wid as i32,
            grid_height: layer.c_hei as i32,
            grid_size: layer.grid_size as i32,
            depth: (25 - layer_z as i32) * 2,
            px_width: layer.c_wid as f32 * (layer.grid_size as f32 * scale.0),
            px_height: layer.c_hei as f32 * (layer.grid_size as f32 * scale.0),
        };

        match layer_type {
            "Tiles" => {
                for tile in layer.grid_tiles.iter() {
                    // TODO flip controls
                    // TODO bake the static layers

                    c.spawn()
                        .insert_bundle(SpriteSheetBundle {
                            transform: Transform {
                                translation: convert_to_world(
                                    layer_info.px_width,
                                    layer_info.px_height,
                                    layer_info.grid_size,
                                    scale.0,
                                    tile.px[0] as i32,
                                    tile.px[1] as i32,
                                    layer_info.depth,
                                ),
                                rotation: Default::default(),
                                scale: Vec3::splat(scale.0),
                            },
                            sprite: TextureAtlasSprite::new(tile.t as u32),
                            texture_atlas: assets.sprite_sheets.get(&tileset_uid).unwrap().clone(),
                            ..Default::default()
                        })
                        .insert(MapTile {
                            size: Vec2::new(layer_info.px_width, layer_info.px_height) * scale.0,
                            depth: layer_info.depth as f32,
                            world_type: world_type_from_str(
                                assets
                                    .tilemap_custom_data
                                    .get(&tileset_uid)
                                    .unwrap_or(&HashMap::new())
                                    .get(&tile.t)
                                    .unwrap_or(&String::new()),
                            )
                            .unwrap_or(WorldType::Air),
                        });
                }
            }
            "Entities" => {
                for entity in layer.entity_instances.iter() {
                    let name = &entity.identifier;

                    let mut fields = HashMap::new();
                    // Construct hashmap from fields, worry about parsing later
                    for field in &entity.field_instances {
                        let field_name = field.identifier.clone();
                        let field_value = field.value.clone();
                        fields.insert(field_name, field_value);
                    }

                    let transform = Transform {
                        translation: convert_to_world(
                            layer_info.px_width,
                            layer_info.px_height,
                            layer_info.grid_size,
                            scale.0,
                            entity.px[0] as i32,
                            entity.px[1] as i32,
                            layer_info.depth,
                        ),
                        rotation: Default::default(),
                        scale: Default::default(),
                    };

                    if let Some(tile) = &entity.tile {
                        c.spawn()
                            .insert_bundle(SpriteSheetBundle {
                                transform: transform,
                                sprite: TextureAtlasSprite::new(tile.tileset_uid as u32),
                                texture_atlas: assets
                                    .sprite_sheets
                                    .get(&(tile.tileset_uid as i32))
                                    .unwrap()
                                    .clone(),
                                ..Default::default()
                            })
                            .insert(MapEntity {
                                name: name.to_string(),
                                fields,
                            })
                            .insert(GlobalTransform::default());
                    } else {
                        c.spawn()
                            .insert(MapEntity {
                                name: name.to_string(),
                                fields,
                            })
                            .insert(transform)
                            .insert(GlobalTransform::default());
                    }
                }
            }
            _ => {
                warn!(
                    "Not Implemented: Skipping loading for layer {}, of type {}",
                    layer_name, layer_type
                );
            }
        }
    }

    map.reload = false;
}
// LDtk 0,0 is the top left, moving -x,-y
// Bevy 0,0 is the center of the screen
// Need to offset and flip y
pub fn convert_to_world(
    width: f32,
    height: f32,
    grid_size: i32,
    scale: f32,
    x: i32,
    y: i32,
    z: i32,
) -> Vec3 {
    let world_x = x as f32;
    let world_y = -y as f32;
    let world_z = z as f32;
    // info!(
    //     "Spawning tile at {:?}",
    //     Vec3::new(world_x, world_y, world_z)
    // );
    Vec3::new(world_x, world_y, world_z)
}
