mod entity_class;
mod map;
pub mod tags;

use crate::entity_class::EntityClasses;
use crate::map::{MapLocation, MapPlugin, MapScale};
use crate::tags::MainCamera;
use crate::tags::Player;
use bevy::app::AppExit;
use bevy::asset::AssetPath;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_inspector_egui::{InspectorPlugin, WorldInspectorPlugin};
use bevy_prototype_debug_lines::DebugLinesPlugin;
use heron::prelude::*;
use ldtk_rust::Project;
use std::collections::HashMap;
use std::path::Path;

// use bevy_retrograde::prelude::*;

const SCREEN_HEIGHT: f32 = 256.;
const SCREEN_WIDTH: f32 = 256.;

#[derive(PhysicsLayer)]
pub enum GameLayer {
    World,
    Player,
    Enemy,
    Projectile,
}

// Marker Tags
struct DebugLine;

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
        .add_plugin(DebugLinesPlugin)
        .add_system(quit_system.system())
        .add_system(ui.system())
        .run()
}

fn ui(
    ui_context: Res<EguiContext>,
    windows: Res<Windows>,
    q_player: Query<&Transform, With<Player>>,
) {
    let window = windows.get_primary().unwrap();
    let mut cursor_pos = window.cursor_position().unwrap_or_default();

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
