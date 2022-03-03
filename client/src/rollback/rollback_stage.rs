#![allow(dead_code)]
use bevy::{prelude::*, reflect::TypeRegistry};
use bevy_networking_turbulence::NetworkResource;
use instant::{Duration, Instant};
use shared::{
    message::{
        ClientMessage, Frame, FrameChecksum, FrameInput, Information, PlayerInput, TickInput,
    },
    FrameNumber, PlayerHandle,
};

use crate::resource::{FrameInfo, RemoteFrames, RenderInfo};

use super::{world_snapshot::WorldSnapshot, RollbackDiagnostics};

pub struct StopRollbackStage;

pub struct StartRollbackStage {
    frame: FrameNumber,
}

impl StartRollbackStage {
    pub fn new(frame: FrameNumber) -> Self {
        Self { frame }
    }
}

pub const DEFAULT_UPDATE_FREQUENCY: u32 = 30;

/// The RollbackStage handles updating, saving and loading the game state.
pub(crate) struct RollbackStage {
    /// Inside this schedule, all rollback systems are registered.
    schedule: Schedule,
    every: Schedule,
    /// Used to register all types considered when loading and saving
    pub(crate) type_registry: TypeRegistry,
    /// This system is used to get an encoded representation of the input that Rollback can handle
    pub(crate) input_system: Option<Box<dyn System<In = PlayerHandle, Out = PlayerInput>>>,
    /// We save the world here, avoiding encoding into `Vec<u8>`.
    snapshot: Option<WorldSnapshot>,
    snapshot_frame: FrameNumber,
    /// fixed FPS our logic is running with
    update_frequency: u32,
    /// counts the number of frames that have been executed
    /// 0 = initial state, no frames executed
    /// 1..n = frame numbers
    last_confirmed_frame: FrameNumber,
    local_frame: FrameNumber,
    target_frame: FrameNumber,
    /// internal time control variables
    last_update: Instant,
    /// accumulated time. once enough time has been accumulated, an update is executed
    accumulator: Duration,
    /// variable to change how fast or slow game is advanced
    /// 1.0 = normal
    /// 1.1 = slow
    /// 0.9 = fast
    run_speed: f64,
    /// boolean to see if stage is running
    running: bool,

    local_frames: Vec<Frame>,

    input_lag: u32,
    send_checksum: bool,
    first_run: bool,
}

