use shared::{message::Frame, FrameNumber};

pub struct Frames {
    pub starts_at: FrameNumber,
    pub frames: Vec<Frame>,
    pub last_confirmed: FrameNumber,
}

impl Frames {
    pub fn new(starts_at: FrameNumber) -> Self {
        assert!(!(starts_at < 1), "Can't have starts_at smaller than 1");

        Self {
            starts_at,
            frames: Vec::new(),
            last_confirmed: 0,
        }
    }

    pub fn reset(&mut self) {
        self.frames = Vec::new();
        self.last_confirmed = 0;
    }

    pub fn initialize_frames_untill(&mut self, number: FrameNumber) {
        let frame_idx = (number - self.starts_at) as usize;

        let mut i = self.frames.len();

        while i <= frame_idx {
            self.frames
                .push(Frame::new(i as FrameNumber + self.starts_at));

            i += 1;
        }
    }

    pub fn get_mut(&mut self, number: FrameNumber) -> &mut Frame {
        let frame_idx = (number - self.starts_at) as usize;
        self.frames.get_mut(frame_idx).unwrap()
    }

    #[allow(dead_code)]
    pub fn get(&mut self, number: FrameNumber) -> &Frame {
        let frame_idx = (number - self.starts_at) as usize;
        self.frames.get(frame_idx).unwrap()
    }

    pub fn get_confirmed_frames_between(
        &self,
        start: Option<FrameNumber>,
        end: Option<FrameNumber>,
    ) -> &[Frame] {
        let start = start.unwrap_or(self.starts_at);
        let end = end.unwrap_or(self.last_confirmed);

        let start_index = (start - self.starts_at) as usize;
        let end_index = (end - self.starts_at) as usize;

        &self.frames[start_index..=end_index]
    }
}
