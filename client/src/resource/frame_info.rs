#[derive(Default, Debug)]
pub struct FrameInfo {
    pub confirmed: bool,
    pub disable_sound: bool,
}

impl FrameInfo {
    pub fn new(confirmed: bool, disable_sound: bool) -> Self {
        FrameInfo {
            confirmed,
            disable_sound,
        }
    }
}
