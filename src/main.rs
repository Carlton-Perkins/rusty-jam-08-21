mod entity_class;
mod map;
mod tags;

use crate::entity_class::EntityClasses;
use crate::entity_class::Player;
use crate::map::MapLocation;
use crate::map::MapPlugin;
use crate::map::MapScale;
use bevy::app::AppExit;
use bevy::asset::AssetPath;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_inspector_egui::WorldInspectorPlugin;
use heron::prelude::*;
use ldtk_rust::Project;
use std::collections::HashMap;
use std::path::Path;

// use bevy_retrograde::prelude::*;

const SCREEN_HEIGHT: f32 = 256.;
const SCREEN_WIDTH: f32 = 256.;

#[derive(PhysicsLayer)]
enum GameLayer {
    World,
    Player,
    Enemy,
}

// Marker Tags
struct DebugLine;
struct MainCamera;

#[derive(StageLabel, Debug, Eq, Hash, PartialEq, Clone)]
struct GameStage;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            width: SCREEN_WIDTH * 4.,
            height: SCREEN_HEIGHT * 4.,
            title: "Illusion Of Security".to_string(),
            ..Default::default()
        })
        .insert_resource(Gravity::from(Vec3::new(0.0, 0.0, 0.0)))
        .insert_resource(MapLocation("assets/map.ldtk".into()))
        .insert_resource(MapScale(0.25))
        .add_plugins(DefaultPlugins)
        .add_plugin(MapPlugin)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(EguiPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(EntityClasses)
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_startup_system(setup_level.system())
        // .add_startup_system(spawn_player.system())
        // .add_system(update_map_collisions.system())
        // .add_stage(
        //     GameStage,
        //     SystemStage::parallel()
        //         .with_run_criteria(FixedTimestep::step(0.015))
        //         .with_system(player_movement.system()),
        // )
        // .add_system(cast_projectile.system())
        .add_system(quit_system.system())
        .add_system(ui.system())
        .add_system(move_camera_with_player.system())
        // .add_system(inspect_map.system())
        // .add_system(player_vision_cone.system())
        // .add_system(delete_debug_lines.system())
        // .add_system(apply_lifetime.system())
        .run()
}

// struct LdtkMapLayer {}
// struct LdtkMapTileSet {
//     enums: HashMap<i64, String>, // TODO allow for serde load into enum type
//     custom_data: HashMap<i64, String>, // TODO give structure to custom_data
// }
//
// struct LdtkEnumTags {}

