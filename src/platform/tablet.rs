// Tablet / stylus input abstraction.
//
// Real implementations (WinTab on Windows, evdev/libinput on Linux) go behind
// this trait. For now we provide a "MouseTablet" fallback so the app always
// works even without a tablet.

use glam::Vec2;

/// A single sampled point from a stylus/mouse.
#[derive(Clone, Copy, Debug, Default)]
pub struct TabletPoint {
    pub position: Vec2,
    /// 0.0..=1.0
    pub pressure: f32,
    /// Tilt in radians (x, y).
    pub tilt: Vec2,
    /// Speed in pixels/sec.
    pub velocity: f32,
}

/// Trait implemented by each platform backend.
pub trait TabletBackend: Send + Sync {
    fn poll_events(&mut self) -> Vec<TabletEvent>;
    fn is_available(&self) -> bool;
}

#[derive(Clone, Debug)]
pub enum TabletEvent {
    Down(TabletPoint),
    Move(TabletPoint),
    Up(TabletPoint),
}

/// Mouse fallback: pressure is always 1.0 (or 0.5 for "soft" feel).
pub struct MouseTablet;

impl TabletBackend for MouseTablet {
    fn poll_events(&mut self) -> Vec<TabletEvent> {
        // The winit mouse events are forwarded by the UI layer instead.
        Vec::new()
    }

    fn is_available(&self) -> bool {
        true
    }
}

/// Factory: pick the best backend for this OS.
pub fn default_tablet() -> Box<dyn TabletBackend> {
    // Platform-specific backends are selected via cfg in the impl modules.
    Box::new(MouseTablet)
}