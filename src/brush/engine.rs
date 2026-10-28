// Custom brush engine.
//
// Supports: size, opacity, flow, hardness, spacing, angle, jitter, scatter,
// texture, smoothing, pressure/tilt/velocity sensitivity.

use glam::Vec2;

#[derive(Clone, Copy, Debug)]
pub enum BrushType {
    Pencil,
    Pen,
    Ink,
    Marker,
    Airbrush,
    Watercolor,
    Oil,
    Charcoal,
    Pastel,
    TextureStamp,
    Pattern,
    Scatter,
}

#[derive(Clone, Debug)]
pub struct BrushSettings {
    pub brush_type: BrushType,
    pub size: f32,
    pub opacity: f32,
    pub flow: f32,
    pub hardness: f32,
    pub spacing: f32,
    pub angle: f32,
    pub jitter: f32,
    pub scatter: f32,
    pub smoothing: f32,
    // Sensitivity curves (0..1).
    pub pressure_size: f32,
    pub pressure_opacity: f32,
    pub tilt_angle: bool,
    pub velocity_size: bool,
}

impl Default for BrushSettings {
    fn default() -> Self {
        Self {
            brush_type: BrushType::Pen,
            size: 8.0,
            opacity: 1.0,
            flow: 1.0,
            hardness: 0.8,
            spacing: 0.25,
            angle: 0.0,
            jitter: 0.0,
            scatter: 0.0,
            smoothing: 0.3,
            pressure_size: 1.0,
            pressure_opacity: 0.5,
            tilt_angle: false,
            velocity_size: false,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct StrokePoint {
    pub pos: Vec2,
    pub pressure: f32,
    pub tilt: Vec2,
    pub velocity: f32,
    pub time: f64,
}

pub struct BrushEngine {
    pub settings: BrushSettings,
    pub current_stroke: Vec<StrokePoint>,
    pub last_stamp_pos: Option<Vec2>,
}

impl BrushEngine {
    pub fn new(settings: BrushSettings) -> Self {
        Self {
            settings,
            current_stroke: Vec::new(),
            last_stamp_pos: None,
        }
    }

    pub fn begin_stroke(&mut self, point: StrokePoint) {
        self.current_stroke.clear();
        self.current_stroke.push(point);
        self.last_stamp_pos = Some(point.pos);
    }

    pub fn add_point(&mut self, point: StrokePoint) {
        self.current_stroke.push(point);
        // Stamping happens in the renderer; here we just record.
    }

    pub fn end_stroke(&mut self) {
        self.current_stroke.clear();
        self.last_stamp_pos = None;
    }

    /// Distance (in canvas units) between stamps based on size + spacing.
    pub fn stamp_spacing(&self) -> f32 {
        (self.settings.size * self.settings.spacing).max(1.0)
    }
}

// Pressure curve helper.
pub fn apply_pressure(value: f32, pressure: f32, sensitivity: f32) -> f32 {
    value * (1.0 - sensitivity + sensitivity * pressure.clamp(0.0, 1.0))
}