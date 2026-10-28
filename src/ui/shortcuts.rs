// Rebindable keyboard shortcuts (stub).

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ShortcutMap {
    pub bindings: HashMap<String, String>,
}

impl Default for ShortcutMap {
    fn default() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }
}