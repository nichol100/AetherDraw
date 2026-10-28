// Project file format (.aether): a zip archive with a JSON manifest + assets.

use crate::canvas::document::Document;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProjectManifest {
    pub version: u32,
    pub width: u32,
    pub height: u32,
    // Layer data is stored separately inside the zip.
}

pub fn save_project(_doc: &Document, _path: &std::path::Path) -> anyhow::Result<()> {
    // TODO: serialize document to zip.
    Ok(())
}

pub fn load_project(_path: &std::path::Path) -> anyhow::Result<Document> {
    // TODO.
    Ok(Document::new(1920, 1080))
}