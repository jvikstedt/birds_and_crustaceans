use bevy::{app::Events, prelude::ResMut};

use crate::event::CollisionEvent;

pub fn clear_events(mut collision_ev: ResMut<Events<CollisionEvent>>) {
    collision_ev.clear();
}
