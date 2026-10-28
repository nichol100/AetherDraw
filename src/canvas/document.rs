// Document: the whole canvas with its layer stack.

use crate::canvas::compositor;
use crate::canvas::layer::{Layer, RasterLayer};
use egui::{Color32, Ui};

#[derive(Clone, Debug)]
pub struct Document {
    pub width: u32,
    pub height: u32,
    pub layers: Vec<Layer>,
    pub active_layer: usize,
}

impl Document {
    pub fn new(width: u32, height: u32) -> Self {
        // Start with one raster layer named "Layer 1".
        let layer = Layer::Raster(RasterLayer::new("Layer 1", width, height));
        Self {
            width,
            height,
            layers: vec![layer],
            active_layer: 0,
        }
    }

    /// Paint a small dot on the active raster layer (demo brush).
    pub fn paint_dot(&mut self, pos: glam::Vec2, radius: f32, color: Color32) {
        let idx = self.active_layer;
        if idx >= self.layers.len() {
            return;
        }
        if let Layer::Raster(layer) = &mut self.layers[idx] {
            let r = radius as i32;
            let cx = pos.x as i32;
            let cy = pos.y as i32;
            for dy in -r..=r {
                for dx in -r..=r {
                    if dx * dx + dy * dy > r * r {
                        continue;
                    }
                    let x = cx + dx;
                    let y = cy + dy;
                    if x < 0 || y < 0 {
                        continue;
                    }
                    let (x, y) = (x as u32, y as u32);
                    if x >= layer.width || y >= layer.height {
                        continue;
                    }
                    layer.set_pixel(x, y, [color.r(), color.g(), color.b(), color.a()]);
                }
            }
        }
    }

    /// Flatten all layers to a single RGBA buffer (used for display/export).
    pub fn composite(&self) -> Vec<u8> {
        compositor::composite(&self.layers, self.width, self.height)
    }

    /// Draw the layer list in the UI (with visibility toggles).
    pub fn draw_layer_list(&mut self, ui: &mut Ui) {
        // "Add layer" button.
        if ui.button("+ New Raster Layer").clicked() {
            let n = self.layers.len() + 1;
            self.layers
                .push(Layer::Raster(RasterLayer::new(format!("Layer {}", n), self.width, self.height)));
        }

        ui.separator();

        // List layers top-to-bottom (reverse order).
        let mut to_toggle: Option<usize> = None;
        let mut to_delete: Option<usize> = None;

        for (i, layer) in self.layers.iter().enumerate().rev() {
            ui.horizontal(|ui| {
                let visible = layer.visible();
                if ui
                    .selectable_label(i == self.active_layer, layer.name())
                    .clicked()
                {
                    self.active_layer = i;
                }
                // Visibility checkbox.
                let mut v = visible;
                if ui.checkbox(&mut v, "").changed() {
                    to_toggle = Some(i);
                }
                if v != visible {
                    // handled via to_toggle
                }
            });
        }

        if let Some(i) = to_toggle {
            if let Some(layer) = self.layers.get_mut(i) {
                let new_vis = !layer.visible();
                layer.set_visible(new_vis);
            }
        }

        // Delete button for active layer (if more than one).
        if self.layers.len() > 1 {
            if ui.button("Delete Active Layer").clicked() {
                to_delete = Some(self.active_layer);
            }
        }
        if let Some(i) = to_delete {
            self.layers.remove(i);
            if self.active_layer >= self.layers.len() {
                self.active_layer = self.layers.len().saturating_sub(1);
            }
        }
    }
}