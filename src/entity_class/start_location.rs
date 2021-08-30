use crate::entity_class::enemy::spawn_enemy;
use crate::entity_class::indexer::{IndexingError, ParseFields};
use crate::entity_class::player::spawn_player;
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
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for (mut start, location) in q.iter_mut() {
        if start.spawned < start.count {
            match start.character {
                StartEntity::Player => {
                    spawn_player(&mut c, &assets, &mut texture_atlases, location)
                }
                StartEntity::Enemy => spawn_enemy(&mut c, &assets, &mut texture_atlases, location),
            }
            start.spawned += 1;
        }
    }
}

// this probably should only be run during init, but its good enough for now...
pub fn mark_spawns_as_invisible(mut q: Query<&mut Visible, With<StartLocation>>) {
    for mut n in q.iter_mut() {
        n.is_visible = false;
    }
}
