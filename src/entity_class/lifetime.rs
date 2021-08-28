use bevy::prelude::*;

pub struct Lifetime {
    pub lifetime: Timer,
}

pub fn apply_lifetime(mut c: Commands, time: Res<Time>, mut q: Query<(Entity, &mut Lifetime)>) {
    for (e_id, mut lifetime) in q.iter_mut() {
        if lifetime.lifetime.tick(time.delta()).just_finished() {
            c.entity(e_id).despawn();
        }
    }
}
