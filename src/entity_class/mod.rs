use bevy::prelude::*;

mod indexer;
mod start_location;

pub struct EntityClasses;

impl Plugin for EntityClasses {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(indexer::index_entities.system())
            .add_system(start_location::spawn_from_spawn_location.system());
    }
}