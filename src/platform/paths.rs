// OS-specific paths for config, cache, data.
//
// Uses the `directories` crate so we get the right folders on Windows and Linux.

use directories::ProjectDirs;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct AppPaths {
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub cache_dir: PathBuf,
}

impl AppPaths {
    pub fn new() -> Self {
        if let Some(proj) = ProjectDirs::from("org", "aetherdraw", "AetherDraw") {
            Self {
                config_dir: proj.config_dir().to_path_buf(),
                data_dir: proj.data_dir().to_path_buf(),
                cache_dir: proj.cache_dir().to_path_buf(),
            }
        } else {
            // Fallback for unusual environments.
            Self {
                config_dir: PathBuf::from("."),
                data_dir: PathBuf::from("."),
                cache_dir: PathBuf::from("."),
            }
        }
    }
}

impl Default for AppPaths {
    fn default() -> Self {
        Self::new()
    }
}