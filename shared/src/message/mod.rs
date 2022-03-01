use bevy::{prelude::Component, reflect::Reflect};
use serde::{Deserialize, Serialize};

use crate::{FrameNumber, PlayerHandle};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct FrameChecksum {
    pub checksum: f64,
    pub number: FrameNumber,
}

// FrameInput contains information about which player pressed what inputs
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FrameInput {
    pub input: PlayerInput,
    pub player_handle: PlayerHandle,
}

// Frame is what contains all the required information for each game frame
// Server and every client needs to execute these in same order and same way in order for game to
// stay deterministic
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Frame {
    pub number: FrameNumber,
    pub inputs: Vec<FrameInput>,
    pub joined_players: Vec<PlayerHandle>,
    pub leaved_players: Vec<PlayerHandle>,
}

impl Frame {
    pub fn new(number: FrameNumber) -> Self {
        Self {
            number,
            ..Frame::default()
        }
    }
}

// PlayerInput is used as part of FrameInput but also can be used as bevy component to assign for
// each player's entity
#[derive(Default, Component, Reflect, Serialize, Deserialize, Debug, Copy, Clone)]
pub struct PlayerInput {
    pub mouse_clicked: bool,
    pub mouse_x: i16,
    pub mouse_y: i16,
}

// ClientMessage is enum that contains all the possible message variants to be sent to the server
// from the client
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientMessage {
    Handshake,
    // TickInput {
    //     frame_number: FrameNumber,
    //     player_input: PlayerInput,
    //     last_confirmed_frame: FrameNumber,
    // },
    VerifyFrameChecksum(FrameChecksum),
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct TickInput {
    pub frame_number: FrameNumber,
    pub player_input: PlayerInput,
    pub last_confirmed_frame: FrameNumber,
}

// ClientState contains all the possible states for the client, server can command client to switch
// the state
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ClientState {
    MainMenu,
    Connecting,
    Loading,
    InGame,
}

// Information is sent from the server to the client after connection is established
// it contains information about the ongoing game.
// Clients will add this as bevy resource, so it can be read in systems to determine for example
// the client's own player_handle.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Information {
    pub player_handle: PlayerHandle,
    pub seed: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FrameInfo {
    pub frames: Vec<Frame>,
    pub frame_diff: i8,
}

// ServerMessage is enum that contains all the possible message variants to be sent to the client
// from the server
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerMessage {
    HandshakeAccepted,
    ChangeState(ClientState),
    // FrameInfo { frames: Vec<Frame>, frame_diff: i8 },
    LoadFrames { is_last: bool, frames: Vec<Frame> },
    InitialInformation(Information),
}
