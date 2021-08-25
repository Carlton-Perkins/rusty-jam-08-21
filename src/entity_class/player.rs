use crate::tags::Player;
use crate::{GameLayer, MainCamera};
use bevy::prelude::*;
use heron::{CollisionLayers, CollisionShape, RigidBody, RotationConstraints, Velocity};

pub fn spawn_player(
    mut c: &mut Commands,
    assets: &Res<AssetServer>,
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
    transform: &Transform,
) {
    let player_sprite = assets.load("player.sprite.png");

    // Spawn camera
    c.spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera)
        .id();

    c.spawn()
        .insert_bundle(SpriteBundle {
            material: materials.add(player_sprite.into()),
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
        )
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
