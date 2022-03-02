use bevy::{
    math::{Vec2, Vec3},
    prelude::{
        AssetServer, BuildChildren, Camera, Color, Commands, Local, Query, Res, ResMut, Transform,
        With,
    },
    render::camera::CameraPlugin,
    sprite::{Sprite, SpriteBundle},
    window::Windows,
};
use rand::Rng;
use shared::message::{Frame, Information};

use crate::{
    component::{
        ai, Checksum, Collider, Dynamic, Enemy, EnemyType, EntityType, Health, HealthBar,
        NetworkEntity, Player, ScoreReward,
    },
    resource::{NetworkIdProvider, Random},
    rollback::{Rollback, RollbackIdProvider},
};

pub fn spawn_enemy(
    mut random: Local<Random>,
    information: Res<Information>,
    mut commands: Commands,
    mut nip: ResMut<NetworkIdProvider>,
    mut rip: ResMut<RollbackIdProvider>,
    asset_server: Res<AssetServer>,
    frame: Res<Frame>,
    wnds: Res<Windows>,
    q_camera: Query<&Camera, With<Camera>>,
    q_players: Query<&Player>,
) {
    random.reseed(information.seed + frame.number as u64);

    let player_count = q_players.iter().len();

    let spawn = random.get_mut().gen_ratio(10 * player_count as u32, 1000);

    if !spawn {
        return;
    }

    let camera = q_camera
        .iter()
        .find(|c| match &c.name {
            Some(name) => name == &CameraPlugin::CAMERA_2D.to_string(),
            None => false,
        })
        .unwrap();

    let wnd = wnds.get(camera.window).unwrap();

    let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

    let size_ratio: f32 = random.get_mut().gen_range(0.1..3.0);
    let health = (100. * size_ratio) as i32;
    let is_bird = random.get_mut().gen_ratio(1, 2);

    let size_x = 40. * size_ratio;
    let size_y = 26. * size_ratio;

    let x_pos: f32 = random
        .get_mut()
        .gen_range((-(window_size.x / 2.) + 60. + 100.)..(window_size.x / 2. - 60.0));
    let y_pos: f32 = random
        .get_mut()
        .gen_range((-(window_size.y / 2.) + 60.)..(window_size.y / 2. - 60.0));

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
            texture: asset_server.load(if is_bird {
                "bird.png"
            } else {
                "crustacean.png"
            }),
            sprite: Sprite {
                custom_size: Some(Vec2::new(size_x, size_y)),
                ..Default::default()
            },

            transform: Transform {
                translation: [x_pos, y_pos, 0.].into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider {
            size: Vec2::new(size_x, size_y),
            pos: Vec2::new(x_pos, y_pos),
            vel: Vec2::default(),
        })
        .insert(Dynamic::default())
        .insert(EntityType::Enemy)
        .insert(NetworkEntity { id: nip.next_id() })
        .insert(Rollback::new(rip.next_id()))
        .insert(Checksum::default())
        .insert(ai::Smart::default())
        .insert(ScoreReward {
            score: (10. * size_ratio) as i32,
        })
        .insert(Health::new(health, health))
        .insert(ai::actions::Wander)
        .insert(if is_bird {
            Enemy {
                enemy_type: EnemyType::Bird,
                x_speed: 3.,
                y_speed: 3.,
            }
        } else {
            Enemy {
                enemy_type: EnemyType::Crustacean,
                x_speed: 7.,
                y_speed: 0.,
            }
        })
        .add_child(health_bar_id);
}