impl Stage for RollbackStage {
    fn run(&mut self, world: &mut World) {
        if world.remove_resource::<StopRollbackStage>().is_some() {
            self.running = false;
        }
        if !self.running {
            // Loading -system will add StartRollbackStage -resource
            // which will cause this stage to start running
            if let Some(start_rollback_stage) = world.remove_resource::<StartRollbackStage>() {
                self.reset_session();
                self.last_confirmed_frame = 1;
                self.target_frame = start_rollback_stage.frame;
                self.running = true;
                self.first_run = true;
            } else {
                return;
            }
        } else {
            self.first_run = false;
        }

        let information_res = world
            .get_resource::<Information>()
            .expect("Information exists");

        let player_handle = information_res.player_handle;

        let mut remote_frames = world
            .get_resource_mut::<RemoteFrames>()
            .expect("RemoteFrames exists");

        let remote_frame_diff = remote_frames.remote_frame_diff;

        // tmp_frames are unordered, so needs to be sorted
        remote_frames.tmp_frames.sort_by_key(|f| f.number);

        // get last confirmed frame number
        let mut prev_frame_number = if let Some(frame) = remote_frames.frames.last() {
            frame.number
        } else {
            0
        };

        // keep only frames that are newer than previous confirmed frame
        remote_frames
            .tmp_frames
            .retain(|frame| frame.number > prev_frame_number);

        // get next tmp_frame that can be confirmed
        let mut next_tmp_frame_number = if let Some(frame) = remote_frames.tmp_frames.first() {
            frame.number
        } else {
            0
        };

        // loop while are sequenced tmp frames have been added to confirmed frames
        while next_tmp_frame_number == prev_frame_number + 1 {
            let frame = remote_frames.tmp_frames.remove(0);
            prev_frame_number = frame.number;

            remote_frames.frames.push(frame);

            next_tmp_frame_number = if let Some(frame) = remote_frames.tmp_frames.first() {
                frame.number
            } else {
                0
            };
        }

        // Advance all confirmed frames
        self.advance_confirmed_frames(world);

        // Change speed depending frame diff with the server
        if remote_frame_diff > 3 {
            self.run_speed = 0.2; // Speed up
        } else if remote_frame_diff > -1 {
            self.run_speed = 0.9; // Speed up
        } else if remote_frame_diff < -3 {
            self.run_speed = 1.1; // Slow down
        } else {
            self.run_speed = 1.; // Keep it steady
        }

        // get delta time from last run() call and accumulate it
        let delta = Instant::now().duration_since(self.last_update);
        let mut fps_delta = 1. / self.update_frequency as f64;
        fps_delta *= self.run_speed;
        self.accumulator = self.accumulator.saturating_add(delta);
        self.last_update = Instant::now();

        // if we accumulated enough time, do steps
        while self.accumulator.as_secs_f64() > fps_delta {
            // decrease accumulator
            self.accumulator = self
                .accumulator
                .saturating_sub(Duration::from_secs_f64(fps_delta));

            // Take the inputs for current frame
            let input = self
                .input_system
                .as_mut()
                .expect("No input system found. Please use AppBuilder::with_input_sampler_system.")
                .run(player_handle, world);

            // Add input to local_frames
            let mut frame = Frame::new(self.target_frame);
            frame.inputs.push(FrameInput {
                input,
                player_handle,
            });
            self.local_frames.push(frame);

            let mut client_res = world
                .get_resource_mut::<NetworkResource>()
                .expect("NetworkResource exists");

            let server_handle = *client_res.connections.keys().last().unwrap();

            // Send input to the server
            client_res
                .send_message(
                    server_handle,
                    TickInput {
                        frame_number: self.target_frame,
                        player_input: input,
                        last_confirmed_frame: self.last_confirmed_frame,
                    },
                )
                .expect("should be able to send handshake");

            if self.send_checksum {
                if let Some(snapshot) = &self.snapshot {
                    // let type_registry = self.type_registry.read();
                    // let str = snapshot.serialize(&type_registry);
                    // println!("{}", str);
                    client_res
                        .send_message(
                            server_handle,
                            ClientMessage::VerifyFrameChecksum(FrameChecksum {
                                checksum: snapshot.checksum,
                                number: self.snapshot_frame,
                            }),
                        )
                        .expect("should be able to send handshake");

                    info!(
                        "checksum sent, frame: {}, cheksum: {}",
                        self.snapshot_frame, snapshot.checksum
                    );
                    self.send_checksum = false;
                }
            }

            self.target_frame += 1;
        }

        // Only keep new local_frames
        let last_confirmed_frame = self.last_confirmed_frame;
        self.local_frames
            .retain(|frame| frame.number >= last_confirmed_frame);

        // Calculate next local frame
        // input_lag should be allowed to be changed by the user
        // this way user can find the best balance between input lag and other player lag
        let mut next_local_frame = self.target_frame - self.input_lag;
        if next_local_frame < self.last_confirmed_frame {
            next_local_frame = self.last_confirmed_frame;
        }

        // Process until local_frame is at target_frame
        while self.local_frame < next_local_frame {
            // Find the next frame or just use empty one
            let frame = if let Some(frame) = self
                .local_frames
                .iter()
                .find(|f| f.number == self.local_frame)
            {
                frame.clone()
            } else {
                Frame::new(self.local_frame)
            };

            // Execute the next frame
            world.insert_resource(frame);
            world.insert_resource(FrameInfo::new(false, true));
            self.schedule.run_once(world);
            world.remove_resource::<FrameInfo>();
            world.remove_resource::<Frame>();

            self.local_frame += 1;
        }

        let delta = self.accumulator.as_secs_f32() / fps_delta as f32;
        world.insert_resource(RenderInfo { delta });
        self.every.run_once(world);
        world.remove_resource::<RenderInfo>();

        let mut rollback_diagnostics = world
            .get_resource_mut::<RollbackDiagnostics>()
            .expect("Rollback exists");

        rollback_diagnostics.last_confirmed_frame = self.last_confirmed_frame;
        rollback_diagnostics.local_frame = self.local_frame;
        rollback_diagnostics.target_frame = self.target_frame;
        rollback_diagnostics.snapshot_frame = self.snapshot_frame;
        rollback_diagnostics.input_lag = self.input_lag;
        rollback_diagnostics.run_speed = self.run_speed;
        rollback_diagnostics.update_frequency = self.update_frequency;
        rollback_diagnostics.remote_frame_diff = remote_frame_diff;
    }
}

