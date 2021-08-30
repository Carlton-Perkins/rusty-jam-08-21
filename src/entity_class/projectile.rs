use crate::entity_class::health::Damaged;
use crate::entity_class::lifetime::Lifetime;
use crate::tags::{MainCamera, Player};
use crate::GameLayer;
use bevy::prelude::*;
use bevy_egui::egui::{Color32, Shape, Stroke};
use bevy_prototype_debug_lines::DebugLines;
use heron::{
    CollisionEvent, CollisionLayers, CollisionShape, PhysicMaterial, RigidBody,
    RotationConstraints, Velocity,
};
use rand::Rng;

pub struct Projectile {
    pub size: Vec2,
    pub damage: i32,
}

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub projectile: Projectile,
    pub collision_layers: CollisionLayers,
    pub velocity: Velocity,
}

impl Default for ProjectileBundle {
    fn default() -> Self {
        ProjectileBundle {
            projectile: Default::default(),
            collision_layers: Default::default(),
            velocity: Default::default(),
        }
    }
}

impl Default for Projectile {
    fn default() -> Self {
        Projectile {
            size: Vec2::new(30., 30.),
            damage: 10,
        }
    }
}

// If an entity containing health collides with a projectile, damage that entity
pub fn on_collide_apply_damage(
    mut c: Commands,
    mut damaged: EventWriter<Damaged>,
    mut collisions: EventReader<CollisionEvent>,
    projectiles: Query<&Projectile>,
) {
    collisions
        .iter()
        .filter(|x| x.is_started())
        .filter_map(|x| {
            let (eid_1, eid_2) = x.rigid_body_entities();
            let (layer_1, layer_2) = x.collision_layers();

            if is_creature(layer_1) && is_projectile(layer_2) {
                Some((eid_1, eid_2))
            } else if is_projectile(layer_1) && is_creature(layer_2) {
                Some((eid_2, eid_1))
            } else {
                None
            }
        })
        .for_each(|(creature_id, projectile_id)| {
            if let Ok(projectile) = projectiles.get(projectile_id) {
                damaged.send(Damaged {
                    damage: projectile.damage,
                    entity: creature_id,
                });
                c.entity(projectile_id).despawn();
            }
        });
}

pub fn on_collide_despawn(mut c: Commands, mut collisions: EventReader<CollisionEvent>) {
    collisions
        .iter()
        .filter(|x| x.is_started())
        .filter_map(|x| {
            let (eid_1, eid_2) = x.rigid_body_entities();
            let (layer_1, layer_2) = x.collision_layers();

            if is_world(layer_1) && is_projectile(layer_2) {
                Some((eid_1, eid_2))
            } else if is_projectile(layer_1) && is_world(layer_2) {
                Some((eid_2, eid_1))
            } else {
                None
            }
        })
        .for_each(|(_, projectile_id)| {
            c.entity(projectile_id).despawn();
        });
}

fn is_creature(layers: CollisionLayers) -> bool {
    use GameLayer::*;
    layers.contains_group(Player) || layers.contains_group(Enemy)
    // (layers.contains_group(GameLayer::Player)
    //     || layers.contains_group(GameLayer::Enemy)
    //     || layers.contains_group(GameLayer::World))
    //     && !layers.contains_group(GameLayer::Projectile)
}

fn is_projectile(layers: CollisionLayers) -> bool {
    use GameLayer::*;
    layers.contains_group(Projectile)
    // !(layers.contains_group(GameLayer::Player)
    //     || layers.contains_group(GameLayer::Enemy)
    //     || layers.contains_group(GameLayer::World))
    //     && layers.contains_group(GameLayer::Projectile)
}

fn is_world(layers: CollisionLayers) -> bool {
    use GameLayer::*;
    layers.contains_group(World)
    // !(layers.contains_group(GameLayer::Player)
    //     || layers.contains_group(GameLayer::Enemy)
    //     || layers.contains_group(GameLayer::Projectile))
    //     && layers.contains_group(GameLayer::World)
}

// TODO generalize to any CASTER instead of player
// TODO abstract over input mode
// Create projectile
pub fn cast_projectile(
    mut c: Commands,
    windows: Res<Windows>,
    input: Res<Input<MouseButton>>,
    player: Query<&Transform, With<Player>>,
    camera: Query<&Transform, With<MainCamera>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // On click, fire a projectile from the player with a velocity relative to the distance the cursor is from the player
    // projectile should have limited bounces and limited lifetime
    // Spread?
    let window = windows.get_primary().unwrap();
    let mut rng = rand::thread_rng();
    let projectile_spritesheet = asset_server.load("projectile.spritemap.png");
    let texture_atlas = TextureAtlas::from_grid(projectile_spritesheet, Vec2::new(64., 64.), 4, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

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
                let vel = (start.translation.truncate() - pos_wld.truncate().truncate()) * 1.;

                c.spawn()
                    .insert_bundle(SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle,
                        transform: Transform::from_xyz(
                            start.translation.x,
                            start.translation.y,
                            100.,
                        ),
                        ..Default::default()
                    })
                    .insert_bundle(ProjectileBundle {
                        projectile: Projectile { size, damage: 50 },
                        collision_layers: CollisionLayers::none()
                            .with_group(GameLayer::Projectile)
                            .with_masks(&[GameLayer::World, GameLayer::Enemy, GameLayer::Player]),
                        velocity: Velocity::from_linear(vel.extend(0.) * Vec3::new(-1., -1., 1.)),
                    })
                    .insert(Lifetime {
                        lifetime: Timer::from_seconds(rng.gen_range(1.0..2.0), false),
                    })
                    .insert(RigidBody::Dynamic)
                    .insert(CollisionShape::Cuboid {
                        half_extends: Vec3::new(6., 6., 0.),
                        border_radius: None,
                    })
                    .insert(RotationConstraints::lock())
                    .insert(PhysicMaterial {
                        restitution: 0.0,
                        density: 1.0,
                        friction: 0.0,
                    });
            }
        }
    }
}
