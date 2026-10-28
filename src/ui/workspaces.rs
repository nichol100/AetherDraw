// Workspaces: Paint, Animate, Model, Physics.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Workspace {
    Paint,
    Animate,
    Model,
    Physics,
}

impl Workspace {
    pub const ALL: [Workspace; 4] = [
        Workspace::Paint,
        Workspace::Animate,
        Workspace::Model,
        Workspace::Physics,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            Workspace::Paint => "Paint",
            Workspace::Animate => "Animate",
            Workspace::Model => "Model",
            Workspace::Physics => "Physics",
        }
    }
}