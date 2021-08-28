use crate::entity_class::creature::Creature;
use bevy::prelude::*;

pub struct Health {
    max: i32,
    value: i32,
}

pub struct Damaged {
    pub(crate) damage: i32,
    pub(crate) entity: Entity,
}

impl Default for Health {
    fn default() -> Self {
        Health {
            max: 100,
            value: 100,
        }
    }
}

// Add health to all creatures
pub fn insert_health_to_creatures(
    mut c: Commands,
    q: Query<Entity, (With<Creature>, Without<Health>)>,
) {
    for eid in q.iter() {
        c.entity(eid).insert(Health::default());
    }
}

pub fn on_damage_drain_health(mut events: EventReader<Damaged>, mut q: Query<&mut Health>) {
    for event in events.iter() {
        if let Ok(mut health) = q.get_mut(event.entity) {
            health.value -= event.damage;
        }
    }
}

pub fn despawn_if_zero_health(mut c: Commands, q: Query<(Entity, &Health), Changed<Health>>) {
    for (eid, health) in q.iter() {
        if health.value <= 0 {
            c.entity(eid).despawn();
        }
    }
}
