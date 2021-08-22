use bevy::app::AppExit;
use bevy::asset::Asset;
use bevy::core::FixedTimestep;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_retrograde::core::bundles::SpriteSheetBundle;
use bevy_retrograde::core::components::SpriteSheet;
use bevy_retrograde::core::image::{load, DynamicImage, GenericImageView};
use bevy_retrograde::epaint::{Color32, Pos2, Stroke};
use bevy_retrograde::ldtk::{LdtkMap, LdtkMapBundle, LdtkMapLayer};
use bevy_retrograde::physics::PhysicsLayer;
use bevy_retrograde::physics::{
    create_convex_collider, CollisionLayers, Gravity, PhysicMaterial, RigidBody,
    RotationConstraints, TesselatedCollider, TesselatedColliderConfig, Velocity,
};
use bevy_retrograde::prelude::heron::rapier_plugin::PhysicsWorld;
use bevy_retrograde::prelude::{
    Camera, CameraBundle, CameraSize, Image, Shape, ShapeBundle, SpriteBundle,
};
use bevy_retrograde::RetroPlugins;
use heron::CollisionShape;
use rand::prelude::*;
use std::f32::consts::PI;
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
struct Player;
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
        .add_plugins(RetroPlugins)
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(LogDiagnosticsPlugin::default())
        .add_startup_system(setup_level.system())
        .add_startup_system(spawn_player.system())
        .add_system(update_map_collisions.system())
        .add_stage(
            GameStage,
            SystemStage::parallel()
                .with_run_criteria(FixedTimestep::step(0.015))
                .with_system(player_movement.system()),
        )
        .add_system(cast_projectile.system())
        .add_system(quit_system.system())
        .add_system(inspect_map.system())
        .add_system(player_vision_cone.system())
        .add_system(delete_debug_lines.system())
        .add_system(apply_lifetime.system())
        .run()
}

fn setup_level(mut c: Commands, asset_server: Res<AssetServer>) {
    // Enable HotReload
    asset_server
        .watch_for_changes()
        .expect("Unable to enable HotReload");

    // World
    c.spawn().insert_bundle(LdtkMapBundle {
        map: asset_server.load("map.ldtk"),
        // Center the map
        transform: Transform::from_xyz(
            -((SCREEN_WIDTH * 4.) / 2.),
            -((SCREEN_HEIGHT * 4.) / 2.),
            0.,
        ),
        ..Default::default()
    });
}

