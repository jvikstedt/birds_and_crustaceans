use bevy::prelude::{error, info, warn, Commands, EventReader, ResMut, State};
use bevy_networking_turbulence::{NetworkEvent, NetworkResource};
use shared::message::{ClientMessage, ClientState, FrameInfo, ServerMessage};

use crate::resource::RemoteFrames;

pub fn handle_client_events(
    mut client_events: EventReader<NetworkEvent>,
    mut app_state: ResMut<State<ClientState>>,
    mut remote_frames: ResMut<RemoteFrames>,
    mut commands: Commands,
    mut net: ResMut<NetworkResource>,
) {
    for (_, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();
        while let Some(server_message) = channels.recv::<ServerMessage>() {
            match server_message {
                ServerMessage::ChangeState(state) => {
                    app_state.set(state).unwrap();
                }
                ServerMessage::LoadingStart {
                    start_frame,
                    end_frame,
                } => {
                    info!("loading start: {} - {}", start_frame, end_frame);
                    *remote_frames = RemoteFrames::new(start_frame);
                    app_state.set(ClientState::Loading).unwrap();
                }
                ServerMessage::LoadingEnd {
                    start_frame,
                    end_frame,
                } => {
                    info!("loading end: {} - {}", start_frame, end_frame);
                    remote_frames.loading_done = true;
                }
                ServerMessage::InitialInformation(information) => {
                    commands.insert_resource(information);
                }
                _ => {}
            }
        }

        while let Some(frame_info) = channels.recv::<FrameInfo>() {
            remote_frames.remote_frame_diff = frame_info.frame_diff;

            for frame in frame_info.frames {
                if !remote_frames
                    .tmp_frames
                    .iter()
                    .any(|f| f.number == frame.number)
                {
                    remote_frames.tmp_frames.push(frame);
                }
            }
        }
    }

    for evt in client_events.iter() {
        match evt {
            NetworkEvent::Connected(server_handle) => {
                info!("Connected to the server!");
                net.send_message(*server_handle, ClientMessage::Handshake)
                    .expect("should be able to send handshake");
            }
            NetworkEvent::Disconnected(_) => {
                warn!("Disconnected from the server!");
            }
            NetworkEvent::Error(_, err) => {
                error!("ERROR: {:?}", err);
            }
            _ => {}
        }
    }
}