fn setup_level(
    mut c: Commands,
    asset_server: Res<AssetServer>,
    mut textures_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Enable HotReload
    asset_server
        .watch_for_changes()
        .expect("Unable to enable HotReload");

    let ldtk = Project::new("assets/map.ldtk".into());
    let mut tileset_textures = HashMap::new();

    // Todo Enums

    // Load all tilesets
    for tileset in ldtk.defs.tilesets {
        let id = tileset.uid;
        let sprite_path = tileset.rel_path;
        let tile_size = Vec2::new(tileset.px_wid as f32, tileset.px_hei as f32);
        let texture_handle = asset_server.load(AssetPath::new_ref(Path::new(&sprite_path), None));
        let atlas = TextureAtlas::from_grid(
            texture_handle.clone(),
            tile_size,
            tileset.c_wid as usize,
            tileset.c_hei as usize,
        );
        let texture_handle_atlas = textures_atlases.add(atlas);

        // let mut tile_enum_values = HashMap::new();
        // for enum_label in tileset.enum_tags {
        //     enum_label.iter().for_each(|(key, value)| {
        //         if let Some(v) = value {
        //             info!("{} - {:?}", key, v);
        //             match v {
        //                 Value::Array(ids) => {
        //                     for id_u in ids {
        //                         match id_u {
        //                             Value::Number(id) => {
        //                                 tile_enum_values.insert(id.as_i64().unwrap(), key.clone());
        //                             }
        //                             _ => {
        //                                 panic!(
        //                                     "Expected number in enum tile id position, got {:?}",
        //                                     id_u
        //                                 )
        //                             }
        //                         }
        //                     }
        //                 }
        //                 _ => {
        //                     panic!("Expected array in enum value position, got {:?}", v)
        //                 }
        //             }
        //         }
        //     })
        // }
        // let extra_data = LdtkMapTileSet {
        //     enums: HashtMap::new(),
        //     custom_data: HashMap::new(), // TODO Populate for real
        // };
        info!("Loading tileset {}...", sprite_path);
        tileset_textures.insert(id, (texture_handle.clone(), texture_handle_atlas));

        // c.spawn().insert_bundle(SpriteBundle {
        //     material: materials.add(texture_handle.clone().into()),
        //     ..Default::default()
        // });
    }

    // Load all levels
    for level in ldtk.levels {
        let layer_instances = level.layer_instances.as_ref().unwrap();
        for (z_layer, layer_instance) in layer_instances.iter().rev().enumerate() {
            let layer_eid = c
                .spawn()
                // .insert(LdtkMapLayer {})
                .insert(Transform::default())
                .id();

            match layer_instance.layer_instance_type.as_str() {
                "Tiles" => {
                    let tileset_id = layer_instance.tileset_def_uid.unwrap();
                    let (tileset_texture, tileset_atlas) =
                        tileset_textures.get(&tileset_id).unwrap();
                    let grid_tiles = &layer_instance.grid_tiles;
                    for tile in grid_tiles {
                        let tile_id = tile.t;
                        let tile_eid = c
                            .spawn()
                            //     .insert_bundle(SpriteBundle {
                            //         material: materials.add(tileset_texture.clone().into()),
                            //         ..Default::default()
                            //     })
                            .insert_bundle(SpriteSheetBundle {
                                sprite: TextureAtlasSprite::new(tile_id as u32),
                                texture_atlas: tileset_atlas.clone(),
                                transform: Transform {
                                    translation: Vec3::new(
                                        tile.px[0] as f32,
                                        -tile.px[1] as f32,
                                        0.,
                                    ),
                                    scale: Vec3::splat(0.25),
                                    rotation: Default::default(),
                                },
                                ..Default::default()
                            })
                            // .insert(LdtkMapTileSet {
                            //     enums: extra_data.enums.clone(),
                            //     custom_data: extra_data.custom_data.clone(),
                            // })
                            .id();

                        if tile.t == 0 {
                            c.entity(tile_eid)
                                .insert(CollisionShape::Cuboid {
                                    half_extends: Vec3::new(
                                        (layer_instance.grid_size as f32 - 2.) / 2.,
                                        (layer_instance.grid_size as f32 - 2.) / 2.,
                                        0.0,
                                    ),
                                    border_radius: None,
                                })
                                .insert(RigidBody::Static)
                                .insert(
                                    CollisionLayers::none()
                                        .with_group(GameLayer::World)
                                        .with_masks(&[GameLayer::Player, GameLayer::Enemy]),
                                );
                        }
                        // c.entity(layer_eid).push_children(&[tile_eid]);
                        // info!("Tile : {:?}", tile.px);
                    }
                }
                mode => {
                    warn!(
                        "Skipping loading for layer {}, of type {}",
                        layer_instance.identifier, mode
                    );
                }
            }
        }
    }

    // World
    // c.spawn().insert_bundle(LdtkMapBundle {
    //     map: asset_server.load("map.ldtk"),
    //     // Center the map
    //     transform: Transform::from_xyz(
    //         -((SCREEN_WIDTH * 4.) / 2.),
    //         -((SCREEN_HEIGHT * 4.) / 2.),
    //         0.,
    //     ),
    //     ..Default::default()
    // });
}

fn move_camera_with_player(
    mut q: QuerySet<(
        Query<&mut Transform, With<MainCamera>>,
        Query<&Transform, With<Player>>,
    )>,
) {
    let mut new_translation = Vec3::default();
    if let Ok(player) = q.q1().single() {
        new_translation = player.translation;
    }

    if let Ok(mut camera) = q.q0_mut().single_mut() {
        camera.translation = new_translation;
    }
}

