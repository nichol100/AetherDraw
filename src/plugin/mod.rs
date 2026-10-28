// Plugin API scaffold. Plugins are dynamic libraries loaded at runtime.

pub trait Plugin {
    fn name(&self) -> &str;
    fn on_load(&mut self);
}