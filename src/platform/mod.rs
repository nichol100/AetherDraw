// Platform abstraction layer.
//
// ALL operating-system-specific code lives here. The rest of the app talks to
// these traits/structs only, so Windows and Linux share one codebase.

pub mod fs;
pub mod paths;
pub mod tablet;
pub mod window;

#[cfg(target_os = "windows")]
pub mod windows_impl;
#[cfg(target_os = "linux")]
pub mod linux_impl;