// 2D viewport: zoom, pan, rotation, grid.

use egui::{Painter, Rect};
use glam::Vec2;

#[derive(Clone, Debug, Default)]
pub struct Viewport2D {
    pub zoom: f32,
    pub pan: Vec2,
    pub rotation: f32,
}

impl Viewport2D {
    pub fn new() -> Self {
        Self {
            zoom: 1.0,
            pan: Vec2::ZERO,
            rotation: 0.0,
        }
    }

    pub fn screen_to_canvas(&self, screen: egui::Pos2, rect: Rect) -> Vec2 {
        // Simple transform: center origin, scale by zoom, apply pan.
        let center = rect.center();
        let local = screen - center;
        (Vec2::new(local.x, local.y) / self.zoom) + self.pan
    }

    pub fn draw_grid(&self, painter: &Painter, rect: Rect) {
        // Draw a light grid so the canvas area is visible.
        let spacing = 64.0 * self.zoom.max(0.1);
        let color = egui::Color32::from_rgba_premultiplied(128, 128, 128, 40);
        let mut x = rect.left();
        while x < rect.right() {
            painter.line_segment(
                [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
                egui::Stroke::new(1.0, color),
            );
            x += spacing;
        }
        let mut y = rect.top();
        while y < rect.bottom() {
            painter.line_segment(
                [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
                egui::Stroke::new(1.0, color),
            );
            y += spacing;
        }
    }
}