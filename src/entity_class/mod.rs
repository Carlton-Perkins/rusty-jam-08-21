use crate::entity_class::health::Damaged;
use crate::GameStage;
use bevy::core::FixedTimestep;
use bevy::prelude::*;

mod creature;
mod door;
mod enemy;
mod gold;
mod health;
mod indexer;
mod lifetime;
mod movement;
mod patrol_path;
mod player;
mod projectile;
mod start_location;

pub struct EntityClasses;

impl Plugin for EntityClasses {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(indexer::index_entities.system())
            .add_system(start_location::spawn_from_spawn_location.system())
            .add_stage(GameStage, SystemStage::parallel())
            .add_system_to_stage(GameStage, player::player_movement.system())
            .add_system_to_stage(GameStage, player::move_camera_with_player.system())
            .add_system_to_stage(GameStage, movement::animate_creature.system())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.05))
                    .with_system(
                        enemy::rand_update_enemy_state
                            .system()
                            .label(enemy::EnemyFunctions::ChangeState),
                    )
                    .with_system(
                        enemy::move_down
                            .system()
                            .label(enemy::EnemyFunctions::Move)
                            .after(enemy::EnemyFunctions::ChangeState),
                    ),
            )
            .add_system(movement::update_last_direction.system())
            .add_system(health::insert_health_to_creatures.system())
            .add_system(health::on_damage_drain_health.system())
            .add_system(health::despawn_if_zero_health.system())
            .add_event::<Damaged>()
            .add_system(projectile::on_collide_apply_damage.system())
            .add_system(projectile::on_collide_despawn.system())
            .add_system(projectile::cast_projectile.system())
            .add_system(lifetime::apply_lifetime.system())
            .add_system(gold::setup_gold.system());
    }
}
