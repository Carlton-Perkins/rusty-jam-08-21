use crate::entity_class::indexer::{IndexingError, ParseFields};
use crate::tags::Player;
use crate::{GameLayer, MainCamera};
use anyhow::anyhow;
use bevy::prelude::*;
use heron::{CollisionLayers, CollisionShape, RigidBody, RotationConstraints, Velocity};
use serde_json::Value;
use std::collections::HashMap;

enum StartEntity {
    Player,
    Enemy,
}

pub const START_LOCATION_ID: &str = "Start_Location";

pub struct StartLocation {
    character: StartEntity,
    count: i64,
    spawned: i64,
}

impl ParseFields for StartLocation {
    fn parse(fields: &HashMap<String, Option<Value>>) -> anyhow::Result<Self> {
        let character = fields.get("Character").ok_or(IndexingError::InvalidParse)?;
        let character_enum = match character {
            Some(Value::String(s)) => StartEntity::from_string(s.as_str()),
            _ => Err(IndexingError::InvalidParse)?,
        }?;

        let count = fields.get("Count").ok_or(IndexingError::InvalidParse)?;
        let count_value = match count {
            Some(Value::Number(n)) => n.as_i64().unwrap(),
            _ => Err(IndexingError::InvalidParse)?,
        };

        Ok(StartLocation {
            character: character_enum,
            count: count_value,
            spawned: 0,
        })
    }
}

impl StartEntity {
    pub fn from_string(s: &str) -> anyhow::Result<Self> {
        match s {
            "Player" => Ok(StartEntity::Player),
            "Enemy" => Ok(StartEntity::Enemy),
            _ => Err(anyhow!("No StartEntity binding for {}", s)),
        }
    }
}

pub fn spawn_from_spawn_location(
    mut c: Commands,
    mut q: Query<(&mut StartLocation, &Transform)>,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (mut start, location) in q.iter_mut() {
        if start.spawned < start.count {
            match start.character {
                StartEntity::Player => spawn_player(&mut c, &assets, &mut materials, location),
                StartEntity::Enemy => {
                    // todo!()
                }
            }
            start.spawned += 1;
        }
    }
}

fn spawn_player(
    mut c: &mut Commands,
    assets: &Res<AssetServer>,
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
    transform: &Transform,
) {
    let player_sprite = assets.load("player.sprite.png");

    // Spawn camera
    c.spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera)
        .id();

    c.spawn()
        .insert_bundle(SpriteBundle {
            material: materials.add(player_sprite.into()),
            transform: Transform::from_xyz(transform.translation.x, transform.translation.y, 100.),
            ..Default::default()
        })
        .insert(Player)
        .insert(Velocity::from_linear(Vec3::default()))
        .insert(RigidBody::Dynamic)
        .insert(RotationConstraints::lock())
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(20., 28., 0.),
            border_radius: None,
        })
        // .insert(TesselatedCollider {
        //     image: player_sprite,
        //     tesselator_config: TesselatedColliderConfig {
        //         vertice_separation: 0.,
        //         ..Default::default()
        //     },
        //     ..Default::default()
        // })
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
    // .insert_children(0, &[camera])
    ;
}
