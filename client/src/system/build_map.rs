use bevy::{
    math::Vec2,
    prelude::{Camera, Color, Commands, Query, Res, ResMut, SpriteBundle, Transform, With},
    render::camera::CameraPlugin,
    sprite::Sprite,
    window::Windows,
};

use crate::{
    component::{Collider, EntityType, Fixed, NetworkEntity, Solid},
    resource::NetworkIdProvider,
};

pub fn build_map(
    mut commands: Commands,
    mut nip: ResMut<NetworkIdProvider>,
    wnds: Res<Windows>,
    q_camera: Query<&Camera, With<Camera>>,
) {
    let camera = q_camera
        .iter()
        .find(|c| match &c.name {
            Some(name) => name == &CameraPlugin::CAMERA_2D.to_string(),
            None => false,
        })
        .unwrap();

    let wnd = wnds.get(camera.window).unwrap();

    let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

    // Add walls
    // Bottom wall
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 1.0),
                custom_size: Some(Vec2::new(window_size.x, 30.)),
                ..Default::default()
            },

            transform: Transform {
                translation: [0., -(window_size.y / 2.), 0.].into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider {
            size: Vec2::new(window_size.x, 30.),
            pos: Vec2::new(0., -(window_size.y / 2.)),
            vel: Vec2::default(),
        })
        .insert(Solid::default())
        .insert(EntityType::Wall)
        .insert(Fixed::default())
        .insert(NetworkEntity { id: nip.next_id() });

    // top wall
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 1.0),
                custom_size: Some(Vec2::new(window_size.x, 30.)),
                ..Default::default()
            },

            transform: Transform {
                translation: [0., window_size.y / 2., 0.].into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider {
            size: Vec2::new(window_size.x, 30.),
            pos: Vec2::new(0., window_size.y / 2.),
            vel: Vec2::default(),
        })
        .insert(Solid::default())
        .insert(EntityType::Wall)
        .insert(Fixed::default())
        .insert(NetworkEntity { id: nip.next_id() });

    // left wall
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 1.0),
                custom_size: Some(Vec2::new(30., window_size.y)),
                ..Default::default()
            },

            transform: Transform {
                translation: [-(window_size.x / 2.), 0., 0.].into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider {
            size: Vec2::new(30., window_size.y),
            pos: Vec2::new(-(window_size.x / 2.), 0.),
            vel: Vec2::default(),
        })
        .insert(Solid::default())
        .insert(EntityType::Wall)
        .insert(Fixed::default())
        .insert(NetworkEntity { id: nip.next_id() });

    // right wall
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 1.0),
                custom_size: Some(Vec2::new(30., window_size.y)),
                ..Default::default()
            },

            transform: Transform {
                translation: [window_size.x / 2., 0., 0.].into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider {
            size: Vec2::new(30., window_size.y),
            pos: Vec2::new(window_size.x / 2., 0.),
            vel: Vec2::default(),
        })
        .insert(Solid::default())
        .insert(EntityType::Wall)
        .insert(Fixed::default())
        .insert(NetworkEntity { id: nip.next_id() });
}
