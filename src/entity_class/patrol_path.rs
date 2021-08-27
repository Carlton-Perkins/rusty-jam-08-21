use crate::entity_class::indexer::{IndexingError, ParseFields};
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use serde_json::Value;
use std::collections::HashMap;

pub const PATROL_PATH_ID: &str = "Patrol_Path";

#[derive(Inspectable, Debug)]
pub struct PatrolPath;

impl ParseFields for PatrolPath {
    fn parse(fields: &HashMap<String, Option<Value>>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let path = fields.get("Path").ok_or(IndexingError::InvalidParse)?;
        let path_arr: Vec<Vec2> = match path {
            Some(Value::Array(arr)) => arr
                .iter()
                .map(|node| {
                    let pos: anyhow::Result<Vec2> = match node {
                        Value::Object(node_map) => {
                            let x = match node_map.get("cx").ok_or(IndexingError::InvalidParse) {
                                Ok(Value::Number(n)) => n.as_f64().unwrap(),
                                _ => Err(IndexingError::InvalidParse)?,
                            };
                            let y = match node_map.get("cy").ok_or(IndexingError::InvalidParse) {
                                Ok(Value::Number(n)) => n.as_f64().unwrap(),
                                _ => Err(IndexingError::InvalidParse)?,
                            };

                            Ok(Vec2::new(x as f32, y as f32))
                        }
                        _ => Err(IndexingError::InvalidParse)?,
                    };
                    pos
                })
                .map(|x| x.unwrap())
                .collect(),
            _ => Err(IndexingError::InvalidParse)?,
        };

        Ok(PatrolPath {})
    }
}
