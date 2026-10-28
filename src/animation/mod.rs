// Animation timeline: frames, cels, keyframes, tweening, onion skinning.
// Scaffold only.

pub struct Timeline {
    pub fps: f32,
    pub current_frame: u32,
    pub total_frames: u32,
    pub looping: bool,
}

impl Default for Timeline {
    fn default() -> Self {
        Self {
            fps: 24.0,
            current_frame: 0,
            total_frames: 120,
            looping: true,
        }
    }
}

pub struct Keyframe {
    pub frame: u32,
    pub layer_id: uuid::Uuid,
    pub property: String,
    pub value: f32,
}