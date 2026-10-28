// Audio engine + mixer scaffold.
//
// Real implementation uses cpal for low-latency playback/recording and
// symphonia for decoding. This module defines the data structures only.

use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct AudioTrack {
    pub name: String,
    pub color: u32, // packed RGBA
    pub volume: f32,
    pub pan: f32,
    pub mute: bool,
    pub solo: bool,
    pub locked: bool,
}

#[derive(Clone, Debug)]
pub struct AudioClip {
    pub path: Option<PathBuf>,
    pub start_frame: u32,
    pub length_frames: u32,
    pub volume: f32,
    pub fades: (f32, f32), // fade in/out seconds
}

pub struct AudioEngine {
    pub tracks: Vec<AudioTrack>,
    pub clips: Vec<AudioClip>,
    pub master_volume: f32,
}

impl Default for AudioEngine {
    fn default() -> Self {
        Self {
            tracks: Vec::new(),
            clips: Vec::new(),
            master_volume: 1.0,
        }
    }
}