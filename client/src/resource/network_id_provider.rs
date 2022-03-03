use bevy::{prelude::Component, reflect::Reflect};

#[derive(Default, Component, Reflect)]
pub struct NetworkIdProvider {
    next_id: u32,
}

impl NetworkIdProvider {
    /// Returns an unused, unique id.
    pub fn next_id(&mut self) -> u32 {
        if self.next_id == u32::MAX {
            // TODO: do something smart?
            panic!("RollbackIdProvider: u32::MAX has been reached.");
        }
        let ret = self.next_id;
        self.next_id += 1;
        ret
    }

    pub fn reset(&mut self) {
        self.next_id = 0;
    }
}
