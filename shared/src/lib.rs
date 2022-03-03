pub mod frame_diff;
pub mod message;
pub mod system;

pub use frame_diff::*;

pub type PlayerHandle = u32;
pub type FrameNumber = u32;

pub const SERVER_PORT: u16 = 14191;

pub const GAME_LENGTH: FrameNumber = 30 * 120;
