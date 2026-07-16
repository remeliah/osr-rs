use std::io::Cursor;
use lzma_rs::{lzma_decompress, lzma_compress};

use crate::consts::*;
use crate::structs::*;

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct CompressionError(pub String);

pub fn decompress_replay_data(file: &[u8]) -> Result<Vec<ReplayData>, CompressionError> {
    let mut decompressed = Vec::new();
    lzma_decompress(&mut Cursor::new(file), &mut decompressed)
        .map_err(|e| CompressionError(format!("Decompressing: {}", e)))?;

    let data_str = String::from_utf8(decompressed)
        .map_err(|e| CompressionError(format!("Invalid UTF-8: {}", e)))?;

    let trimmed = data_str.trim_matches(',');
    let event_count = trimmed.matches(',').count() + if trimmed.is_empty() { 0 } else { 1 };
    let mut result = Vec::with_capacity(event_count);

    for (i, event) in trimmed.split(',').enumerate() {
        let mut parts = event.split('|');

        let time = parts.next()
            .ok_or_else(|| CompressionError(format!("Missing Time on event {}", i)))?
            .parse::<f64>()
            .map_err(|e| CompressionError(format!("Parsing Time on event {}: {}", i, e)))?;

        let mouse_x = parts.next()
            .ok_or_else(|| CompressionError(format!("Missing MouseX on event {}", i)))?
            .parse::<f64>()
            .map_err(|e| CompressionError(format!("Parsing MouseX on event {}: {}", i, e)))?;

        let mouse_y = parts.next()
            .ok_or_else(|| CompressionError(format!("Missing MouseY on event {}", i)))?
            .parse::<f64>()
            .map_err(|e| CompressionError(format!("Parsing MouseY on event {}: {}", i, e)))?;

        let keys = parts.next()
            .ok_or_else(|| CompressionError(format!("Missing Keys on event {}", i)))?
            .parse::<i32>()
            .map_err(|e| CompressionError(format!("Parsing Keys on event {}: {}", i, e)))?;

        result.push(ReplayData {
            time,
            mouse_x,
            mouse_y,
            key_pressed: KeyPressed {
                left_click: keys & LEFTCLICK > 0,
                right_click: keys & RIGHTCLICK > 0,
                key1: keys & KEY1 > 0,
                key2: keys & KEY2 > 0,
                smoke: keys & SMOKE > 0,
            },
        });
    }

    Ok(result)
}

pub fn decompress_score_info(file: &[u8]) -> Result<ScoreInfo, CompressionError> {
    let mut decompressed = Vec::new();
    lzma_decompress(&mut Cursor::new(file), &mut decompressed)
        .map_err(|e| CompressionError(format!("Decompressing: {}", e)))?;

    let score_info: ScoreInfo = serde_json::from_slice(&decompressed)
        .map_err(|e| CompressionError(format!("Parsing JSON: {}", e)))?;

    Ok(score_info)
}

pub fn compress_replay_data(data: &[ReplayData]) -> Result<Vec<u8>, CompressionError> {
    use std::io::Write;

    let estimated_size = data.len() * 40;
    let mut frame_data = Vec::with_capacity(estimated_size);

    for (i, rd) in data.iter().enumerate() {
        let mut keys = 0;

        if rd.key_pressed.left_click {
            keys |= LEFTCLICK;
        }
        if rd.key_pressed.right_click {
            keys |= RIGHTCLICK;
        }
        if rd.key_pressed.key1 {
            keys |= KEY1;
        }
        if rd.key_pressed.key2 {
            keys |= KEY2;
        }
        if rd.key_pressed.smoke {
            keys |= SMOKE;
        }

        write!(&mut frame_data, "{}|{}|{}|{}", rd.time, rd.mouse_x, rd.mouse_y, keys)
            .map_err(|e| CompressionError(format!("Writing frame data: {}", e)))?;

        if i < data.len() - 1 {
            frame_data.push(b',');
        }
    }

    let mut compressed = Vec::new();
    lzma_compress(&mut Cursor::new(&frame_data), &mut compressed)
        .map_err(|e| CompressionError(format!("Compressing: {}", e)))?;

    Ok(compressed)
}

pub fn compress_score_info(score_info: &ScoreInfo) -> Result<Vec<u8>, CompressionError> {
    let json_bytes = serde_json::to_vec(score_info)
        .map_err(|e| CompressionError(format!("Serializing JSON: {}", e)))?;

    let mut compressed = Vec::new();
    lzma_compress(&mut Cursor::new(&json_bytes), &mut compressed)
        .map_err(|e| CompressionError(format!("Compressing: {}", e)))?;

    Ok(compressed)
}
