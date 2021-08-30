use crate::entity_class::creature::Creature;
use bevy::prelude::*;
use heron::Velocity;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum MovementDirection {
    Up,
    Down,
    Left,
    Right,
}

pub struct LastMovementDirection(pub MovementDirection);

pub fn animate_creature(
    mut q: Query<
        (&LastMovementDirection, &mut TextureAtlasSprite),
        (With<Creature>, Changed<LastMovementDirection>),
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
        sprite.index = map_direction_to_sprite(dir.0);
    }
}

pub fn update_last_direction(
    mut q: Query<(&Velocity, &mut LastMovementDirection), Changed<Velocity>>,
) {
    for (vel, mut dir) in q.iter_mut() {
        // Update last direction
        let normalized = vel.linear.normalize();
        let absed = normalized.abs();
        let norm_x = absed.x;
        let norm_y = absed.y;

        // Find primary direction
        let new_dir = match (norm_x, norm_y) {
            (0., 0.) => MovementDirection::Down,
            (x, y) if x > y => {
                if normalized.x > 0. {
                    MovementDirection::Right
                } else {
                    MovementDirection::Left
                }
            }
            (x, y) if y > x => {
                if normalized.y > 0. {
                    MovementDirection::Up
                } else {
                    MovementDirection::Down
                }
            }
            _ => dir.0,
        };

        if dir.0 != new_dir {
            dir.0 = new_dir
        }
    }
}
