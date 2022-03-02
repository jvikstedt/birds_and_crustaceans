use bevy::{
    math::Vec2,
    prelude::{
        BuildChildren, Camera, Color, Commands, Query, Res, ResMut, SpriteBundle, Transform, With,
    },
    render::camera::CameraPlugin,
    sprite::Sprite,
    window::Windows,
};
use glam::Vec3;

use crate::{
    component::{
        Checksum, Collider, Dynamic, EntityType, Fixed, Health, HealthBar, NetworkEntity, Solid,
        Training,
    },
    resource::NetworkIdProvider,
    rollback::{Rollback, RollbackIdProvider},
};

pub fn build_map(
    mut commands: Commands,
    mut nip: ResMut<NetworkIdProvider>,
    mut rip: ResMut<RollbackIdProvider>,
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
                translation: [150., -(window_size.y / 2.), 0.].into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider {
            size: Vec2::new(window_size.x, 30.),
            pos: Vec2::new(150., -(window_size.y / 2.)),
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
                translation: [150., window_size.y / 2., 0.].into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider {
            size: Vec2::new(window_size.x, 30.),
            pos: Vec2::new(150., window_size.y / 2.),
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
                custom_size: Some(Vec2::new(15., window_size.y)),
                ..Default::default()
            },

            transform: Transform {
                translation: [-(window_size.x / 2.) + 150., 0., 0.].into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider {
            size: Vec2::new(15., window_size.y),
            pos: Vec2::new(-(window_size.x / 2.) + 150., 0.),
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

    let size_x = 20.;
    let size_y = 20.;

    // Spawn training area

    // DAMAGE
    let health_bar_id = commands
        .spawn()
        .insert(HealthBar::default())
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1., 0.0, 0.0),
                custom_size: Some(Vec2::new(20., 3.)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0., size_x / 2. + 10., 10.),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 1.0),
                custom_size: Some(Vec2::new(size_x, size_y)),
                ..Default::default()
            },

            transform: Transform {
                translation: [-(window_size.x / 2.) + 30., (window_size.y / 2.) - 60., 0.].into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider {
            size: Vec2::new(size_x, size_y),
            pos: Vec2::new(-(window_size.x / 2.) + 30., (window_size.y / 2.) - 60.),
            vel: Vec2::default(),
        })
        .insert(Dynamic::default())
        .insert(EntityType::Training)
        .insert(NetworkEntity { id: nip.next_id() })
        .insert(Rollback::new(rip.next_id()))
        .insert(Checksum::default())
        .insert(Health::new(500, 500))
        .insert(Training {
            area_increase: 0,
            damage_increase: 5,
        })
        .add_child(health_bar_id);

    let health_bar_id = commands
        .spawn()
        .insert(HealthBar::default())
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1., 0.0, 0.0),
                custom_size: Some(Vec2::new(20., 3.)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0., size_x / 2. + 10., 10.),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 1.0),
                custom_size: Some(Vec2::new(size_x, size_y)),
                ..Default::default()
            },

            transform: Transform {
                translation: [-(window_size.x / 2.) + 100., (window_size.y / 2.) - 60., 0.].into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider {
            size: Vec2::new(size_x, size_y),
            pos: Vec2::new(-(window_size.x / 2.) + 100., (window_size.y / 2.) - 60.),
            vel: Vec2::default(),
        })
        .insert(Dynamic::default())
        .insert(EntityType::Training)
        .insert(NetworkEntity { id: nip.next_id() })
        .insert(Rollback::new(rip.next_id()))
        .insert(Checksum::default())
        .insert(Health::new(500, 500))
        .insert(Training {
            area_increase: 0,
            damage_increase: 5,
        })
        .add_child(health_bar_id);

    // AREA

    let health_bar_id = commands
        .spawn()
        .insert(HealthBar::default())
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1., 0.0, 0.0),
                custom_size: Some(Vec2::new(20., 3.)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0., size_x / 2. + 10., 10.),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 1.0),
                custom_size: Some(Vec2::new(size_x, size_y)),
                ..Default::default()
            },

            transform: Transform {
                translation: [-(window_size.x / 2.) + 30., (window_size.y / 2.) - 130., 0.].into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider {
            size: Vec2::new(size_x, size_y),
            pos: Vec2::new(-(window_size.x / 2.) + 30., (window_size.y / 2.) - 130.),
            vel: Vec2::default(),
        })
        .insert(Dynamic::default())
        .insert(EntityType::Training)
        .insert(NetworkEntity { id: nip.next_id() })
        .insert(Rollback::new(rip.next_id()))
        .insert(Checksum::default())
        .insert(Health::new(500, 500))
        .insert(Training {
            area_increase: 3,
            damage_increase: 0,
        })
        .add_child(health_bar_id);

    let health_bar_id = commands
        .spawn()
        .insert(HealthBar::default())
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1., 0.0, 0.0),
                custom_size: Some(Vec2::new(20., 3.)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0., size_x / 2. + 10., 10.),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 1.0),
                custom_size: Some(Vec2::new(size_x, size_y)),
                ..Default::default()
            },

            transform: Transform {
                translation: [
                    -(window_size.x / 2.) + 100.,
                    (window_size.y / 2.) - 130.,
                    0.,
                ]
                .into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider {
            size: Vec2::new(size_x, size_y),
            pos: Vec2::new(-(window_size.x / 2.) + 100., (window_size.y / 2.) - 130.),
            vel: Vec2::default(),
        })
        .insert(Dynamic::default())
        .insert(EntityType::Training)
        .insert(NetworkEntity { id: nip.next_id() })
        .insert(Rollback::new(rip.next_id()))
        .insert(Checksum::default())
        .insert(Health::new(500, 500))
        .insert(Training {
            area_increase: 3,
            damage_increase: 0,
        })
        .add_child(health_bar_id);
}
