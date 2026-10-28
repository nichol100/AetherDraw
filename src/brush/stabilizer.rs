// Stroke stabilizer: smooths noisy stylus input using a moving average.

use glam::Vec2;

pub struct StrokeStabilizer {
    pub strength: f32,
    history: Vec<Vec2>,
}

impl StrokeStabilizer {
    pub fn new(strength: f32) -> Self {
        Self {
            strength,
            history: Vec::new(),
        }
    }

    /// Feed a raw point, get a smoothed point back.
    pub fn smooth(&mut self, p: Vec2) -> Vec2 {
        self.history.push(p);
        if self.history.len() > 8 {
            self.history.remove(0);
        }
        let n = self.history.len() as f32;
        let avg = self.history.iter().copied().fold(Vec2::ZERO, |a, b| a + b) / n;
        // Lerp between raw and averaged based on strength.
        p.lerp(avg, self.strength.clamp(0.0, 1.0))
    }

    pub fn reset(&mut self) {
        self.history.clear();
    }
}