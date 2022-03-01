#![allow(dead_code)]
use bevy::{
    ecs::system::Resource,
    prelude::*,
    reflect::{FromType, GetTypeRegistration},
};
use reflect_resource::ReflectResource;
use rollback_stage::RollbackStage;
use shared::{message::PlayerInput, FrameNumber, PlayerHandle};

pub(crate) mod reflect_resource;
pub(crate) mod rollback_stage;
pub(crate) mod world_snapshot;

/// Stage label for the Custom Rollback Stage.
pub const ROLLBACK_UPDATE: &str = "rollback_update";

/// Add this component to all entities you want to be loaded/saved on rollback.
/// The `id` has to be unique. Consider using the `RollbackIdProvider` resource.
#[derive(Component)]
pub struct Rollback {
    id: u32,
}

impl Rollback {
    /// Creates a new rollback tag with the given id.
    pub fn new(id: u32) -> Self {
        Self { id }
    }

    /// Returns the rollback id.
    pub const fn id(&self) -> u32 {
        self.id
    }
}

#[derive(Default)]
pub struct RollbackDiagnostics {
    pub last_confirmed_frame: FrameNumber,
    pub local_frame: FrameNumber,
    pub target_frame: FrameNumber,
    pub snapshot_frame: FrameNumber,
    pub input_lag: u32,
    pub run_speed: f64,
    pub update_frequency: u32,
    pub remote_frame_diff: i8,
}

/// Provides unique ids for your Rollback components.
/// When you add the Rollback Plugin, this should be available as a resource.
#[derive(Default)]
pub struct RollbackIdProvider {
    next_id: u32,
}

impl RollbackIdProvider {
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
}

pub struct RollbackPlugin {
    pub input_lag: u32,
}

impl RollbackPlugin {
    pub fn new(input_lag: u32) -> Self {
        Self { input_lag }
    }
}

impl Plugin for RollbackPlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_before(
            CoreStage::Update,
            ROLLBACK_UPDATE,
            RollbackStage::new(self.input_lag),
        );
        app.insert_resource(RollbackIdProvider::default());
        app.insert_resource(RollbackDiagnostics::default());
    }
}

/// Extension trait for the `App`.
pub trait RollbackApp {
    /// Adds a schedule into the RollbackStage that holds the game logic systems. This schedule should contain all
    /// systems you want to be executed during frame advances.
    fn with_rollback_schedule(&mut self, schedule: Schedule) -> &mut Self;
    fn with_every(&mut self, schedule: Schedule) -> &mut Self;

    /// Registers a given system as the input system. This system should provide encoded inputs for a given player.
    fn with_input_system<Params>(
        &mut self,
        input_system: impl IntoSystem<PlayerHandle, PlayerInput, Params>,
    ) -> &mut Self;

    /// Sets the fixed update frequency
    fn with_update_frequency(&mut self, update_frequency: u32) -> &mut Self;

    /// Registers a type of component for saving and loading during rollbacks.
    fn register_rollback_type<T>(&mut self) -> &mut Self
    where
        T: GetTypeRegistration + Reflect + Default + Component;

    // Inserts a resource in bevy with saving and loading during rollbacks.
    fn insert_rollback_resource<T>(&mut self, resource: T) -> &mut Self
    where
        T: GetTypeRegistration + Reflect + Default + Component + Resource;
}

impl RollbackApp for App {
    fn with_rollback_schedule(&mut self, schedule: Schedule) -> &mut Self {
        let rollback_stage = self
            .schedule
            .get_stage_mut::<RollbackStage>(&ROLLBACK_UPDATE)
            .expect("No RollbackStage found! Did you install the RollbackPlugin?");
        rollback_stage.set_schedule(schedule);
        self
    }

    fn with_every(&mut self, schedule: Schedule) -> &mut Self {
        let rollback_stage = self
            .schedule
            .get_stage_mut::<RollbackStage>(&ROLLBACK_UPDATE)
            .expect("No RollbackStage found! Did you install the RollbackPlugin?");
        rollback_stage.set_every(schedule);
        self
    }

    fn with_input_system<Params>(
        &mut self,
        input_system: impl IntoSystem<PlayerHandle, PlayerInput, Params>,
    ) -> &mut Self {
        let mut input_system = input_system.system();
        input_system.initialize(&mut self.world);
        let rollback_stage = self
            .schedule
            .get_stage_mut::<RollbackStage>(&ROLLBACK_UPDATE)
            .expect("No RollbackStage found! Did you install the RollbackPlugin?");
        rollback_stage.input_system = Some(Box::new(input_system));
        self
    }

    fn with_update_frequency(&mut self, update_frequency: u32) -> &mut Self {
        let rollback_stage = self
            .schedule
            .get_stage_mut::<RollbackStage>(&ROLLBACK_UPDATE)
            .expect("No RollbackStage found! Did you install the RollbackPlugin?");
        rollback_stage.set_update_frequency(update_frequency);
        self
    }

    fn register_rollback_type<T>(&mut self) -> &mut Self
    where
        T: GetTypeRegistration + Reflect + Default + Component,
    {
        let rollback_stage = self
            .schedule
            .get_stage_mut::<RollbackStage>(&ROLLBACK_UPDATE)
            .expect("No RollbackStage found! Did you install the RollbackPlugin?");

        let mut registry = rollback_stage.type_registry.write();

        registry.register::<T>();

        let registration = registry.get_mut(std::any::TypeId::of::<T>()).unwrap();
        registration.insert(<ReflectComponent as FromType<T>>::from_type());
        registration.insert(<ReflectResource as FromType<T>>::from_type());
        drop(registry);

        self
    }

    fn insert_rollback_resource<T>(&mut self, resource: T) -> &mut Self
    where
        T: GetTypeRegistration + Reflect + Default + Component + Resource,
    {
        self.insert_resource(resource).register_rollback_type::<T>()
    }
}
