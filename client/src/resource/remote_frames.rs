use shared::{message::Frame, FrameNumber};

pub struct RemoteFrames {
    pub starts_at: FrameNumber,
    pub frames: Vec<Frame>,
    pub tmp_frames: Vec<Frame>,
    pub loading_done: bool,
    pub remote_frame_diff: i8,
}

impl RemoteFrames {
    pub fn new(starts_at: FrameNumber) -> Self {
        Self {
            starts_at,
            frames: Vec::new(),
            tmp_frames: Vec::new(),
            loading_done: false,
            remote_frame_diff: 0,
        }
    }
}
