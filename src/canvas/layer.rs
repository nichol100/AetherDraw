// Layer system.
//
// A document holds a list of layers. Each layer is one of several types.
// Phase 1 fully implements Raster layers; others are stubs for later phases.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    SoftLight,
    HardLight,
    ColorDodge,
    ColorBurn,
    Darken,
    Lighten,
    Difference,
    Add,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

impl BlendMode {
    pub fn all() -> &'static [BlendMode] {
        &[
            BlendMode::Normal,
            BlendMode::Multiply,
            BlendMode::Screen,
            BlendMode::Overlay,
            BlendMode::SoftLight,
            BlendMode::HardLight,
            BlendMode::ColorDodge,
            BlendMode::ColorBurn,
            BlendMode::Darken,
            BlendMode::Lighten,
            BlendMode::Difference,
            BlendMode::Add,
            BlendMode::Hue,
            BlendMode::Saturation,
            BlendMode::Color,
            BlendMode::Luminosity,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            BlendMode::Normal => "Normal",
            BlendMode::Multiply => "Multiply",
            BlendMode::Screen => "Screen",
            BlendMode::Overlay => "Overlay",
            BlendMode::SoftLight => "Soft Light",
            BlendMode::HardLight => "Hard Light",
            BlendMode::ColorDodge => "Color Dodge",
            BlendMode::ColorBurn => "Color Burn",
            BlendMode::Darken => "Darken",
            BlendMode::Lighten => "Lighten",
            BlendMode::Difference => "Difference",
            BlendMode::Add => "Add",
            BlendMode::Hue => "Hue",
            BlendMode::Saturation => "Saturation",
            BlendMode::Color => "Color",
            BlendMode::Luminosity => "Luminosity",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Layer {
    Raster(RasterLayer),
    Vector(VectorLayer),
    Text(TextLayer),
    Object3D(Object3DLayer),
    Reference(ReferenceLayer),
    Group(GroupLayer),
}

impl Layer {
    pub fn name(&self) -> &str {
        match self {
            Layer::Raster(l) => &l.name,
            Layer::Vector(l) => &l.name,
            Layer::Text(l) => &l.name,
            Layer::Object3D(l) => &l.name,
            Layer::Reference(l) => &l.name,
            Layer::Group(l) => &l.name,
        }
    }

    pub fn visible(&self) -> bool {
        match self {
            Layer::Raster(l) => l.visible,
            Layer::Vector(l) => l.visible,
            Layer::Text(l) => l.visible,
            Layer::Object3D(l) => l.visible,
            Layer::Reference(l) => l.visible,
            Layer::Group(l) => l.visible,
        }
    }

    pub fn set_visible(&mut self, v: bool) {
        match self {
            Layer::Raster(l) => l.visible = v,
            Layer::Vector(l) => l.visible = v,
            Layer::Text(l) => l.visible = v,
            Layer::Object3D(l) => l.visible = v,
            Layer::Reference(l) => l.visible = v,
            Layer::Group(l) => l.visible = v,
        }
    }

    pub fn opacity(&self) -> f32 {
        match self {
            Layer::Raster(l) => l.opacity,
            Layer::Vector(l) => l.opacity,
            Layer::Text(l) => l.opacity,
            Layer::Object3D(l) => l.opacity,
            Layer::Reference(l) => l.opacity,
            Layer::Group(l) => l.opacity,
        }
    }

    pub fn blend(&self) -> BlendMode {
        match self {
            Layer::Raster(l) => l.blend,
            Layer::Vector(l) => l.blend,
            Layer::Text(l) => l.blend,
            Layer::Object3D(l) => l.blend,
            Layer::Reference(l) => l.blend,
            Layer::Group(l) => l.blend,
        }
    }
}

// ---------- Raster layer (fully used in Phase 1) ----------

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RasterLayer {
    pub id: Uuid,
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    pub opacity: f32,
    pub blend: BlendMode,
    pub alpha_lock: bool,
    pub clipping: bool,
    pub width: u32,
    pub height: u32,
    /// RGBA8 pixel buffer.
    pub pixels: Vec<u8>,
}

impl RasterLayer {
    pub fn new(name: impl Into<String>, width: u32, height: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            visible: true,
            locked: false,
            opacity: 1.0,
            blend: BlendMode::Normal,
            alpha_lock: false,
            clipping: false,
            width,
            height,
            pixels: vec![0u8; (width * height * 4) as usize],
        }
    }

    pub fn index(&self, x: u32, y: u32) -> usize {
        ((y * self.width + x) * 4) as usize
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, rgba: [u8; 4]) {
        if x >= self.width || y >= self.height {
            return;
        }
        let i = self.index(x, y);
        self.pixels[i..i + 4].copy_from_slice(&rgba);
    }
}

// ---------- Other layer types (stubs) ----------

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VectorLayer {
    pub id: Uuid,
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    pub opacity: f32,
    pub blend: BlendMode,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextLayer {
    pub id: Uuid,
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    pub opacity: f32,
    pub blend: BlendMode,
    pub text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Object3DLayer {
    pub id: Uuid,
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    pub opacity: f32,
    pub blend: BlendMode,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReferenceLayer {
    pub id: Uuid,
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    pub opacity: f32,
    pub blend: BlendMode,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupLayer {
    pub id: Uuid,
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    pub opacity: f32,
    pub blend: BlendMode,
    pub children: Vec<Layer>,
}

impl VectorLayer {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            visible: true,
            locked: false,
            opacity: 1.0,
            blend: BlendMode::Normal,
        }
    }
}
impl TextLayer {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            visible: true,
            locked: false,
            opacity: 1.0,
            blend: BlendMode::Normal,
            text: String::new(),
        }
    }
}
impl Object3DLayer {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            visible: true,
            locked: false,
            opacity: 1.0,
            blend: BlendMode::Normal,
        }
    }
}
impl ReferenceLayer {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            visible: true,
            locked: false,
            opacity: 1.0,
            blend: BlendMode::Normal,
        }
    }
}
impl GroupLayer {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            visible: true,
            locked: false,
            opacity: 1.0,
            blend: BlendMode::Normal,
            children: Vec::new(),
        }
    }
}