// Windowing helpers (fullscreen, always-on-top, ...).
// winit/eframe already owns the window; this module exposes small helpers.

pub fn set_title(_title: &str) {
    // eframe owns the window; title is set via NativeOptions.
}