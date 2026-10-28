// Undo/redo history.
//
// We use a simple snapshot-based approach: before a mutating action we clone
// the whole Document and push it onto the undo stack. Undo swaps the current
// document with the top of the undo stack (moving the current one to redo).

use crate::canvas::document::Document;

pub struct History {
    undo_stack: Vec<Document>,
    redo_stack: Vec<Document>,
    /// Maximum number of undo steps to keep.
    limit: usize,
}

impl Default for History {
    fn default() -> Self {
        Self::new(100)
    }
}

impl History {
    pub fn new(limit: usize) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            limit,
        }
    }

    /// Call this BEFORE mutating the document.
    pub fn push(&mut self, snapshot: Document) {
        self.undo_stack.push(snapshot);
        if self.undo_stack.len() > self.limit {
            self.undo_stack.remove(0);
        }
        // A new action invalidates the redo branch.
        self.redo_stack.clear();
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Undo: returns true if something was undone.
    pub fn undo(&mut self, current: &mut Document) -> bool {
        if let Some(prev) = self.undo_stack.pop() {
            self.redo_stack.push(current.clone());
            *current = prev;
            true
        } else {
            false
        }
    }

    pub fn redo(&mut self, current: &mut Document) -> bool {
        if let Some(next) = self.redo_stack.pop() {
            self.undo_stack.push(current.clone());
            *current = next;
            true
        } else {
            false
        }
    }
}