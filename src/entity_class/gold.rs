use crate::entity_class::enemy::spawn_enemy;
use crate::entity_class::indexer::{IndexingError, ParseFields};
use crate::entity_class::player::spawn_player;
use crate::tags::Player;
use crate::{GameLayer, MainCamera};
use anyhow::anyhow;
use bevy::prelude::*;
use bevy_egui::egui::Shape;
use bevy_prototype_debug_lines::DebugLines;
use heron::{CollisionLayers, CollisionShape, RigidBody, RotationConstraints, Velocity};
use serde_json::Value;
use std::collections::HashMap;

pub struct Gold {
    value: i32,
}

pub const GOLD_ID: &str = "Gold";

impl ParseFields for Gold {
    fn parse(fields: &HashMap<String, Option<Value>>) -> anyhow::Result<Self> {
        let value = fields
            .get("Value")
            .ok_or_else(|| IndexingError::InvalidParse)?;
        let value_number = match value {
            Some(Value::Number(n)) => n.as_i64().unwrap(),
            _ => Err(IndexingError::InvalidParse)?,
        };

        Ok(Gold {
            value: value_number as i32,
        })
    }
}

pub fn setup_gold(
    mut c: Commands,
    mut q: Query<(Entity, &mut Transform), (With<Gold>, Without<Sprite>)>,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut debug_lines: ResMut<DebugLines>,
) {
    // let gold_sprite = assets.load("gold.sprite.png");

    for (gold, mut transform) in q.iter_mut() {
        // c.entity(gold).insert(SpriteBundle {
        //     material: materials.add(gold_sprite.clone().into()),
        //     ..Default::default()
        // });
        // transform.translation += Vec3::new(0., 0., 150.)
        // debug_lines.line(Vec3::default(), transform.translation, 10.);
    }
}
