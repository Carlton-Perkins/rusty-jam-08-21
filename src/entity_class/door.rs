use crate::entity_class::indexer::ParseFields;
use bevy::prelude::*;
use serde_json::Value;
use std::collections::HashMap;

pub const DOOR_ID: &str = "Door";
pub struct Door;

impl ParseFields for Door {
    fn parse(fields: &HashMap<String, Option<Value>>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Door {})
    }
}
