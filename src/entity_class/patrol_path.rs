use crate::entity_class::indexer::ParseFields;
use bevy::prelude::*;
use serde_json::Value;
use std::collections::HashMap;
use bevy_inspector_egui::Inspectable;

pub const PATROL_PATH_ID: &str = "Patrol_Path";

#[derive(Inspectable, Debug)]
pub struct PatrolPath;

impl ParseFields for PatrolPath {
    fn parse(fields: &HashMap<String, Option<Value>>) -> anyhow::Result<Self>
        where
            Self: Sized,
    {
        Ok(PatrolPath {})
    }
}