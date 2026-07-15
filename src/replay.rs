use std::io::{self, Cursor, Write};

use crate::binary::*;
use crate::compression::{self, CompressionError};
use crate::utils::{parse_lifebar, serialize_lifebar, ticks_from_time, time_from_ticks};
use crate::Replay;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO: {0}")]
    Io(#[from] io::Error),
    #[error("Compression: {0}")]
    Compression(#[from] CompressionError),
    #[error("Parse: {0}")]
    Parse(String),
}

pub fn parse_replay(file: &[u8]) -> Result<Replay, Error> {
    let mut cursor = Cursor::new(file);
    let mut replay = Replay::new();

    replay.play_mode = read_i8(&mut cursor)
        .map_err(|e| Error::Parse(format!("Reading PlayMode: {}", e)))?;

    replay.osu_version = read_i32(&mut cursor)
        .map_err(|e| Error::Parse(format!("Reading OsuVersion: {}", e)))?;

    replay.beatmap_md5 = read_bstring(&mut cursor)
        .map_err(|e| Error::Parse(format!("Reading BeatmapMD5: {}", e)))?;

    replay.username = read_bstring(&mut cursor)
        .map_err(|e| Error::Parse(format!("Reading Username: {}", e)))?;

    replay.replay_md5 = read_bstring(&mut cursor)
        .map_err(|e| Error::Parse(format!("Reading ReplayMD5: {}", e)))?;

    replay.count_300 = read_u16(&mut cursor)
        .map_err(|e| Error::Parse(format!("Reading Count300: {}", e)))?;

    replay.count_100 = read_u16(&mut cursor)
        .map_err(|e| Error::Parse(format!("Reading Count100: {}", e)))?;

    replay.count_50 = read_u16(&mut cursor)
        .map_err(|e| Error::Parse(format!("Reading Count50: {}", e)))?;

    replay.count_geki = read_u16(&mut cursor)
        .map_err(|e| Error::Parse(format!("Reading CountGeki: {}", e)))?;

    replay.count_katu = read_u16(&mut cursor)
        .map_err(|e| Error::Parse(format!("Reading CountKatu: {}", e)))?;

    replay.count_miss = read_u16(&mut cursor)
        .map_err(|e| Error::Parse(format!("Reading CountMiss: {}", e)))?;

    replay.score = read_i32(&mut cursor)
        .map_err(|e| Error::Parse(format!("Reading Score: {}", e)))?;

    replay.max_combo = read_u16(&mut cursor)
        .map_err(|e| Error::Parse(format!("Reading MaxCombo: {}", e)))?;

    replay.fullcombo = read_bool(&mut cursor)
        .map_err(|e| Error::Parse(format!("Reading Fullcombo: {}", e)))?;

    replay.mods = read_u32(&mut cursor)
        .map_err(|e| Error::Parse(format!("Reading Mods: {}", e)))?;

    let lifebar_raw = read_bstring(&mut cursor)
        .map_err(|e| Error::Parse(format!("Reading LifeBar: {}", e)))?;

    replay.lifebar_graph = parse_lifebar(&lifebar_raw);

    let ts = read_i64(&mut cursor)
        .map_err(|e| Error::Parse(format!("Reading Timestamp: {}", e)))?;

    replay.timestamp = time_from_ticks(ts);

    let c_length = read_i32(&mut cursor)
        .map_err(|e| Error::Parse(format!("Reading ReplayData length: {}", e)))?;

    if c_length > 0 {
        let compressed_replay = read_slice(&mut cursor, c_length)
            .map_err(|e| Error::Parse(format!("Reading ReplayData: {}", e)))?;

        replay.replay_data = compression::decompress_replay_data(&compressed_replay)?;
    }

    let remaining = cursor.get_ref().len() - cursor.position() as usize;

    if remaining >= 8 {
        replay.online_id = read_i64(&mut cursor)
            .map_err(|e| Error::Parse(format!("Reading ScoreID: {}", e)))?;
    } else if remaining >= 4 {
        let s_id = read_i32(&mut cursor)
            .map_err(|e| Error::Parse(format!("Reading ScoreID: {}", e)))?;
        replay.online_id = s_id as i64;
    }

    let remaining = cursor.get_ref().len() - cursor.position() as usize;
    if remaining < 4 {
        return Ok(replay);
    }

    let d_length = read_i32(&mut cursor)
        .map_err(|e| Error::Parse(format!("Reading ScoreInfo length: {}", e)))?;

    if d_length > 0 {
        let compressed_score_info = read_slice(&mut cursor, d_length)
            .map_err(|e| Error::Parse(format!("Reading ScoreInfo: {}", e)))?;

        let score_info = compression::decompress_score_info(&compressed_score_info)?;

        replay.online_id = score_info.online_id;
        replay.score_info = Some(score_info);
    } else if d_length == 0 {
        replay.score_info = None;
    }

    Ok(replay)
}

pub fn write_replay(replay: &Replay) -> Result<Vec<u8>, Error> {
    let mut buf = Vec::with_capacity(1024);

    buf.write_all(&write_i8(replay.play_mode))?;
    buf.write_all(&write_i32(replay.osu_version))?;
    buf.write_all(&write_bstring(&replay.beatmap_md5))?;
    buf.write_all(&write_bstring(&replay.username))?;
    buf.write_all(&write_bstring(&replay.replay_md5))?;
    buf.write_all(&write_u16(replay.count_300))?;
    buf.write_all(&write_u16(replay.count_100))?;
    buf.write_all(&write_u16(replay.count_50))?;
    buf.write_all(&write_u16(replay.count_geki))?;
    buf.write_all(&write_u16(replay.count_katu))?;
    buf.write_all(&write_u16(replay.count_miss))?;
    buf.write_all(&write_i32(replay.score))?;
    buf.write_all(&write_u16(replay.max_combo))?;
    buf.write_all(&write_bool(replay.fullcombo))?;
    buf.write_all(&write_u32(replay.mods))?;
    buf.write_all(&write_bstring(&serialize_lifebar(&replay.lifebar_graph)))?;
    buf.write_all(&write_i64(ticks_from_time(replay.timestamp)))?;

    if !replay.replay_data.is_empty() {
        let data = compression::compress_replay_data(&replay.replay_data)?;
        buf.write_all(&write_i32(data.len() as i32))?;
        buf.write_all(&data)?;
    } else {
        buf.write_all(&write_i32(0))?;
    }

    buf.write_all(&write_u64(replay.online_id as u64))?;

    Ok(buf)
}
