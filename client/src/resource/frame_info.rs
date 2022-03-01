#[derive(Default, Debug)]
pub struct FrameInfo {
    pub confirmed: bool,
}

impl FrameInfo {
    pub fn new(confirmed: bool) -> Self {
        FrameInfo { confirmed }
    }
}
