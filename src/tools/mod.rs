// Tool definitions (brush, eraser, move, select, text, shape, ...).
// Tools are just enums; the app maps the active tool to behavior.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tool {
    Brush,
    Eraser,
    Move,
    Select,
    Text,
    Shape,
    Fill,
    Eyedropper,
    Zoom,
    Pan,
}

impl Tool {
    pub fn all() -> &'static [Tool] {
        &[
            Tool::Brush,
            Tool::Eraser,
            Tool::Move,
            Tool::Select,
            Tool::Text,
            Tool::Shape,
            Tool::Fill,
            Tool::Eyedropper,
            Tool::Zoom,
            Tool::Pan,
        ]
    }
    pub fn label(&self) -> &'static str {
        match self {
            Tool::Brush => "Brush",
            Tool::Eraser => "Eraser",
            Tool::Move => "Move",
            Tool::Select => "Select",
            Tool::Text => "Text",
            Tool::Shape => "Shape",
            Tool::Fill => "Fill",
            Tool::Eyedropper => "Eyedropper",
            Tool::Zoom => "Zoom",
            Tool::Pan => "Pan",
        }
    }
}