impl RollbackStage {
    pub(crate) fn new(input_lag: u32) -> Self {
        Self {
            schedule: Schedule::default(),
            every: Schedule::default(),
            type_registry: TypeRegistry::default(),
            input_system: None,
            snapshot_frame: 0,
            snapshot: None,
            last_confirmed_frame: 0,
            target_frame: 1,
            local_frame: 0,
            update_frequency: DEFAULT_UPDATE_FREQUENCY,
            last_update: Instant::now(),
            accumulator: Duration::ZERO,
            run_speed: 1.,
            running: false,
            local_frames: Vec::new(),
            input_lag,
            send_checksum: false,
            first_run: true,
        }
    }

    // Reverts to latest snapshot and executes all new confirmed frames
    fn advance_confirmed_frames(&mut self, world: &mut World) {
        let remote_frames = world
            .get_resource::<RemoteFrames>()
            .expect("RemoteFrames resource");

        // Get all new confirmed frames, return if none
        let next_frame_index = (self.last_confirmed_frame - remote_frames.starts_at) as usize;
        let new_confirmed_frames = &remote_frames.frames[next_frame_index..];
        if new_confirmed_frames.is_empty() {
            return;
        }

        let frames = new_confirmed_frames.to_vec();

        // Revert to previous snapshot
        if let Some(snapshot) = &self.snapshot {
            self.last_confirmed_frame = self.snapshot_frame;
            snapshot.write_to_world(world, &self.type_registry);
        }

        // Execute all new confirmed frames
        for frame in frames {
            world.insert_resource(frame);
            world.insert_resource(FrameInfo::new(true, self.first_run));
            self.schedule.run_once(world);
            world.remove_resource::<FrameInfo>();
            world.remove_resource::<Frame>();
            self.last_confirmed_frame += 1;
        }

        // Take a new snapshot
        self.snapshot = Some(WorldSnapshot::from_world(world, &self.type_registry));
        self.snapshot_frame = self.last_confirmed_frame;
        self.local_frame = self.last_confirmed_frame;

        // Only send checksum once per second
        if self.snapshot_frame % self.update_frequency == 0 {
            self.send_checksum = true;
        }
    }

    pub(crate) fn reset_session(&mut self) {
        self.last_update = Instant::now();
        self.accumulator = Duration::ZERO;
        self.last_confirmed_frame = 0;
        self.local_frame = 0;
        self.target_frame = 1;
        self.snapshot = None;
        self.snapshot_frame = 0;
        self.run_speed = 1.;
        self.local_frames = Vec::new();
        self.send_checksum = false;
        self.first_run = true;
    }

    pub(crate) fn set_update_frequency(&mut self, update_frequency: u32) {
        self.update_frequency = update_frequency
    }

    pub(crate) fn set_schedule(&mut self, schedule: Schedule) {
        self.schedule = schedule;
    }

    pub(crate) fn set_every(&mut self, schedule: Schedule) {
        self.every = schedule;
    }
}
