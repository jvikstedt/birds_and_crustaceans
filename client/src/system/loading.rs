use bevy::prelude::{Commands, ResMut, State};
use bevy_networking_turbulence::NetworkResource;
use shared::message::{ClientState, PlayerInput, TickInput};

use crate::{resource::RemoteFrames, rollback::rollback_stage::StartRollbackStage};

pub fn loading(
    mut commands: Commands,
    mut app_state: ResMut<State<ClientState>>,
    mut remote_frames: ResMut<RemoteFrames>,
    mut client_res: ResMut<NetworkResource>,
) {
    if remote_frames.loading_done {
        let loading_end = remote_frames.loading_end;
        remote_frames.tmp_frames.retain(|f| f.number <= loading_end);
    }
    // Sort tmp_frames
    remote_frames.tmp_frames.sort_by_key(|f| f.number);

    // Get the previous confirmed frame number
    let mut prev_frame_number = if let Some(frame) = remote_frames.frames.last() {
        frame.number
    } else {
        0
    };

    // Only keep tmp frames that are newer than the previous confirmed frame number
    remote_frames
        .tmp_frames
        .retain(|frame| frame.number > prev_frame_number);

    // Get the frame number of next tmp_frame
    let mut next_tmp_frame_number = if let Some(frame) = remote_frames.tmp_frames.first() {
        frame.number
    } else {
        0
    };

    // Loop tmp_frames as long as the first tmp_frame is the next frame
    // Client can receive frames in wrong order and it needs to wait to receive frames in correct
    // order
    while next_tmp_frame_number == prev_frame_number + 1 {
        let frame = remote_frames.tmp_frames.remove(0);
        prev_frame_number = frame.number;

        // Add to confirmed frames
        remote_frames.frames.push(frame);

        next_tmp_frame_number = if let Some(frame) = remote_frames.tmp_frames.first() {
            frame.number
        } else {
            0
        };
    }

    // If loading is done and also tmp_frames is empty, it means we are done processing frames
    // means that we were able to process all the frames
    if remote_frames.loading_done && remote_frames.tmp_frames.is_empty() {
        let mut prev_number = 0;

        // Verify that all the frames are in correct order
        for frame in &remote_frames.frames {
            assert_eq!(
                prev_number + 1,
                frame.number,
                "frames at wrong order after loading"
            );
            prev_number = frame.number;
        }

        // Change state and add StartRollbackStage resource to inform RollbackStage that it can
        // start the game loop
        app_state.set(ClientState::InGame).unwrap();
        commands.insert_resource(StartRollbackStage::new(prev_number));
    }

    let server_handle = *client_res.connections.keys().last().unwrap();

    let prev_frame_number = if let Some(frame) = remote_frames.frames.last() {
        frame.number
    } else {
        0
    };

    client_res
        .send_message(
            server_handle,
            TickInput {
                frame_number: 0,
                player_input: PlayerInput::default(),
                last_confirmed_frame: prev_frame_number,
            },
        )
        .expect("should be able to send handshake");
}
