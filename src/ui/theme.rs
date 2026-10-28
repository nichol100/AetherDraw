// Theme system: design tokens saved as JSON.
//
// A theme is just a JSON file with colors/radii/spacing. We parse it into
// `ThemeTokens` and then convert those tokens into egui visuals.

use crate::platform::paths::AppPaths;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;

#[derive(Clone, Debug, Deserialize)]
pub struct ThemeTokens {
    pub name: String,
    pub colors: BTreeMap<String, String>,
    pub radii: BTreeMap<String, f32>,
    pub spacing: BTreeMap<String, f32>,
}

impl ThemeTokens {
    pub fn from_json(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }

    pub fn color(&self, key: &str) -> egui::Color32 {
        self.colors
            .get(key)
            .and_then(|h| parse_hex(h))
            .unwrap_or(egui::Color32::GRAY)
    }
}

fn parse_hex(hex: &str) -> Option<egui::Color32> {
    let h = hex.trim_start_matches('#');
    if h.len() == 6 {
        let r = u8::from_str_radix(&h[0..2], 16).ok()?;
        let g = u8::from_str_radix(&h[2..4], 16).ok()?;
        let b = u8::from_str_radix(&h[4..6], 16).ok()?;
        Some(egui::Color32::from_rgb(r, g, b))
    } else {
        None
    }
}

pub struct ThemeManager {
    themes: Vec<ThemeTokens>,
    active: usize,
    // Cached colors for quick access by the canvas.
    canvas_bg_cached: egui::Color32,
    text_cached: egui::Color32,
}

impl ThemeManager {
    pub fn new(paths: &AppPaths, ctx: &egui::Context) -> Self {
        // Try to load themes from <config>/themes first (user themes).
        let mut themes = Vec::new();
        let user_dir = paths.config_dir.join("themes");
        if user_dir.is_dir() {
            if let Ok(entries) = fs::read_dir(&user_dir) {
                for e in entries.flatten() {
                    if let Ok(s) = fs::read_to_string(e.path()) {
                        if let Ok(t) = ThemeTokens::from_json(&s) {
                            themes.push(t);
                        }
                    }
                }
            }
        }

        // Fall back to built-in themes embedded at compile time.
        if themes.is_empty() {
            let builtins: &[(&str, &str)] = &[
                ("light", include_str!("../../themes/light.json")),
                ("dark", include_str!("../../themes/dark.json")),
                ("amoled", include_str!("../../themes/amoled.json")),
                ("green", include_str!("../../themes/green.json")),
                ("blue", include_str!("../../themes/blue.json")),
                ("purple", include_str!("../../themes/purple.json")),
                ("high_contrast", include_str!("../../themes/high_contrast.json")),
                ("custom", include_str!("../../themes/custom.json")),
            ];
            for (_key, json) in builtins {
                if let Ok(t) = ThemeTokens::from_json(json) {
                    themes.push(t);
                }
            }
        }

        let mut mgr = Self {
            themes,
            active: 1, // dark by default
            canvas_bg_cached: egui::Color32::from_rgb(24, 24, 36),
            text_cached: egui::Color32::from_rgb(205, 214, 244),
        };
        mgr.refresh_cache();
        mgr.apply(ctx);
        mgr
    }

    pub fn available(&self) -> Vec<&str> {
        self.themes.iter().map(|t| t.name.as_str()).collect()
    }

    pub fn select(&mut self, name: &str) {
        if let Some(i) = self.themes.iter().position(|t| t.name == name) {
            self.active = i;
            self.refresh_cache();
        }
    }

    pub fn apply(&mut self, ctx: &egui::Context) {
        let t = &self.themes[self.active];
        let mut visuals = egui::Visuals::dark();
        visuals.window_fill = t.color("panel");
        visuals.panel_fill = t.color("panel");
        visuals.extreme_bg_color = t.color("background");
        visuals.faint_bg_color = t.color("background");
        visuals.override_text_color = Some(t.color("text"));
        visuals.selection.bg_fill = t.color("accent");
        ctx.set_visuals(visuals);
    }

    pub fn canvas_bg(&self) -> egui::Color32 {
        self.canvas_bg_cached
    }

    pub fn text_color(&self) -> egui::Color32 {
        self.text_cached
    }

    fn refresh_cache(&mut self) {
        let t = &self.themes[self.active];
        self.canvas_bg_cached = t.color("canvas");
        self.text_cached = t.color("text");
    }
}

// Keep paths param used.
#[allow(dead_code)]
fn _use_paths(_p: &AppPaths) {}