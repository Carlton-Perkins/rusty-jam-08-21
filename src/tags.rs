use crate::entity_class::enemy::EnemyState;
use bevy_inspector_egui::Inspectable;
use bevy::prelude::Transform;

pub struct Player;

#[derive(Inspectable, Debug)]
pub struct Enemy {
    pub state: EnemyState,
    pub start_loc: Transform,
    pub move_mod: i8,
}
impl Default for Enemy {
    fn default() -> Self {
        Enemy {
            state: EnemyState::Idle,
            ..Default::default()
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum WorldType {
    Air,
    Wall,
    Door,    //Todo replace with Entity
    Nothing, // Stub
}

pub fn world_type_from_str(s: &str) -> Option<WorldType> {
    match s {
        "Wall" => Some(WorldType::Wall),
        "Floor" => Some(WorldType::Air),
        _ => None,
    }
}
