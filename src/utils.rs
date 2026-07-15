use chrono::{DateTime, NaiveDate, Utc};
use crate::structs::LifeBarGraph;

pub fn time_from_ticks(ticks: i64) -> DateTime<Utc> {
    let base = NaiveDate::from_ymd_opt(1, 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc()
        .timestamp();

    let seconds = ticks / 10_000_000 + base;
    let nanos = (ticks % 10_000_000) * 100;

    DateTime::from_timestamp(seconds, nanos as u32).unwrap_or_else(|| Utc::now())
}

pub fn ticks_from_time(time: DateTime<Utc>) -> i64 {
    let base = NaiveDate::from_ymd_opt(1, 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc()
        .timestamp();

    let timestamp = time.timestamp();
    let nanos = time.timestamp_subsec_nanos();

    (timestamp - base) * 10_000_000 + (nanos as i64) / 100
}

pub fn parse_lifebar(s: &str) -> Vec<LifeBarGraph> {
    let mut result = Vec::new();
    let s = s.trim_matches(',');

    for entry in s.split(',') {
        let parts: Vec<&str> = entry.split('|').collect();
        if parts.len() < 2 {
            continue;
        }

        let time = match parts[0].parse::<f32>() {
            Ok(v) => v as i32,
            Err(_) => continue,
        };

        let hp = match parts[1].parse::<f32>() {
            Ok(v) => v,
            Err(_) => continue,
        };

        result.push(LifeBarGraph { time, hp });
    }

    result
}

pub fn serialize_lifebar(lifebar: &[LifeBarGraph]) -> String {
    lifebar
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let s = format!("{}|{}", entry.time, entry.hp);
            if i < lifebar.len() - 1 {
                format!("{},", s)
            } else {
                s
            }
        })
        .collect()
}
