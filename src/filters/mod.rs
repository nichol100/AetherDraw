// Non-destructive filter layers: blur, sharpen, noise, hue/sat, etc.
// Phase 1: enum + trait only.

#[derive(Clone, Copy, Debug)]
pub enum FilterType {
    Blur,
    Sharpen,
    Noise,
    HueSaturation,
    BrightnessContrast,
    Curves,
    Levels,
    ColorBalance,
    GradientMap,
    Posterize,
    Halftone,
    Outline,
}

pub trait Filter {
    fn apply(&self, input: &[u8], width: u32, height: u32) -> Vec<u8>;
}