fn ui(
    ui_context: Res<EguiContext>,
    windows: Res<Windows>,
    q_player: Query<&Transform, With<Player>>,
) {
    let window = windows.get_primary().unwrap();
    let mut cursor_pos = window.cursor_position().unwrap_or_default();
    // let player = q_player.single()().unwrap_or(Transform::default());

    egui::Window::new("Cursor").show(ui_context.ctx(), |ui| {
        ui.label("Pos:");
        ui.add(egui::Slider::new(&mut cursor_pos.x, 0.0..=window.width()).text("x"));
        ui.add(egui::Slider::new(&mut cursor_pos.y, 0.0..=window.height()).text("y"));
        // ui.add(egui::Label::new(format!(
        //     "player x {}",
        //     player.translation.x
        // )));
        // ui.add(egui::Label::new(format!(
        //     "player y {}",
        //     player.translation.y
        // )));
    });
}

// fn player_vision_cone(
//     mut c: Commands,
//     physics_world: PhysicsWorld,
//     player_q: Query<(Entity, &Transform, &GlobalTransform), With<Player>>,
// ) {
//     let samples = 10;
//     let cone_radius = 150.;
//     let slice = 2. * PI / samples as f32;
//
//     // Spawn a point to the left and right of the player
//     // let left_debug = c
//     //     .spawn()
//     //     .insert_bundle(ShapeBundle {
//     //         shape: Shape::Circle {
//     //             center: Default::default(),
//     //             radius: 10.,
//     //             fill: Color32::TRANSPARENT,
//     //             stroke: Stroke::new(5., Color32::YELLOW),
//     //         },
//     //         transform: Transform::from_xyz(-cone_radius, 0., 5.),
//     //         // global_transform: GlobalTransform::from_translation(
//     //         //     local.translation + Vec3::new(-100., 0., 0.),
//     //         // ),
//     //         ..Default::default()
//     //     })
//     //     .id();
//     // let right_debug = c
//     //     .spawn()
//     //     .insert_bundle(ShapeBundle {
//     //         shape: Shape::Circle {
//     //             center: Default::default(),
//     //             radius: 10.,
//     //             fill: Color32::TRANSPARENT,
//     //             stroke: Stroke::new(5., Color32::YELLOW),
//     //         },
//     //         transform: Transform::from_xyz(cone_radius, 0., 5.),
//     //         // global_transform: GlobalTransform::from_translation(
//     //         //     local.translation + Vec3::new(-100., 0., 0.),
//     //         // ),
//     //         ..Default::default()
//     //     })
//     //     .id();
//     //
//     // let ray_circle = c
//     //     .spawn()
//     //     .insert_bundle(ShapeBundle {
//     //         shape: Shape::Circle {
//     //             center: Default::default(),
//     //             radius: cone_radius,
//     //             fill: Color32::TRANSPARENT,
//     //             stroke: Stroke::new(5., Color32::BLUE),
//     //         },
//     //         transform: Transform::from_xyz(0., 0., 5.),
//     //         // global_transform: GlobalTransform::from_translation(
//     //         //     local.translation + Vec3::new(-100., 0., 0.),
//     //         // ),
//     //         ..Default::default()
//     //     })
//     //     .id();
//
//     if let Ok((e_id, local, global)) = player_q.single() {
//         // For sample in a circle around the player
//
//         // let mut loop_debug_lines = vec![];
//
//         for i in 0..samples {
//             let deg = i as f32 * slice;
//             let pos_y = deg.sin() * -cone_radius;
//             let pos_x = deg.cos() * cone_radius;
//
//             trace!(
//                 "Ray casting at {} -> {} {}",
//                 global.translation,
//                 pos_x,
//                 pos_y
//             );
//
//             let hit = physics_world.ray_cast_with_filter(
//                 global.translation,
//                 global.translation + Transform::from_xyz(pos_x, pos_y, 0.0).translation
//                     - local.translation,
//                 true,
//                 CollisionLayers::none()
//                     .with_group(GameLayer::World)
//                     .with_mask(GameLayer::World),
//                 |x| true,
//             );
//
//             if let Some(info) = hit {
//                 trace!(
//                     "Got a hit with distance {} ",
//                     info.collision_point.distance(global.translation)
//                 );
//
//                 // // Draw the collision
//                 // c.spawn()
//                 //     .insert_bundle(ShapeBundle {
//                 //         shape: Shape::Circle {
//                 //             center: Default::default(),
//                 //             radius: 10.,
//                 //             fill: Color32::TRANSPARENT,
//                 //             stroke: Stroke::new(1., Color32::RED),
//                 //         },
//                 //         global_transform: GlobalTransform::from_xyz(
//                 //             info.collision_point.x,
//                 //             info.collision_point.y,
//                 //             3.,
//                 //         ),
//                 //         ..Default::default()
//                 //     })
//                 //     .insert(DebugLine);
//                 //
//                 // c.spawn()
//                 //     .insert_bundle(ShapeBundle {
//                 //         shape: Shape::Circle {
//                 //             center: Default::default(),
//                 //             radius: 10.,
//                 //             fill: Color32::TRANSPARENT,
//                 //             stroke: Stroke::new(2., Color32::YELLOW),
//                 //         },
//                 //         // transform: Transform::from_xyz(pos_x, pos_x, 3.),
//                 //         global_transform: GlobalTransform::from_xyz(
//                 //             center.translation.x + pos_x,
//                 //             center.translation.y + pos_y,
//                 //             3.,
//                 //         ),
//                 //         ..Default::default()
//                 //     })
//                 //     .insert(DebugLine);
//                 //
//                 // Draw the line Player -> collision
//                 // let ray_collide = c
//                 //     .spawn()
//                 //     .insert_bundle(ShapeBundle {
//                 //         shape: Shape::LineSegment {
//                 //             points: [
//                 //                 // Pos2::new(0., 0.),
//                 //                 Pos2::new(global.translation.x, global.translation.y),
//                 //                 Pos2::new(info.collision_point.x, info.collision_point.y),
//                 //             ],
//                 //             stroke: Stroke::new(5., Color32::RED),
//                 //         },
//                 //         // transform: Transform::from_xyz(0., 0., 3.),
//                 //         ..Default::default()
//                 //     })
//                 //     .insert(DebugLine)
//                 //     .id();
//                 // loop_debug_lines.push(ray_collide);
//
//                 // Draw the line Player -> Raycast
//                 // let ray_line = c
//                 //     .spawn()
//                 //     .insert_bundle(ShapeBundle {
//                 //         shape: Shape::LineSegment {
//                 //             points: [Pos2::new(0., 0.), Pos2::new(pos_x, pos_y)],
//                 //             stroke: Stroke::new(5., Color32::BLUE),
//                 //         },
//                 //         transform: Transform::from_xyz(0., 0., 3.),
//                 //         // global_transform: GlobalTransform::from_xyz(0., 0., 2.),
//                 //         ..Default::default()
//                 //     })
//                 //     .insert(DebugLine)
//                 //     .id();
//                 // loop_debug_lines.push(ray_line);
//
//                 // Draw the hit ray point
//                 // let ray_line = c
//                 //     .spawn()
//                 //     .insert_bundle(ShapeBundle {
//                 //         shape: Shape::Circle {
//                 //             center: Default::default(),
//                 //             radius: 2.,
//                 //             fill: Color32::TRANSPARENT,
//                 //             stroke: Stroke::new(5., Color32::YELLOW),
//                 //         },
//                 //         transform: Transform::from_xyz(
//                 //             info.collision_point.x - local.translation.x,
//                 //             info.collision_point.y - local.translation.y,
//                 //             10.,
//                 //         ),
//                 //         // global_transform: GlobalTransform::from_xyz(
//                 //         //     info.collision_point.x,
//                 //         //     info.collision_point.y,
//                 //         //     10.,
//                 //         // ),
//                 //         ..Default::default()
//                 //     })
//                 //     .insert(DebugLine)
//                 //     .id();
//                 // loop_debug_lines.push(ray_line);
//
//                 // Draw the target ray point
//                 // let ray_target = c
//                 //     .spawn()
//                 //     .insert_bundle(ShapeBundle {
//                 //         shape: Shape::Circle {
//                 //             center: Default::default(),
//                 //             radius: 2.,
//                 //             fill: Color32::TRANSPARENT,
//                 //             stroke: Stroke::new(5., Color32::GREEN),
//                 //         },
//                 //         transform: Transform::from_xyz(
//                 //             pos_x + global.translation.x - local.translation.x,
//                 //             pos_y + global.translation.y - local.translation.y,
//                 //             10.,
//                 //         ),
//                 //         // global_transform: GlobalTransform::from_xyz(
//                 //         //     info.collision_point.x,
//                 //         //     info.collision_point.y,
//                 //         //     10.,
//                 //         // ),
//                 //         ..Default::default()
//                 //     })
//                 //     .insert(DebugLine)
//                 //     .id();
//                 // loop_debug_lines.push(ray_target);
//             }
//         }
//
//         // c.entity(e_id)
//         //     .insert_children(0, &[left_debug, right_debug, ray_circle]);
//         // c.entity(e_id).insert_children(1, &loop_debug_lines);
//     }
// }

