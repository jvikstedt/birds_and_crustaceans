#![allow(dead_code)]

use bevy::prelude::*;
use bevy_networking_turbulence::{ConnectionHandle, NetworkResource};
use instant::{Duration, Instant};
use shared::{
    message::{FrameInfo, ServerMessage},
    FrameNumber,
};

use crate::resource::{Frames, Players};

/// Stage label for the Custom Server Stage.
pub const SERVER_UPDATE: &str = "server_update";

pub trait ServerApp {
    fn with_server_schedule(&mut self, schedule: Schedule) -> &mut Self;

    /// Sets the fixed update frequency
    fn with_update_frequency(&mut self, update_frequency: u32) -> &mut Self;
}

impl ServerApp for App {
    fn with_server_schedule(&mut self, schedule: Schedule) -> &mut Self {
        let server_stage = self
            .schedule
            .get_stage_mut::<ServerStage>(&SERVER_UPDATE)
            .expect("No ServerStage found! Did you install the ServerPlugin?");
        server_stage.set_schedule(schedule);
        self
    }

    fn with_update_frequency(&mut self, update_frequency: u32) -> &mut Self {
        let server_stage = self
            .schedule
            .get_stage_mut::<ServerStage>(&SERVER_UPDATE)
            .expect("No ServerStage found! Did you install the ServerPlugin?");
        server_stage.set_update_frequency(update_frequency);
        self
    }
}

/// The RollbackStage handles updating, saving and loading the game state.
pub(crate) struct ServerStage {
    /// Inside this schedule, all rollback systems are registered.
    schedule: Schedule,
    /// fixed FPS our logic is running with
    update_frequency: u32,
    current_frame: FrameNumber,
    /// internal time control variables
    last_update: Instant,
    /// accumulated time. once enough time has been accumulated, an update is executed
    accumulator: Duration,
    /// boolean to see if we should run slow to let remote clients catch up
    run_slow: bool,
}

impl Stage for ServerStage {
    fn run(&mut self, world: &mut World) {
        // get delta time from last run() call and accumulate it
        let delta = Instant::now().duration_since(self.last_update);
        let mut fps_delta = 1. / self.update_frequency as f64;
        if self.run_slow {
            fps_delta *= 1.1;
        }
        self.accumulator = self.accumulator.saturating_add(delta);
        self.last_update = Instant::now();

        // if we accumulated enough time, do steps
        while self.accumulator.as_secs_f64() > fps_delta {
            // decrease accumulator
            self.accumulator = self
                .accumulator
                .saturating_sub(Duration::from_secs_f64(fps_delta));

            // run scheduled systems if any
            self.schedule.run_once(world);

            // Update frames last_confirmed to next one
            let mut frames = world.get_resource_mut::<Frames>().expect("Frames exists");
            frames.last_confirmed = self.current_frame;
            frames.initialize_frames_untill(self.current_frame);

            let frames = world.get_resource::<Frames>().expect("Frames exists");

            // Players to get PlayerInfo -> FrameDiffCounter for each connection
            let players = world.get_resource::<Players>().expect("Players exists");

            // ServerResource for getting connections and sending data
            let server_res = world
                .get_resource::<NetworkResource>()
                .expect("ServerResource exists");

            let mut frame_info_to_send: Vec<(ConnectionHandle, FrameInfo)> = Vec::new();
            let mut load_frames_to_send: Vec<(ConnectionHandle, ServerMessage, FrameNumber, bool)> =
                Vec::new();

            for handle in server_res.connections.keys() {
                if let Some(player_info) = players.0.get(handle) {
                    let frame_diff = player_info.frame_diff.average_i8();

                    if player_info.loading {
                        let end_frame = if (player_info.last_sent + 10) > frames.last_confirmed {
                            frames.last_confirmed
                        } else {
                            player_info.last_sent + 10
                        };
                        let is_last = frames.last_confirmed == end_frame;

                        let frames = frames.get_confirmed_frames_between(
                            if player_info.last_sent == 0 {
                                None
                            } else {
                                Some(player_info.last_sent)
                            },
                            Some(end_frame),
                        );
                        let load_frames = ServerMessage::LoadFrames {
                            frames: frames.to_vec(),
                            is_last,
                        };

                        load_frames_to_send.push((*handle, load_frames, end_frame, is_last));
                    } else {
                        let start_frame = if player_info.last_confirmed_frame == 0 {
                            frames.last_confirmed
                        } else {
                            player_info.last_confirmed_frame
                        };
                        let frames = frames.get_confirmed_frames_between(Some(start_frame), None);
                        let frame_info = FrameInfo {
                            frames: frames.to_vec(),
                            frame_diff,
                        };

                        frame_info_to_send.push((*handle, frame_info));
                    }
                }
            }

            let mut players = world.get_resource_mut::<Players>().expect("Players exists");
            for (handle, _, end_frame, is_last) in load_frames_to_send.iter() {
                if let Some(mut player_info) = players.0.get_mut(handle) {
                    player_info.loading = !is_last;
                    player_info.last_sent = *end_frame;
                }
            }

            let mut server_res = world
                .get_resource_mut::<NetworkResource>()
                .expect("ServerResource exists");

            for (handle, packet) in frame_info_to_send {
                server_res
                    .send_message(handle, packet)
                    .expect("should be able to send frame info");
            }

            for (handle, packet, _, _) in load_frames_to_send {
                server_res
                    .send_message(handle, packet)
                    .expect("should be able to send load frames");
            }

            // Move to next frame
            self.current_frame += 1;
        }
    }
}

impl ServerStage {
    pub(crate) fn new() -> Self {
        Self {
            schedule: Schedule::default(),
            current_frame: 1,
            update_frequency: 30,
            last_update: Instant::now(),
            accumulator: Duration::ZERO,
            run_slow: false,
        }
    }

    pub(crate) fn reset_session(&mut self) {
        self.last_update = Instant::now();
        self.accumulator = Duration::ZERO;
        self.current_frame = 1;
        self.run_slow = false;
    }

    pub(crate) fn set_update_frequency(&mut self, update_frequency: u32) {
        self.update_frequency = update_frequency
    }

    pub(crate) fn set_schedule(&mut self, schedule: Schedule) {
        self.schedule = schedule;
    }
}
