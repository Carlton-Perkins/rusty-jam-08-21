use bevy::prelude::Transform;
use bevy_inspector_egui::Inspectable;

pub struct Player;

pub struct MainCamera;

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