// struct Projectile;
// struct Lifetime {
//     lifetime: Timer,
// }

// TODO generalize to any CASTER instead of player
// TODO abstract over input mode
// Create projectile
// fn cast_projectile(
//     mut c: Commands,
//     windows: Res<Windows>,
//     input: Res<Input<MouseButton>>,
//     player: Query<&Transform, With<Player>>,
//     camera: Query<&Transform, With<MainCamera>>,
//     asset_server: Res<AssetServer>,
//     mut sprite_sheets: ResMut<Assets<SpriteSheet>>,
// ) {
//     // On click, fire a projectile from the player with a velocity relative to the distance the cursor is from the player
//     // projectile should have limited bounces and limited lifetime
//     // Spread?
//     let window = windows.get_primary().unwrap();
//     let mut rng = rand::thread_rng();
//     let projectile_spritesheet = asset_server.load("projectile.spritemap.png");
//
//     if input.just_pressed(MouseButton::Left) {
//         if let Ok(start) = player.single() {
//             if let Some(cursor) = window.cursor_position() {
//                 let size = Vec2::new(window.width() as f32, window.height() as f32);
//
//                 // the default orthographic projection is in pixels from the center;
//                 // just undo the translation
//                 let p = cursor - size / 2.0;
//
//                 // assuming there is exactly one main camera entity, so this is OK
//                 let camera_transform = camera.single().unwrap();
//
//                 // apply the camera transform
//                 let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
//                 info!("World coords: {}/{}", pos_wld.x, pos_wld.y);
//
//                 let vel = (start.translation.truncate() + pos_wld.truncate().truncate());
//                 // let vel = Vec2::new(500., 0.);
//                 info!("Projectile velocity: {}", vel);
//                 info!("Player pos: {}", start.translation);
//
//                 c.spawn()
//                     .insert_bundle(SpriteSheetBundle {
//                         sprite_bundle: SpriteBundle {
//                             image: projectile_spritesheet.clone(),
//                             transform: Transform::from_xyz(
//                                 start.translation.x,
//                                 start.translation.y,
//                                 3.,
//                             ),
//                             ..Default::default()
//                         },
//                         sprite_sheet: sprite_sheets.add(SpriteSheet {
//                             grid_size: UVec2::new(64, 64),
//                             tile_index: 0,
//                         }),
//                     })
//                     .insert(Projectile {})
//                     .insert(Lifetime {
//                         lifetime: Timer::from_seconds(rng.gen_range(1.0..2.0), false),
//                     })
//                     .insert(Velocity::from_linear(
//                         vel.extend(0.) * Vec3::new(1., -1., 1.),
//                     ))
//                     .insert(RigidBody::Dynamic)
//                     .insert(TesselatedCollider {
//                         image: projectile_spritesheet,
//                         tesselator_config: TesselatedColliderConfig {
//                             vertice_separation: 0.,
//                             ..Default::default()
//                         },
//                         ..Default::default()
//                     })
//                     .insert(RotationConstraints::lock())
//                     .insert(PhysicMaterial {
//                         restitution: 0.0,
//                         density: 1.0,
//                         friction: 0.0,
//                     })
//                     .insert(CollisionLayers::all::<GameLayer>().with_groups([GameLayer::Player]));
//
//                 // c.spawn()
//                 //     .insert_bundle(ShapeBundle {
//                 //         shape: Shape::Circle {
//                 //             center: Default::default(),
//                 //             radius: 2.,
//                 //             fill: Color32::TRANSPARENT,
//                 //             stroke: Stroke::new(5., Color32::GREEN),
//                 //         },
//                 //         transform: Transform::from_xyz(
//                 //             start.translation.x,
//                 //             start.translation.y,
//                 //             10.,
//                 //         ),
//                 //         // global_transform: GlobalTransform::from_xyz(
//                 //         //     info.collision_point.x,
//                 //         //     info.collision_point.y,
//                 //         //     10.,
//                 //         // ),
//                 //         ..Default::default()
//                 //     })
//                 //     .insert(DebugLine);
//             }
//         }
//     }
// }

