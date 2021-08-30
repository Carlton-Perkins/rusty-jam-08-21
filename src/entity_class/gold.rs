use crate::entity_class::indexer::{IndexingError, ParseFields};
use anyhow::anyhow;
use bevy::prelude::*;
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
