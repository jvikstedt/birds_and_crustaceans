use bevy::{
    math::Vec2,
    prelude::{Entity, EventWriter, Query, Without},
};

use crate::{
    component::{Collider, Dynamic, EntityType, Fixed, NetworkEntity, Solid},
    event::CollisionEvent,
};

// TODO
// - Add broadphase to only check collisions that are close to dynamics
// - Only execute CCD (substeps) for fast moving objects? Not sure about this one

fn aabb_intersect(
    source_pos: &Vec2,
    source_size: &Vec2,
    target_pos: &Vec2,
    target_size: &Vec2,
) -> bool {
    (source_pos.x - source_size.x / 2.) < (target_pos.x + target_size.x / 2.)
        && (source_pos.x + source_size.x / 2.) > (target_pos.x - target_size.x / 2.)
        && (source_pos.y - source_size.y / 2.) < (target_pos.y + target_size.y / 2.)
        && (source_pos.y + source_size.y / 2.) > (target_pos.y - target_size.y / 2.)
}

pub fn check_collision(
    mut collision_ev: EventWriter<CollisionEvent>,
    dynamic_q: Query<(Entity, &NetworkEntity, &Dynamic), Without<Fixed>>,
    mut collider_q: Query<(
        Entity,
        &NetworkEntity,
        &mut Collider,
        Option<&Solid>,
        &EntityType,
    )>,
) {
    // Sort dynamics by NetworkEntity for deterministic behaviour
    let mut dynamics = dynamic_q.iter().collect::<Vec<_>>();
    dynamics.sort_by_key(|e| e.1.id);

    // Sort colliders by NetworkEntity for deterministic behaviour
    let mut colliders = collider_q.iter().collect::<Vec<_>>();
    colliders.sort_by_key(|e| e.1.id);

    let substep_count = 1;
    let delta_time = 1. / (substep_count as f32);

    let mut collisions: Vec<(Entity, Entity)> = Vec::new();

    for _ in 0..substep_count {
        // Test dynamics collision
        for (d_entity, _, _) in dynamics.iter() {
            let (_, _, mut d_collider, _, d_entity_type) = collider_q.get_mut(*d_entity).unwrap();

            let mut x_vel = d_collider.vel.x * delta_time;

            // MOVE-X
            if x_vel != 0. {
                for (c_entity, _, c_collider, solid, c_entity_type) in colliders.iter() {
                    if d_entity == c_entity {
                        continue;
                    }

                    // Skip non-solid entities
                    if solid.is_none() {
                        continue;
                    }
                    if aabb_intersect(
                        &Vec2::new(d_collider.pos.x + x_vel, d_collider.pos.y),
                        &d_collider.size,
                        &Vec2::new(c_collider.pos.x, c_collider.pos.y),
                        &c_collider.size,
                    ) {
                        if x_vel > 0. {
                            // Moving right
                            x_vel = (d_collider.pos.x
                                - (c_collider.pos.x
                                    - c_collider.size.x / 2.
                                    - d_collider.size.x / 2.))
                                .abs();
                        } else if x_vel < 0. {
                            // Moving left
                            x_vel = -(d_collider.pos.x
                                - (c_collider.pos.x
                                    + c_collider.size.x / 2.
                                    + d_collider.size.x / 2.));
                        }

                        if !collisions
                            .iter()
                            .any(|(e1, e2)| e1 == d_entity && e2 == c_entity)
                        {
                            collision_ev.send(CollisionEvent {
                                source: *d_entity,
                                source_type: *d_entity_type,
                                target: *c_entity,
                                target_type: **c_entity_type,
                                contact_normal: d_collider.vel.normalize(),
                                is_solid: true,
                            });

                            collisions.push((*d_entity, *c_entity));
                        }

                        // No point to continue checking if no more x-velocity
                        if x_vel == 0. {
                            break;
                        }
                    }
                }
            }

            let mut y_vel = d_collider.vel.y * delta_time;

            // MOVE-Y
            if y_vel != 0. {
                for (c_entity, _, c_collider, solid, c_entity_type) in colliders.iter() {
                    if d_entity == c_entity {
                        continue;
                    }

                    // Skip non-solid entities
                    if solid.is_none() {
                        continue;
                    }
                    if aabb_intersect(
                        &Vec2::new(d_collider.pos.x + x_vel, d_collider.pos.y + y_vel),
                        &d_collider.size,
                        &Vec2::new(c_collider.pos.x, c_collider.pos.y),
                        &c_collider.size,
                    ) {
                        if y_vel > 0. {
                            // Moving up
                            y_vel = (d_collider.pos.y
                                - (c_collider.pos.y
                                    - c_collider.size.y / 2.
                                    - d_collider.size.y / 2.))
                                .abs();
                        } else if y_vel < 0. {
                            // Moving down
                            y_vel = -(d_collider.pos.y
                                - (c_collider.pos.y
                                    + c_collider.size.y / 2.
                                    + d_collider.size.y / 2.));
                        }
                        if !collisions
                            .iter()
                            .any(|(e1, e2)| e1 == d_entity && e2 == c_entity)
                        {
                            collision_ev.send(CollisionEvent {
                                source: *d_entity,
                                source_type: *d_entity_type,
                                target: *c_entity,
                                target_type: **c_entity_type,
                                contact_normal: d_collider.vel.normalize(),
                                is_solid: true,
                            });

                            collisions.push((*d_entity, *c_entity));
                        }

                        // No point to continue checking if no more y-velocity
                        if y_vel == 0. {
                            break;
                        }
                    }
                }
            }

            // Round small velocities to zero to prevent weird behaviour
            if x_vel.abs() < 0.01 {
                x_vel = 0.;
            }
            if y_vel.abs() < 0.01 {
                y_vel = 0.;
            }

            d_collider.pos += Vec2::new(x_vel, y_vel);
            d_collider.vel.x = x_vel;
            d_collider.vel.y = y_vel;
        }

        // Check other collisions
        for (d_entity, _, _) in dynamics.iter() {
            let (_, _, d_collider, _, d_entity_type) = collider_q.get(*d_entity).unwrap();

            for (c_entity, _, c_collider, solid, c_entity_type) in colliders.iter() {
                if d_entity == c_entity {
                    continue;
                }

                // Skip solid entities
                if solid.is_some() {
                    continue;
                }

                if aabb_intersect(
                    &Vec2::new(d_collider.pos.x, d_collider.pos.y),
                    &d_collider.size,
                    &Vec2::new(c_collider.pos.x, c_collider.pos.y),
                    &c_collider.size,
                ) && !collisions
                    .iter()
                    .any(|(e1, e2)| e1 == d_entity && e2 == c_entity)
                {
                    collision_ev.send(CollisionEvent {
                        source: *d_entity,
                        source_type: *d_entity_type,
                        target: *c_entity,
                        target_type: **c_entity_type,
                        contact_normal: Vec2::new(0., 0.),
                        is_solid: false,
                    });

                    collisions.push((*d_entity, *c_entity));
                }
            }
        }
    }
}
