// Built-in brush presets. A preset is just a BrushSettings + a name.

use crate::brush::engine::{BrushSettings, BrushType};

pub struct BrushPreset {
    pub name: &'static str,
    pub settings: BrushSettings,
}

pub fn builtin_presets() -> Vec<BrushPreset> {
    let mut v = Vec::new();

    let mut pencil = BrushSettings::default();
    pencil.brush_type = BrushType::Pencil;
    pencil.size = 4.0;
    pencil.hardness = 0.9;
    pencil.opacity = 0.9;
    v.push(BrushPreset { name: "Pencil", settings: pencil });

    let mut ink = BrushSettings::default();
    ink.brush_type = BrushType::Ink;
    ink.size = 6.0;
    ink.hardness = 1.0;
    ink.opacity = 1.0;
    v.push(BrushPreset { name: "Ink", settings: ink });

    let mut air = BrushSettings::default();
    air.brush_type = BrushType::Airbrush;
    air.size = 40.0;
    air.hardness = 0.0;
    air.opacity = 0.3;
    air.flow = 0.5;
    v.push(BrushPreset { name: "Airbrush", settings: air });

    let mut wc = BrushSettings::default();
    wc.brush_type = BrushType::Watercolor;
    wc.size = 30.0;
    wc.hardness = 0.2;
    wc.opacity = 0.6;
    wc.scatter = 0.4;
    v.push(BrushPreset { name: "Watercolor", settings: wc });

    v
}