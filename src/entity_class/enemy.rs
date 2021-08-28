use crate::entity_class::patrol_path::PatrolPath;

use crate::entity_class::creature::Creature;
use crate::entity_class::player::{LastMovementDirection, MovementDirection};
use crate::GameLayer;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use heron::{CollisionLayers, CollisionShape, RigidBody, RotationConstraints, Velocity};
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;

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

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum EnemyFunctions {
    Move,
    ChangeState,
}

#[derive(Inspectable, Debug)]
pub enum EnemyState {
    Idle,
    Patrol,
    Attack,
    Wander,
}

impl Distribution<EnemyState> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EnemyState {
        use EnemyState::*;
        match rng.gen_range(0..4) {
            0 => Idle,
            1 => Patrol,
            _ => Wander,
        }
    }
}

pub fn spawn_enemy(
    mut c: &mut Commands,
    assets: &Res<AssetServer>,
    mut texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    transform: &Transform,
) {
    let enemy_sprite = assets.load("enemy.spritemap.png");
    let texture_atlas = TextureAtlas::from_grid(enemy_sprite, Vec2::new(64., 64.), 4, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let start_loc = Transform::from_xyz(transform.translation.x, transform.translation.y, 100.);

    c.spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: start_loc,
            ..Default::default()
        })
        .insert(GameLayer::Enemy)
        .insert(Enemy {
            state: EnemyState::Idle,
            start_loc,
            move_mod: -1,
        })
        .insert(Creature)
        .insert(Velocity::from_linear(Vec3::default()))
        .insert(RigidBody::Dynamic)
        .insert(RotationConstraints::lock())
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(20., 28., 0.),
            border_radius: None,
        })
        .insert(LastMovementDirection(MovementDirection::Down))
        .insert(
            CollisionLayers::none()
                .with_group(GameLayer::Enemy)
                .with_masks(&[GameLayer::World, GameLayer::Player, GameLayer::Projectile]),
        );
}

// rand_update_enemy_state iterates over all enemies on the board and
// randomly determines if the enemy state should change from {Idle} to {Patrol}
// and vice versa
pub fn rand_update_enemy_state(mut enemies: Query<&mut Enemy>) {
    let mut rng = rand::thread_rng();

    for mut enemy in enemies.iter_mut() {
        match &enemy.state {
            EnemyState::Idle => {
                let chance_to_change = rng.gen_range(0..33);
                if chance_to_change == 0 {
                    enemy.state = rng.gen();
                }
            }
            EnemyState::Patrol => {
                let chance_to_change = rng.gen_range(0..66);
                if chance_to_change == 0 {
                    enemy.state = EnemyState::Idle;
                }
            }
            EnemyState::Wander => {
                let chance_to_change = rng.gen_range(0..66);
                if chance_to_change == 0 {
                    enemy.state = EnemyState::Idle;
                }
            }
            _ => {}
        };
    }
}

pub fn move_down(mut q: Query<(&mut Velocity, &mut Enemy, &Transform), With<Enemy>>) {
    let mut rng = rand::thread_rng();

    for (mut real_vel, mut enemy, transform) in q.iter_mut() {
        let mut vel = real_vel.clone();
        let move_speed = 10.;
        let min_speed = 0.01;
        let max_speed = 100.;
        let friction = 0.95;

        match &enemy.state {
            EnemyState::Patrol => {
                if transform.translation.y < enemy.start_loc.translation.y - 200. {
                    enemy.move_mod = 1
                } else if transform.translation.y >= enemy.start_loc.translation.y {
                    enemy.move_mod = -1
                }

                vel.linear.y += move_speed * enemy.move_mod as f32;
            }
            EnemyState::Wander => {
                vel.linear += Vec3::new(
                    rng.gen_range(-move_speed..move_speed),
                    rng.gen_range(-move_speed..move_speed),
                    0.,
                )
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

pub fn find_nearest_patrol_path(mut q: Query<(&mut PatrolPath, &Transform)>) {
    for (mut path, location) in q.iter_mut() {}
}
