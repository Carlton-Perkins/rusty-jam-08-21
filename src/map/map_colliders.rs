use crate::map::MapTile;
use crate::tags::WorldType;
use crate::GameLayer;
use bevy::prelude::*;
use heron::{CollisionLayers, CollisionShape, RigidBody};

pub struct MapCollider;

pub fn generate_colliders_for_map_tiles(
    mut c: Commands,
    tiles: Query<(Entity, &MapTile), Without<MapCollider>>,
) {
    // TODO restrict to only WORLD tiles NOT BG tiles
    for (eid, tile) in tiles.iter() {
        if tile.world_type == WorldType::Wall {
            c.entity(eid)
                .insert(MapCollider)
                .insert(RigidBody::Static)
                .insert(CollisionShape::Cuboid {
                    half_extends: (tile.size / 2.).extend(tile.depth as f32),
                    border_radius: None,
                })
                .insert(
                    CollisionLayers::none()
                        .with_group(GameLayer::World)
                        .with_masks(&[GameLayer::Projectile, GameLayer::Enemy, GameLayer::Player]),
                );
        } else {
            c.entity(eid).insert(MapCollider);
        }
    }
}
