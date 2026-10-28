// Thin file-system helpers. Kept separate so platform quirks (case sensitivity,
// path separators, sandboxing) can be handled in one place.

use std::fs;
use std::path::Path;

/// Read a text file, returning its contents.
pub fn read_text(path: &Path) -> std::io::Result<String> {
    fs::read_to_string(path)
}

/// Write a text file, creating parent directories first.
pub fn write_text(path: &Path, contents: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, contents)
}

/// Returns true if the path exists and is a directory.
pub fn is_dir(path: &Path) -> bool {
    path.is_dir()
}