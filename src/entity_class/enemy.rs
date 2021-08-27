use bevy::prelude::*;
use crate::{GameLayer};
use crate::tags::Enemy;
use heron::{Velocity, RigidBody, CollisionShape, RotationConstraints, CollisionLayers};
use rand::Rng;
use bevy_inspector_egui::Inspectable;
use crate::entity_class::patrol_path::PatrolPath;

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum EnemyFunctions {
    Move,
    ChangeState,
}

#[derive(Inspectable, Debug)]
pub enum EnemyState {
    Idle,
    Patrol,
    Attack
}

pub fn spawn_enemy(
    mut c: &mut Commands,
    assets: &Res<AssetServer>,
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
    transform: &Transform,
) {
    let enemy_sprite = assets.load("enemy.sprite.png");
    let start_loc = Transform::from_xyz(transform.translation.x, transform.translation.y, 100.);

    c.spawn()
        .insert_bundle(SpriteBundle {
            material: materials.add(enemy_sprite.into()),
            transform: start_loc,
            ..Default::default()
        })
        .insert(GameLayer::Enemy)
        .insert(Enemy {
            state: EnemyState::Idle,
            start_loc,
            move_mod: -1
        })
        .insert(Velocity::from_linear(Vec3::default()))
        .insert(RigidBody::Dynamic)
        .insert(RotationConstraints::lock())
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(20., 28., 0.),
            border_radius: None,
        })
        .insert(
            CollisionLayers::none()
                .with_group(GameLayer::Enemy)
                .with_masks(&[GameLayer::World, GameLayer::Player]),
        );
}

// rand_update_enemy_state iterates over all enemies on the board and
// randomly determines if the enemy state should change from {Idle} to {Patrol}
// and vice versa
pub fn rand_update_enemy_state(
    mut enemies: Query<&mut Enemy>,
) {
    let mut rng = rand::thread_rng();

    for mut enemy in enemies.iter_mut() {
        match &enemy.state {
            EnemyState::Idle => {
                let chance_to_change = rng.gen_range(0..33);
                if chance_to_change == 0 {
                    enemy.state = EnemyState::Patrol;
                }
            }
            EnemyState::Patrol => {
                let chance_to_change = rng.gen_range(0..66);
                if chance_to_change == 0 {
                    enemy.state = EnemyState::Idle;
                }
            }
            _ => {}
        };
    }
}

pub fn move_down(
    mut q: Query<(&mut Velocity, &mut Enemy, &Transform), With<Enemy>>
) {
    for (mut real_vel, mut enemy, transform) in q.iter_mut() {
        let mut vel = real_vel.clone();
        let move_speed = 10.;
        let min_speed = 0.01;
        let max_speed = 100.;
        let friction = 0.95;

        match &enemy.state {
            EnemyState::Patrol => {
                if transform.translation.y < enemy.start_loc.translation.y -200. {
                    enemy.move_mod = 1
                } else if transform.translation.y >= enemy.start_loc.translation.y {
                    enemy.move_mod = -1
                }

                vel.linear.y += move_speed * enemy.move_mod as f32;

            }
            _ => {}
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
    }
}

pub fn find_nearest_patrol_path(
    mut q: Query<(&mut PatrolPath, &Transform)>,
) {
    for (mut path, location) in q.iter_mut() {

    }
}