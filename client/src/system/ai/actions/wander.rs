use bevy::prelude::{Local, Query, Res, With};
use rand::Rng;
use shared::message::{Frame, Information};

use crate::{
    component::{self, Collider, Enemy, NetworkEntity},
    resource::Random,
};

pub fn wander(
    mut collider_q: Query<
        (&mut Collider, &NetworkEntity, &Enemy),
        With<component::ai::actions::Wander>,
    >,
    mut random: Local<Random>,
    information: Res<Information>,
    frame: Res<Frame>,
) {
    random.reseed(information.seed + frame.number as u64);

    let mut colliders = collider_q.iter_mut().collect::<Vec<_>>();
    colliders.sort_by_key(|e| e.1.id);

    for c in colliders.iter_mut() {
        let n = random.get_mut().gen_range(0..10);

        if n == 0 {
            c.0.vel.x = c.2.x_speed;
        } else if n == 1 {
            c.0.vel.x = -c.2.x_speed;
        }

        if n == 3 {
            c.0.vel.y = c.2.y_speed;
        } else if n == 4 {
            c.0.vel.y = -c.2.y_speed;
        }
    }
}
