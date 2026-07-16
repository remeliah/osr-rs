//! osu! replay file parser and writer.
//!
//! This library provides functionality to parse and write osu! replay files (.osr format),
//! supporting both stable and lazer replays with LZMA compression.
//!
//! # Quick Start
//!
//! ```no_run
//! use osr_rs::Replay;
//!
//! // Parse from file
//! let replay = Replay::from("replay.osr")?;
//! println!("Player: {}, Score: {}", replay.username, replay.score);
//!
//! // Write to file
//! replay.write("output.osr")?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use std::{
    fs,
    path::Path
};
use chrono::{DateTime, Utc};

mod binary;
mod compression;
mod consts;
mod mods;
mod replay;
mod structs;
mod utils;

pub use compression::{compress_replay_data, compress_score_info, decompress_replay_data, decompress_score_info};
pub use consts::*;
pub use mods::*;
pub use replay::{parse_replay, write_replay, Error};
pub use structs::{ScoreInfo, ModInfo, ReplayData, KeyPressed, LifeBarGraph};
pub use utils::{parse_lifebar, serialize_lifebar, ticks_from_time, time_from_ticks};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct Replay {
    pub play_mode: i8,
    pub osu_version: i32,
    pub beatmap_md5: String,
    pub username: String,
    pub replay_md5: String,
    pub count_300: u16,
    pub count_100: u16,
    pub count_50: u16,
    pub count_geki: u16,
    pub count_katu: u16,
    pub count_miss: u16,
    pub score: i32,
    pub max_combo: u16,
    pub fullcombo: bool,
    pub mods: u32,
    pub lifebar_graph: Vec<LifeBarGraph>,
    pub timestamp: DateTime<Utc>,
    pub replay_data: Vec<ReplayData>,
    pub online_id: i64,
    pub score_info: Option<ScoreInfo>,
}

impl Replay {
    pub fn new() -> Self {
        Self {
            play_mode: 0,
            osu_version: 0,
            beatmap_md5: String::new(),
            username: String::new(),
            replay_md5: String::new(),
            count_300: 0,
            count_100: 0,
            count_50: 0,
            count_geki: 0,
            count_katu: 0,
            count_miss: 0,
            score: 0,
            max_combo: 0,
            fullcombo: false,
            mods: 0,
            lifebar_graph: Vec::new(),
            timestamp: Utc::now(),
            replay_data: Vec::new(),
            online_id: 0,
            score_info: None,
        }
    }

    /// Parses a replay from raw bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the data is malformed or cannot be decompressed.
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        crate::replay::parse_replay(data)
    }

    /// Parses a replay from a file path.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or the replay data is invalid.
    pub fn from<P: AsRef<Path>>(path: P) -> std::result::Result<Self, Box<dyn std::error::Error>> {
        let data = fs::read(path)?;

        Ok(Self::from_bytes(&data)?)
    }

    /// Serializes the replay to bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if compression or serialization fails.
    pub fn write_bytes(&self) -> Result<Vec<u8>> {
        crate::replay::write_replay(self)
    }

    /// Writes the replay to a file path.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be written or serialization fails.
    pub fn write<P: AsRef<Path>>(&self, path: P) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let data = self.write_bytes()?;
        fs::write(path, data)?;

        Ok(())
    }

    pub fn play_mode(mut self, mode: i8) -> Self {
        self.play_mode = mode;
        self
    }

    pub fn osu_version(mut self, version: i32) -> Self {
        self.osu_version = version;
        self
    }

    pub fn beatmap_md5(mut self, md5: String) -> Self {
        self.beatmap_md5 = md5;
        self
    }

    pub fn username(mut self, name: String) -> Self {
        self.username = name;
        self
    }

    pub fn replay_md5(mut self, md5: String) -> Self {
        self.replay_md5 = md5;
        self
    }

    pub fn n300(mut self, count: u16) -> Self {
        self.count_300 = count;
        self
    }

    pub fn n100(mut self, count: u16) -> Self {
        self.count_100 = count;
        self
    }

    pub fn n50(mut self, count: u16) -> Self {
        self.count_50 = count;
        self
    }

    pub fn ngeki(mut self, count: u16) -> Self {
        self.count_geki = count;
        self
    }

    pub fn nkatu(mut self, count: u16) -> Self {
        self.count_katu = count;
        self
    }

    pub fn nmiss(mut self, count: u16) -> Self {
        self.count_miss = count;
        self
    }

    pub fn score(mut self, score: i32) -> Self {
        self.score = score;
        self
    }

    pub fn max_combo(mut self, combo: u16) -> Self {
        self.max_combo = combo;
        self
    }

    pub fn full_combo(mut self, fc: bool) -> Self {
        self.fullcombo = fc;
        self
    }

    pub fn mods(mut self, mods: u32) -> Self {
        self.mods = mods;
        self
    }

    pub fn timestamp(mut self, time: DateTime<Utc>) -> Self {
        self.timestamp = time;
        self
    }

    pub fn online_id(mut self, id: i64) -> Self {
        self.online_id = id;
        self
    }
}

impl Default for Replay {
    fn default() -> Self {
        Self::new()
    }
}
