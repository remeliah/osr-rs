# osr-rs

osu! replay parser/writer.

## Features

- Parse osu! replay files (.osr format)
- Write osu! replay files
- Support for both stable and lazer replays
- LZMA compression/decompression
- Full replay data parsing (input events, lifebar, score info)

## Usage

```rust
use osr_rs::Replay;

// Parse from file path
let replay = Replay::from("replay.osr")?;

println!("Username: {}", replay.username);
println!("Score: {}", replay.score);
println!("Max Combo: {}", replay.max_combo);

// Write to file path
replay.write("output.osr")?;
```

### Advanced API

```rust
use osr_rs::{Replay, parse_replay, write_replay};
use std::fs;

// Parse from bytes
let data = fs::read("replay.osr")?;
let replay = parse_replay(&data)?;

// Or use methods
let replay = Replay::from_bytes(&data)?;

// Write to bytes
let bytes = write_replay(&replay)?;
// Or use method
let bytes = replay.write_bytes()?;
```

### Mod System (Lazer)

The library provides a comprehensive mod system with over 70 osu!lazer mods and typed settings.

```rust
use osr_rs::{
    ModAcronym, Replay, simple_mod, double_time, RateAdjustSettings, 
    difficulty_adjust, DifficultyAdjustSettings
};

// Simple mods (no settings)
let hd = simple_mod(ModAcronym::HD);

// Mods with settings
let dt = double_time(RateAdjustSettings {
    speed_change: Some(1.5),
    adjust_pitch: Some(true),
});

let da = difficulty_adjust(DifficultyAdjustSettings {
    circle_size: Some(9.0),
    approach_rate: Some(10.0),
    overall_difficulty: None,
    drain_rate: None,
    extended_limits: Some(true),
});

// Apply to replay
let mut replay = Replay::new();
if let Some(ref mut score_info) = replay.score_info {
    score_info.mods = vec![hd, dt, da];
}
```

## Structure

- `src/consts.rs` - Constants (play modes, key states, lazer hit results)
- `src/structs.rs` - Data structures (Replay, ReplayData, ScoreInfo, etc.)
- `src/binary.rs` - Binary I/O functions (little-endian, ULEB128)
- `src/mods.rs` - Lazer mod system with typed settings
- `src/replay.rs` - Core parse/write API
- `src/compression.rs` - LZMA compression/decompression
- `src/utils.rs` - Time conversion and lifebar parsing
- `src/lib.rs` - Public API exports
- `src/main.rs` - CLI example

## API

### Convenience methods on Replay

```rust
impl Replay {
    // Parse from bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self, Error>
    
    // Parse from file path
    pub fn from<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>>
    
    // Serialize to bytes
    pub fn write_bytes(&self) -> Result<Vec<u8>, Error>
    
    // Write to file path
    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn Error>>
}
```

### Low-level functions

```rust
// Core parsing/writing
pub fn parse_replay(file: &[u8]) -> Result<Replay, Error>
pub fn write_replay(replay: &Replay) -> Result<Vec<u8>, Error>

// Compression utilities
pub fn decompress_replay_data(file: &[u8]) -> Result<Vec<ReplayData>, CompressionError>
pub fn decompress_score_info(file: &[u8]) -> Result<ScoreInfo, CompressionError>
pub fn compress_replay_data(data: &[ReplayData]) -> Result<Vec<u8>, CompressionError>

// Time and lifebar utilities
pub fn time_from_ticks(ticks: i64) -> DateTime<Utc>
pub fn ticks_from_time(time: DateTime<Utc>) -> i64
pub fn parse_lifebar(s: &str) -> Vec<LifeBarGraph>
pub fn serialize_lifebar(lifebar: &[LifeBarGraph]) -> String
```
