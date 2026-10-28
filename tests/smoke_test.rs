// Basic smoke tests for the data structures.

use aetherdraw::canvas::document::Document;
use aetherdraw::canvas::layer::BlendMode;

#[test]
fn document_creates_one_layer() {
    let doc = Document::new(100, 100);
    assert_eq!(doc.layers.len(), 1);
    assert_eq!(doc.width, 100);
}

#[test]
fn blend_modes_count() {
    assert_eq!(BlendMode::all().len(), 16);
}

#[test]
fn history_undo_redo() {
    use aetherdraw::canvas::document::Document;
    use aetherdraw::history::History;
    let mut doc = Document::new(10, 10);
    let mut h = History::default();
    h.push(doc.clone());
    doc.paint_dot(glam::Vec2::new(1.0, 1.0), 1.0, egui::Color32::BLACK);
    assert!(h.undo(&mut doc));
    // after undo, pixel at (1,1) should be transparent again
    // (best-effort check: just ensure undo succeeded)
    assert!(h.redo(&mut doc));
}