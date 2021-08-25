use crate::entity_class::door::{Door, DOOR_ID};
use crate::entity_class::start_location::{StartLocation, START_LOCATION_ID};
use crate::map::MapEntity;
use anyhow::{anyhow, Error};
use bevy::ecs::component::Component;
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

// TODO Convert to serde...
pub trait ParseFields {
    fn parse(fields: &HashMap<String, Option<Value>>) -> anyhow::Result<Self>
    where
        Self: Sized;
}

fn insert<'a>(
    eid: Entity,
    name: &'a String,
    s: Result<impl Component, Error>,
    mut c: Commands<'a>,
) -> Commands<'a> {
    if let Ok(component) = s {
        c.entity(eid).insert(component).insert(EntityIndexed);
    } else {
        error!("Failed to parse entity {} into a known type", name);
        c.entity(eid).insert(EntityIndexed);
    }
    c
}

pub fn index_entities(
    mut c_orig: Commands,
    entities: Query<(Entity, &MapEntity), Without<EntityIndexed>>,
) {
    let mut c = c_orig;
    for (eid, entity) in entities.iter() {
        info!("Indexing entity {} {:?}", entity.name, eid);
        let fields = &entity.fields;
        let name = &entity.name;
        c = match entity.name.as_str() {
            START_LOCATION_ID => insert(eid, name, StartLocation::parse(fields), c),
            DOOR_ID => insert(eid, name, Door::parse(fields), c),
            _ => {
                warn!("Unknown entity type {}", entity.name);

                // Err(anyhow!("Unknown entity type {}", entity.name))
                insert(eid, name, Ok(EntityIndexed), c)
            }
        };

        // if let Ok(component) = component_o {
        //     c.entity(eid).insert(component).insert(EntityIndexed);
        // } else {
        //     error!("Failed to parse entity {} into a known type", entity.name);
        //     c.entity(eid).insert(EntityIndexed);
        // }
    }
}
