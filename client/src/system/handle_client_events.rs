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
                ServerMessage::LoadFrames { frames, is_last } => {
                    for frame in frames {
                        remote_frames.tmp_frames.push(frame);
                    }
                    remote_frames.loading_done = is_last;
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
                remote_frames.tmp_frames.push(frame);
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