// fn show_colliders(q: Query<&CollisionShape>) {
//     for shape in q.iter() {
//         info!("Shape {:?}", shape);
//     }
// }

// fn apply_lifetime(mut c: Commands, time: Res<Time>, mut q: Query<(Entity, &mut Lifetime)>) {
//     for (e_id, mut lifetime) in q.iter_mut() {
//         if lifetime.lifetime.tick(time.delta()).just_finished() {
//             c.entity(e_id).despawn();
//         }
//     }
// }

// fn delete_debug_lines(mut c: Commands, q: Query<Entity, With<DebugLine>>) {
//     for e in q.iter() {
//         c.entity(e).despawn_recursive();
//     }
// }

// fn inspect_map(q: Query<&LdtkMapLayer>, q2: Res<Assets<LdtkMap>>) {
//     // if let Some(project) = q.iter().next() {
//     //     let map_handle = &project.map;
//     //     let map = q2.get(map_handle).unwrap();
//     //     info!("project defs {:?}", map.project.defs.tilesets);
//     // }
// }

// struct MapLayerLoaded;
// /// This system will go through each layer in spawned maps and generate a collision shape for each tile
// fn update_map_collisions(
//     mut commands: Commands,
//     map_layers: Query<(Entity, &LdtkMapLayer, &Handle<Image>), Without<MapLayerLoaded>>,
//     image_assets: Res<Assets<Image>>,
// ) {
//     for (layer_ent, map_layer, image_handle) in map_layers.iter() {
//         // ( which should be fixed eventually by rust-analyzer )
//         let map_layer: &LdtkMapLayer = map_layer;
//
//         let image = if let Some(image) = image_assets.get(image_handle) {
//             image
//         } else {
//             continue;
//         };
//
//         // Get the tile size of the map
//         let tile_size = map_layer.layer_instance.__grid_size as u32;
//
//         let mut layer_commands = commands.entity(layer_ent);
//
//         // Only create collsions for things in the world layer for now
//         // TODO make this smarter by reading the tag data of the tile set
//         if map_layer.layer_instance.__identifier == "World" {
//             // For every tile grid
//             for tile_x in 0u32..map_layer.layer_instance.__c_wid as u32 {
//                 for tile_y in 0u32..map_layer.layer_instance.__c_hei as u32 {
//                     // Get the tile image
//                     let tile_img = image
//                         .view(tile_x * tile_size, tile_y * tile_size, tile_size, tile_size)
//                         .to_image();
//
//                     // Try to generate a convex collision mesh from the tile
//                     let mesh = create_convex_collider(
//                         DynamicImage::ImageRgba8(tile_img),
//                         &TesselatedColliderConfig {
//                             // The maximum accuracy for collision mesh generation
//                             vertice_separation: 0.,
//                             ..Default::default()
//                         },
//                     );
//
//                     // If mesh generation was successful ( wouldn't be for empty tiles, etc. )
//                     if let Some(mesh) = mesh {
//                         // Spawn a collider as a child of the map layer
//                         layer_commands.with_children(|layer| {
//                             layer
//                                 .spawn()
//                                 .insert_bundle((
//                                     mesh,
//                                     Transform::from_xyz(
//                                         (tile_x * tile_size + tile_size / 2) as f32,
//                                         (tile_y * tile_size + tile_size / 2) as f32,
//                                         0.,
//                                     ),
//                                     GlobalTransform::default(),
//                                 ))
//                                 .insert(PhysicMaterial {
//                                     friction: 1.0,
//                                     restitution: 0.1,
//                                     ..Default::default()
//                                 })
//                                 .insert(
//                                     CollisionLayers::all::<GameLayer>()
//                                         .with_group(GameLayer::World),
//                                 );
//                         });
//                     }
//                 }
//             }
//         }
//
//         layer_commands
//             // Make layer a static body
//             .insert(RigidBody::Static)
//             // Mark as loaded
//             .insert(MapLayerLoaded);
//     }
// }

/// Allows for the game to be quit via the ESC key
fn quit_system(input: Res<Input<KeyCode>>, mut app: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        // Quit Game
        app.send(AppExit);
        info!("Exiting cleanly via esc");
    }
}
