use crate::entity_class::start_location::{StartLocation, START_LOCATION_ID};
use crate::map::MapEntity;
use anyhow::anyhow;
use bevy::prelude::*;
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IndexingError {
    #[error("No known type to parse to")]
    NoKnownType,

    #[error("Missing fields from type, failed to parse")]
    InvalidParse,
}
pub struct EntityIndexed;

pub trait ParseFields {
    fn parse(fields: &HashMap<String, Option<Value>>) -> anyhow::Result<Self>
    where
        Self: Sized;
}

pub fn index_entities(
    mut c: Commands,
    entities: Query<(Entity, &MapEntity), Without<EntityIndexed>>,
) {
    for (eid, entity) in entities.iter() {
        info!("Indexing entity {} {:?}", entity.name, eid);
        let fields = &entity.fields;
        let component_o = match entity.name.as_str() {
            START_LOCATION_ID => StartLocation::parse(fields),
            _ => {
                warn!("Unknown entity type {}", entity.name);
                Err(anyhow!("Unknown entity type {}", entity.name))
            }
        };

        if let Ok(component) = component_o {
            c.entity(eid).insert(component).insert(EntityIndexed);
        } else {
            error!("Failed to parse entity {} into a known type", entity.name);
            c.entity(eid).insert(EntityIndexed);
        }
    }
}