fn spawn_player(mut c: Commands, assets: Res<AssetServer>) {
    let player_sprite = assets.load("player.sprite.png");

    // Spawn camera
    let camera = c
        .spawn()
        .insert_bundle(CameraBundle {
            camera: Camera {
                size: CameraSize::FixedHeight(SCREEN_HEIGHT as u32 * 4),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MainCamera)
        .id();

    c.spawn()
        .insert_bundle(SpriteBundle {
            image: player_sprite.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..Default::default()
        })
        .insert(Player)
        .insert(Velocity::from_linear(Vec3::default()))
        .insert(RigidBody::Dynamic)
        .insert(RotationConstraints::lock())
        .insert(TesselatedCollider {
            image: player_sprite,
            tesselator_config: TesselatedColliderConfig {
                vertice_separation: 0.,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(
            CollisionLayers::none()
                .with_group(GameLayer::Player)
                .with_masks(&[GameLayer::World, GameLayer::Enemy]),
        )
        // .insert_bundle(ShapeBundle {
        //     shape: Shape::Circle {
        //         center: Default::default(),
        //         radius: 150.,
        //         fill: Color32::TRANSPARENT,
        //         stroke: Stroke::new(1., Color32::RED),
        //     },
        //     transform: Transform::from_xyz(0., 0., 1.),
        //     ..Default::default()
        // });
        .insert_children(0, &[camera]);
}

fn player_vision_cone(
    mut c: Commands,
    physics_world: PhysicsWorld,
    player_q: Query<(Entity, &Transform, &GlobalTransform), With<Player>>,
) {
    let samples = 10;
    let cone_radius = 150.;
    let slice = 2. * PI / samples as f32;

    // Spawn a point to the left and right of the player
    // let left_debug = c
    //     .spawn()
    //     .insert_bundle(ShapeBundle {
    //         shape: Shape::Circle {
    //             center: Default::default(),
    //             radius: 10.,
    //             fill: Color32::TRANSPARENT,
    //             stroke: Stroke::new(5., Color32::YELLOW),
    //         },
    //         transform: Transform::from_xyz(-cone_radius, 0., 5.),
    //         // global_transform: GlobalTransform::from_translation(
    //         //     local.translation + Vec3::new(-100., 0., 0.),
    //         // ),
    //         ..Default::default()
    //     })
    //     .id();
    // let right_debug = c
    //     .spawn()
    //     .insert_bundle(ShapeBundle {
    //         shape: Shape::Circle {
    //             center: Default::default(),
    //             radius: 10.,
    //             fill: Color32::TRANSPARENT,
    //             stroke: Stroke::new(5., Color32::YELLOW),
    //         },
    //         transform: Transform::from_xyz(cone_radius, 0., 5.),
    //         // global_transform: GlobalTransform::from_translation(
    //         //     local.translation + Vec3::new(-100., 0., 0.),
    //         // ),
    //         ..Default::default()
    //     })
    //     .id();
    //
    // let ray_circle = c
    //     .spawn()
    //     .insert_bundle(ShapeBundle {
    //         shape: Shape::Circle {
    //             center: Default::default(),
    //             radius: cone_radius,
    //             fill: Color32::TRANSPARENT,
    //             stroke: Stroke::new(5., Color32::BLUE),
    //         },
    //         transform: Transform::from_xyz(0., 0., 5.),
    //         // global_transform: GlobalTransform::from_translation(
    //         //     local.translation + Vec3::new(-100., 0., 0.),
    //         // ),
    //         ..Default::default()
    //     })
    //     .id();

    if let Ok((e_id, local, global)) = player_q.single() {
        // For sample in a circle around the player

        // let mut loop_debug_lines = vec![];

        for i in 0..samples {
            let deg = i as f32 * slice;
            let pos_y = deg.sin() * -cone_radius;
            let pos_x = deg.cos() * cone_radius;

            trace!(
                "Ray casting at {} -> {} {}",
                global.translation,
                pos_x,
                pos_y
            );

            let hit = physics_world.ray_cast_with_filter(
                global.translation,
                global.translation + Transform::from_xyz(pos_x, pos_y, 0.0).translation
                    - local.translation,
                true,
                CollisionLayers::none()
                    .with_group(GameLayer::World)
                    .with_mask(GameLayer::World),
                |x| true,
            );

            if let Some(info) = hit {
                trace!(
                    "Got a hit with distance {} ",
                    info.collision_point.distance(global.translation)
                );

                // // Draw the collision
                // c.spawn()
                //     .insert_bundle(ShapeBundle {
                //         shape: Shape::Circle {
                //             center: Default::default(),
                //             radius: 10.,
                //             fill: Color32::TRANSPARENT,
                //             stroke: Stroke::new(1., Color32::RED),
                //         },
                //         global_transform: GlobalTransform::from_xyz(
                //             info.collision_point.x,
                //             info.collision_point.y,
                //             3.,
                //         ),
                //         ..Default::default()
                //     })
                //     .insert(DebugLine);
                //
                // c.spawn()
                //     .insert_bundle(ShapeBundle {
                //         shape: Shape::Circle {
                //             center: Default::default(),
                //             radius: 10.,
                //             fill: Color32::TRANSPARENT,
                //             stroke: Stroke::new(2., Color32::YELLOW),
                //         },
                //         // transform: Transform::from_xyz(pos_x, pos_x, 3.),
                //         global_transform: GlobalTransform::from_xyz(
                //             center.translation.x + pos_x,
                //             center.translation.y + pos_y,
                //             3.,
                //         ),
                //         ..Default::default()
                //     })
                //     .insert(DebugLine);
                //
                // Draw the line Player -> collision
                // let ray_collide = c
                //     .spawn()
                //     .insert_bundle(ShapeBundle {
                //         shape: Shape::LineSegment {
                //             points: [
                //                 // Pos2::new(0., 0.),
                //                 Pos2::new(global.translation.x, global.translation.y),
                //                 Pos2::new(info.collision_point.x, info.collision_point.y),
                //             ],
                //             stroke: Stroke::new(5., Color32::RED),
                //         },
                //         // transform: Transform::from_xyz(0., 0., 3.),
                //         ..Default::default()
                //     })
                //     .insert(DebugLine)
                //     .id();
                // loop_debug_lines.push(ray_collide);

                // Draw the line Player -> Raycast
                // let ray_line = c
                //     .spawn()
                //     .insert_bundle(ShapeBundle {
                //         shape: Shape::LineSegment {
                //             points: [Pos2::new(0., 0.), Pos2::new(pos_x, pos_y)],
                //             stroke: Stroke::new(5., Color32::BLUE),
                //         },
                //         transform: Transform::from_xyz(0., 0., 3.),
                //         // global_transform: GlobalTransform::from_xyz(0., 0., 2.),
                //         ..Default::default()
                //     })
                //     .insert(DebugLine)
                //     .id();
                // loop_debug_lines.push(ray_line);

                // Draw the hit ray point
                // let ray_line = c
                //     .spawn()
                //     .insert_bundle(ShapeBundle {
                //         shape: Shape::Circle {
                //             center: Default::default(),
                //             radius: 2.,
                //             fill: Color32::TRANSPARENT,
                //             stroke: Stroke::new(5., Color32::YELLOW),
                //         },
                //         transform: Transform::from_xyz(
                //             info.collision_point.x - local.translation.x,
                //             info.collision_point.y - local.translation.y,
                //             10.,
                //         ),
                //         // global_transform: GlobalTransform::from_xyz(
                //         //     info.collision_point.x,
                //         //     info.collision_point.y,
                //         //     10.,
                //         // ),
                //         ..Default::default()
                //     })
                //     .insert(DebugLine)
                //     .id();
                // loop_debug_lines.push(ray_line);

                // Draw the target ray point
                // let ray_target = c
                //     .spawn()
                //     .insert_bundle(ShapeBundle {
                //         shape: Shape::Circle {
                //             center: Default::default(),
                //             radius: 2.,
                //             fill: Color32::TRANSPARENT,
                //             stroke: Stroke::new(5., Color32::GREEN),
                //         },
                //         transform: Transform::from_xyz(
                //             pos_x + global.translation.x - local.translation.x,
                //             pos_y + global.translation.y - local.translation.y,
                //             10.,
                //         ),
                //         // global_transform: GlobalTransform::from_xyz(
                //         //     info.collision_point.x,
                //         //     info.collision_point.y,
                //         //     10.,
                //         // ),
                //         ..Default::default()
                //     })
                //     .insert(DebugLine)
                //     .id();
                // loop_debug_lines.push(ray_target);
            }
        }

        // c.entity(e_id)
        //     .insert_children(0, &[left_debug, right_debug, ray_circle]);
        // c.entity(e_id).insert_children(1, &loop_debug_lines);
    }
}

struct Projectile;
struct Lifetime {
    lifetime: Timer,
}

// TODO generalize to any CASTER instead of player
// TODO abstract over input mode
/// Create projectile
fn cast_projectile(
    mut c: Commands,
    windows: Res<Windows>,
    input: Res<Input<MouseButton>>,
    player: Query<&Transform, With<Player>>,
    camera: Query<&Transform, With<MainCamera>>,
    asset_server: Res<AssetServer>,
    mut sprite_sheets: ResMut<Assets<SpriteSheet>>,
) {
    // On click, fire a projectile from the player with a velocity relative to the distance the cursor is from the player
    // projectile should have limited bounces and limited lifetime
    // Spread?
    let window = windows.get_primary().unwrap();
    let mut rng = rand::thread_rng();
    let projectile_spritesheet = asset_server.load("projectile.spritemap.png");

    if input.just_pressed(MouseButton::Left) {
        if let Ok(start) = player.single() {
            if let Some(cursor) = window.cursor_position() {
                let size = Vec2::new(window.width() as f32, window.height() as f32);

                // the default orthographic projection is in pixels from the center;
                // just undo the translation
                let p = cursor - size / 2.0;

                // assuming there is exactly one main camera entity, so this is OK
                let camera_transform = camera.single().unwrap();

                // apply the camera transform
                let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
                info!("World coords: {}/{}", pos_wld.x, pos_wld.y);

                let vel = (start.translation.truncate() + pos_wld.truncate().truncate());
                // let vel = Vec2::new(500., 0.);
                info!("Projectile velocity: {}", vel);
                info!("Player pos: {}", start.translation);

                c.spawn()
                    .insert_bundle(SpriteSheetBundle {
                        sprite_bundle: SpriteBundle {
                            image: projectile_spritesheet.clone(),
                            transform: Transform::from_xyz(
                                start.translation.x,
                                start.translation.y,
                                3.,
                            ),
                            ..Default::default()
                        },
                        sprite_sheet: sprite_sheets.add(SpriteSheet {
                            grid_size: UVec2::new(64, 64),
                            tile_index: 0,
                        }),
                    })
                    .insert(Projectile {})
                    .insert(Lifetime {
                        lifetime: Timer::from_seconds(rng.gen_range(1.0..2.0), false),
                    })
                    .insert(Velocity::from_linear(
                        vel.extend(0.) * Vec3::new(1., -1., 1.),
                    ))
                    .insert(RigidBody::Dynamic)
                    .insert(TesselatedCollider {
                        image: projectile_spritesheet,
                        tesselator_config: TesselatedColliderConfig {
                            vertice_separation: 0.,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(RotationConstraints::lock())
                    .insert(PhysicMaterial {
                        restitution: 0.0,
                        density: 1.0,
                        friction: 0.0,
                    })
                    .insert(CollisionLayers::all::<GameLayer>().with_groups([GameLayer::Player]));

                c.spawn()
                    .insert_bundle(ShapeBundle {
                        shape: Shape::Circle {
                            center: Default::default(),
                            radius: 2.,
                            fill: Color32::TRANSPARENT,
                            stroke: Stroke::new(5., Color32::GREEN),
                        },
                        transform: Transform::from_xyz(
                            start.translation.x,
                            start.translation.y,
                            10.,
                        ),
                        // global_transform: GlobalTransform::from_xyz(
                        //     info.collision_point.x,
                        //     info.collision_point.y,
                        //     10.,
                        // ),
                        ..Default::default()
                    })
                    .insert(DebugLine);
            }
        }
    }
}

fn show_colliders(q: Query<&CollisionShape>) {
    for shape in q.iter() {
        info!("Shape {:?}", shape);
    }
}

fn apply_lifetime(mut c: Commands, time: Res<Time>, mut q: Query<(Entity, &mut Lifetime)>) {
    for (e_id, mut lifetime) in q.iter_mut() {
        if lifetime.lifetime.tick(time.delta()).just_finished() {
            c.entity(e_id).despawn();
        }
    }
}

fn player_movement(input: Res<Input<KeyCode>>, mut q: Query<&mut Velocity, With<Player>>) {
    for mut real_vel in q.iter_mut() {
        let mut vel = real_vel.clone();
        let move_speed = 10.;
        let min_speed = 0.01;
        let max_speed = 100.;
        let friction = 0.95;

        // Adjust current velocity
        if input.pressed(KeyCode::W) {
            vel.linear.y -= move_speed
        };
        if input.pressed(KeyCode::A) {
            vel.linear.x -= move_speed
        };
        if input.pressed(KeyCode::S) {
            vel.linear.y += move_speed
        };
        if input.pressed(KeyCode::D) {
            vel.linear.x += move_speed
        };

        // Clamp velocity to MAX
        vel.linear
            .clamp(Vec3::splat(-max_speed), Vec3::splat(max_speed));

        // Apply velocity degradation due to friction
        vel.linear *= friction;

        // Zero out velocities lower then 0.00001

        if vel.linear.x.abs() <= min_speed {
            vel.linear.x = 0.
        }
        if vel.linear.y.abs() <= min_speed {
            vel.linear.y = 0.
        }

        // Only update if different
        // if real_vel.linear.abs_diff_eq(vel.linear, min_speed) {
        real_vel.linear = vel.linear;
        //     info!(
        //         "Updating player velocity From {:?} to {:?}",
        //         real_vel.linear, vel.linear
        //     );
        // }
    }
}

fn delete_debug_lines(mut c: Commands, q: Query<Entity, With<DebugLine>>) {
    for e in q.iter() {
        c.entity(e).despawn_recursive();
    }
}

fn inspect_map(q: Query<&LdtkMapLayer>, q2: Res<Assets<LdtkMap>>) {
    // if let Some(project) = q.iter().next() {
    //     let map_handle = &project.map;
    //     let map = q2.get(map_handle).unwrap();
    //     info!("project defs {:?}", map.project.defs.tilesets);
    // }
}

struct MapLayerLoaded;
/// This system will go through each layer in spawned maps and generate a collision shape for each tile
fn update_map_collisions(
    mut commands: Commands,
    map_layers: Query<(Entity, &LdtkMapLayer, &Handle<Image>), Without<MapLayerLoaded>>,
    image_assets: Res<Assets<Image>>,
) {
    for (layer_ent, map_layer, image_handle) in map_layers.iter() {
        // ( which should be fixed eventually by rust-analyzer )
        let map_layer: &LdtkMapLayer = map_layer;

        let image = if let Some(image) = image_assets.get(image_handle) {
            image
        } else {
            continue;
        };

        // Get the tile size of the map
        let tile_size = map_layer.layer_instance.__grid_size as u32;

        let mut layer_commands = commands.entity(layer_ent);

        // Only create collsions for things in the world layer for now
        // TODO make this smarter by reading the tag data of the tile set
        if map_layer.layer_instance.__identifier == "World" {
            // For every tile grid
            for tile_x in 0u32..map_layer.layer_instance.__c_wid as u32 {
                for tile_y in 0u32..map_layer.layer_instance.__c_hei as u32 {
                    // Get the tile image
                    let tile_img = image
                        .view(tile_x * tile_size, tile_y * tile_size, tile_size, tile_size)
                        .to_image();

                    // Try to generate a convex collision mesh from the tile
                    let mesh = create_convex_collider(
                        DynamicImage::ImageRgba8(tile_img),
                        &TesselatedColliderConfig {
                            // The maximum accuracy for collision mesh generation
                            vertice_separation: 0.,
                            ..Default::default()
                        },
                    );

                    // If mesh generation was successful ( wouldn't be for empty tiles, etc. )
                    if let Some(mesh) = mesh {
                        // Spawn a collider as a child of the map layer
                        layer_commands.with_children(|layer| {
                            layer
                                .spawn()
                                .insert_bundle((
                                    mesh,
                                    Transform::from_xyz(
                                        (tile_x * tile_size + tile_size / 2) as f32,
                                        (tile_y * tile_size + tile_size / 2) as f32,
                                        0.,
                                    ),
                                    GlobalTransform::default(),
                                ))
                                .insert(PhysicMaterial {
                                    friction: 1.0,
                                    restitution: 0.1,
                                    ..Default::default()
                                })
                                .insert(
                                    CollisionLayers::all::<GameLayer>()
                                        .with_group(GameLayer::World),
                                );
                        });
                    }
                }
            }
        }

        layer_commands
            // Make layer a static body
            .insert(RigidBody::Static)
            // Mark as loaded
            .insert(MapLayerLoaded);
    }
}

// Allows for the game to be quit via the ESC key
fn quit_system(input: Res<Input<KeyCode>>, mut app: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        // Quit Game
        app.send(AppExit);
        info!("Exiting cleanly via esc");
    }
}
