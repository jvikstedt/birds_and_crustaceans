use bevy::prelude::{error, info, warn, EventReader, ResMut};
use bevy_networking_turbulence::{ConnectionHandle, NetworkEvent, NetworkResource};
use shared::message::{
    ClientMessage, ClientState, FrameInput, Information, ServerMessage, TickInput,
};

use crate::resource::{Frames, PlayerHandleProvider, PlayerInfo, Players};

pub fn handle_server_events(
    mut net: ResMut<NetworkResource>,
    mut reader: EventReader<NetworkEvent>,
    mut frames: ResMut<Frames>,
    mut players: ResMut<Players>,
    mut php: ResMut<PlayerHandleProvider>,
) {
    let mut server_messages_to_send: Vec<(ConnectionHandle, ServerMessage)> = Vec::new();
    for (handle, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();
        while let Some(tick_input) = channels.recv::<TickInput>() {
            // Get PlayerInfo, if it's not found, it means that either player left or
            // player ServerEvent::Connected event never arrived
            let player_info = match players.0.get_mut(handle) {
                Some(player_info) => player_info,
                None => {
                    warn!("WARNING: could not find player info {:?}", handle);
                    continue;
                }
            };

            // Update player info
            if tick_input.last_confirmed_frame > player_info.last_confirmed_frame {
                player_info.last_confirmed_frame = tick_input.last_confirmed_frame;
            }
            player_info
                .frame_diff
                .add(frames.last_confirmed as i32 - tick_input.frame_number as i32);

            // If received input frame is old, just skip it
            // in future it could make sense to apply this to next possible frame
            if tick_input.frame_number <= frames.last_confirmed {
                warn!("WARNING: received old input {:?}", tick_input.frame_number);
                continue;
            }

            // Add new player input
            frames.initialize_frames_untill(tick_input.frame_number);
            let frame = frames.get_mut(tick_input.frame_number);
            frame.inputs.push(FrameInput {
                input: tick_input.player_input,
                player_handle: player_info.handle,
            });
        }

        while let Some(client_message) = channels.recv::<ClientMessage>() {
            match client_message {
                ClientMessage::Handshake => {
                    server_messages_to_send.push((*handle, ServerMessage::HandshakeAccepted));
                }
                ClientMessage::VerifyFrameChecksum(frame_checksum) => {
                    let player_info = match players.0.get_mut(handle) {
                        Some(player_info) => player_info,
                        None => {
                            warn!("WARNING: could not find player info {:?}", handle);
                            continue;
                        }
                    };

                    player_info
                        .checksums
                        .insert(frame_checksum.number, frame_checksum.checksum);

                    let sender_handle = player_info.handle;

                    for (_, player_info) in players.0.iter() {
                        if sender_handle == player_info.handle {
                            continue;
                        }
                        if let Some(checksum) = player_info.checksums.get(&frame_checksum.number) {
                            if *checksum != frame_checksum.checksum {
                                warn!(
                                    "CHECKSUM-MISMATCH: FRAME: {}, CHECKSUMS: {} != {} ({} | {:?})",
                                    frame_checksum.number,
                                    checksum,
                                    frame_checksum.checksum,
                                    player_info.handle,
                                    handle
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    for (handle, server_message) in server_messages_to_send {
        net.send_message(handle, server_message)
            .expect("should be able to send data");
    }

    for evt in reader.iter() {
        match evt {
            NetworkEvent::Connected(handle) => {
                // Get the next possible id for player
                let player_handle = php.next_id();
                info!("connected {:?} with handle {:?}", handle, player_handle);

                // Command client to change state to loading
                net.send_message(*handle, ServerMessage::ChangeState(ClientState::Loading))
                    .expect("should be able to change state to loading event");

                // Add new Connection -> PlayerInfo mapping
                players.0.insert(
                    *handle,
                    PlayerInfo {
                        handle: player_handle,
                        loading: true,
                        ..Default::default()
                    },
                );

                // Add joined player to next possible frame
                let next_frame = frames.last_confirmed + 1;
                frames.initialize_frames_untill(next_frame);
                let frame = frames.get_mut(next_frame);
                frame.joined_players.push(player_handle);

                // Send initial information to the client
                net.send_message(
                    *handle,
                    ServerMessage::InitialInformation(Information {
                        seed: 1,
                        player_handle,
                    }),
                )
                .expect("should be able to send initial information");
            }
            NetworkEvent::Disconnected(handle) => {
                // Remove left client, if client has disconnected there is no longer need to send
                // anything to that client
                if let Some(player_handle) = players.0.remove(handle) {
                    info!("disconnected {:?} with handle: {:?}", handle, player_handle);

                    // Add leave player to next possible frame
                    let next_frame = frames.last_confirmed + 1;
                    frames.initialize_frames_untill(next_frame);
                    let frame = frames.get_mut(next_frame);
                    frame.leaved_players.push(player_handle.handle);
                }
            }
            NetworkEvent::Error(handle, err) => {
                error!("ERROR: {} {:?}", handle, err);
            }
            _ => {}
        }
    }
}
