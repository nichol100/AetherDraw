// AetherDraw - entry point.
//
// This file only wires the window + GUI framework (eframe/egui) to our
// `AetherApp`. All real logic lives in the modules under `src/`.

mod app;
mod animation;
mod history;
mod audio;
mod brush;
mod canvas;
mod eraser;
mod fileformat;
mod filters;
mod physics;
mod platform;
mod plugin;
mod render3d;
mod tools;
mod ui;

use app::AetherApp;
use eframe::NativeOptions;

fn main() -> eframe::Result<()> {
    // Turn on logging so beginners can see what the app is doing.
    env_logger::init();

    // Basic window options. Platform-specific tweaks live in platform/.
    let mut options = NativeOptions::default();
    options.viewport.inner_size = Some(egui::Vec2::new(1600.0, 960.0));
    options.viewport.title = "AetherDraw".to_string();

    // Hand control to eframe. It will call AetherApp::update() every frame.
    eframe::run_native(
        "AetherDraw",
        options,
        Box::new(|cc| {
            // cc (CreationContext) gives us the egui context + storage.
            Ok(Box::new(AetherApp::new(cc)))
        }),
    )
}