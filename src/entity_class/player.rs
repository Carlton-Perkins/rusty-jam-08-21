use crate::{GameLayer, MainCamera};
use bevy::prelude::*;
use heron::{CollisionLayers, CollisionShape, RigidBody, RotationConstraints, Velocity};
use std::fs::DirBuilder;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Player;

pub struct LastMovementDirection(Direction);

pub fn spawn_player(
    mut c: &mut Commands,
    assets: &Res<AssetServer>,
    mut texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    transform: &Transform,
) {
    let player_spritesheet = assets.load("player.spritemap.png");
    let texture_atlas = TextureAtlas::from_grid(player_spritesheet, Vec2::new(64., 64.), 3, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Spawn camera
    c.spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera)
        .id();

    c.spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_xyz(transform.translation.x, transform.translation.y, 100.),
            ..Default::default()
        })
        .insert(Player)
        .insert(Velocity::from_linear(Vec3::default()))
        .insert(RigidBody::Dynamic)
        .insert(RotationConstraints::lock())
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(20., 28., 0.),
            border_radius: None,
        })
        // .insert(TesselatedCollider {
        //     image: player_sprite,
        //     tesselator_config: TesselatedColliderConfig {
        //         vertice_separation: 0.,
        //         ..Default::default()
        //     },
        //     ..Default::default()
        // })
        .insert(
            CollisionLayers::none()
                .with_group(GameLayer::Player)
                .with_masks(&[GameLayer::World, GameLayer::Enemy]),
        ).insert(LastMovementDirection(Direction::Down))
    // .insert_bundle(ShapeBundle {
    //     shape: Shape::Circle {
    //         center: Default::default(),
    //         radius: 150.,
    //         fill: Color32::TRANSPARENT,
    //         stroke: Stroke::new(1., Color32::RED),
    //     },
    //     transform: Transform::from_xyz(0., 0., 1.),
    //     ..Default::default()
    // });
    // .insert_children(0, &[camera])
    ;
}

pub fn player_movement(
    input: Res<Input<KeyCode>>,
    mut q: Query<(&mut Velocity, &mut LastMovementDirection), With<Player>>,
) {
    let move_speed = 10.;
    let min_speed = 0.01;
    let max_speed = 100.;
    let friction = 0.95;
    for (mut real_vel, mut dir) in q.iter_mut() {
        let mut vel = real_vel.clone();

        // Adjust current velocity
        if input.pressed(KeyCode::W) {
            vel.linear.y += move_speed
        };
        if input.pressed(KeyCode::A) {
            vel.linear.x -= move_speed
        };
        if input.pressed(KeyCode::S) {
            vel.linear.y -= move_speed
        };
        if input.pressed(KeyCode::D) {
            vel.linear.x += move_speed
        };

        // Clamp velocity to MAX
        vel.linear
            .clamp(Vec3::splat(-max_speed), Vec3::splat(max_speed));

        // Apply velocity degradation due to friction
        vel.linear *= friction;

        // Zero out velocities lower then 0.00001

        if vel.linear.x.abs() <= min_speed {
            vel.linear.x = 0.
        }
        if vel.linear.y.abs() <= min_speed {
            vel.linear.y = 0.
        }

        // Only update if different
        // if real_vel.linear.abs_diff_eq(vel.linear, min_speed) {
        real_vel.linear = vel.linear;
        //     info!(
        //         "Updating player velocity From {:?} to {:?}",
        //         real_vel.linear, vel.linear
        //     );
        // }

        // Update last direction
        let normalized = vel.linear.normalize();
        let absed = normalized.abs();
        let norm_x = absed.x;
        let norm_y = absed.y;

        // Find primary direction
        let new_dir = match (norm_x, norm_y) {
            (0., 0.) => Direction::Down,
            (x, y) if x > y => {
                if normalized.x > 0. {
                    Direction::Right
                } else {
                    Direction::Left
                }
            }
            (x, y) if y > x => Direction::Down,
            _ => Direction::Down,
        };

        if dir.0 != new_dir {
            info!(
                "Changed player dir to {:?} with vel {:?}",
                new_dir, normalized
            );
            dir.0 = new_dir
        }
    }
}

pub fn animate_player(
    mut q: Query<
        (&LastMovementDirection, &mut TextureAtlasSprite),
        (With<Player>, Changed<LastMovementDirection>),
    >,
) {
    fn map_direction_to_sprite(d: Direction) -> u32 {
        match d {
            Direction::Up => 0,
            Direction::Down => 0,
            Direction::Left => 2,
            Direction::Right => 1,
        }
    }

    for (dir, mut sprite) in q.iter_mut() {
        sprite.index = map_direction_to_sprite(dir.0);
    }
}
