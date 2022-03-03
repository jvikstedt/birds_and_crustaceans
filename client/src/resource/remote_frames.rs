use shared::{message::Frame, FrameNumber};

pub struct RemoteFrames {
    pub starts_at: FrameNumber,
    pub frames: Vec<Frame>,
    pub tmp_frames: Vec<Frame>,
    pub loading_done: bool,
    pub loading_start: FrameNumber,
    pub loading_end: FrameNumber,
    pub remote_frame_diff: i8,
}

impl RemoteFrames {
    pub fn new(starts_at: FrameNumber) -> Self {
        Self {
            starts_at,
            frames: Vec::new(),
            tmp_frames: Vec::new(),
            loading_done: false,
            loading_start: 0,
            loading_end: 0,
            remote_frame_diff: 0,
        }
    }
}
