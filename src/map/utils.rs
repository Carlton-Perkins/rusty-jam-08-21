use bevy::prelude::*;

// LDtk 0,0 is the top left, moving -x,-y
// Bevy 0,0 is the center of the screen
// Need to offset and flip y
pub fn convert_to_world(
    _width: f32,
    _height: f32,
    _grid_size: i32,
    _scale: f32,
    x: i32,
    y: i32,
    z: i32,
) -> Vec3 {
    let world_x = x as f32;
    let world_y = -y as f32;
    let world_z = z as f32;
    // info!(
    //     "Spawning tile at {:?}",
    //     Vec3::new(world_x, world_y, world_z)
    // );
    Vec3::new(world_x, world_y, world_z)
}
