// Erasers: hard, soft, textured, stroke eraser, layer-aware, color eraser, smudge, blur.

#[derive(Clone, Copy, Debug)]
pub enum EraserType {
    Hard,
    Soft,
    Textured,
    Stroke, // removes a whole stroke in one tap
    Color,  // erases a specific color
    Smudge,
    Blur,
}

pub struct EraserSettings {
    pub eraser_type: EraserType,
    pub size: f32,
    pub opacity: f32,
    pub hardness: f32,
    /// If true, only erase on the current layer.
    pub current_layer_only: bool,
    /// If true, erase pixels matching a target color.
    pub target_color: Option<[u8; 4]>,
    pub tolerance: f32,
}

impl Default for EraserSettings {
    fn default() -> Self {
        Self {
            eraser_type: EraserType::Hard,
            size: 20.0,
            opacity: 1.0,
            hardness: 1.0,
            current_layer_only: true,
            target_color: None,
            tolerance: 0.1,
        }
    }
}