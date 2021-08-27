use crate::entity_class::player::LastMovementDirection;
use crate::entity_class::player::MovementDirection;
use crate::tags::Player;
use bevy::prelude::*;

pub fn animate_creature(
    mut q: Query<
        (&LastMovementDirection, &mut TextureAtlasSprite),
        (With<Player>, Changed<LastMovementDirection>),
    >,
) {
    fn map_direction_to_sprite(d: MovementDirection) -> u32 {
        match d {
            MovementDirection::Up => 3,
            MovementDirection::Down => 0,
            MovementDirection::Left => 2,
            MovementDirection::Right => 1,
        }
    }

    for (dir, mut sprite) in q.iter_mut() {
        info!("Updating sprite due to movement {:?}", dir.0);
        sprite.index = map_direction_to_sprite(dir.0);
    }
}
