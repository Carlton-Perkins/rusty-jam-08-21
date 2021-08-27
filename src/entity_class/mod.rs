use crate::GameStage;
use bevy::prelude::*;

mod door;
pub(crate) mod enemy;
mod indexer;
mod player;
mod start_location;
mod patrol_path;

pub use player::Player;

pub struct EntityClasses;

impl Plugin for EntityClasses {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(indexer::index_entities.system())
            .add_system(start_location::spawn_from_spawn_location.system())
            .add_stage(GameStage, SystemStage::parallel())
            .add_system_to_stage(GameStage, player::player_movement.system())
            .add_system_to_stage(GameStage, player::animate_player.system());
    }
}
