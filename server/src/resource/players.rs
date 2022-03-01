use std::collections::HashMap;

use bevy_networking_turbulence::ConnectionHandle;
use shared::{FrameDiff, FrameNumber, PlayerHandle};

#[derive(Default, Debug, Clone)]
pub struct PlayerInfo {
    pub handle: PlayerHandle,
    pub loading: bool,
    pub last_sent: FrameNumber,
    pub frame_diff: FrameDiff,
    pub last_confirmed_frame: FrameNumber,
    pub checksums: HashMap<FrameNumber, f64>,
}

#[derive(Default, Debug, Clone)]
pub struct Players(pub HashMap<ConnectionHandle, PlayerInfo>);
