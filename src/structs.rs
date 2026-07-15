use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::consts::LazerHitResult;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScoreInfo {
    #[serde(default)]
    pub online_id: i64,
    pub mods: Vec<ModInfo>,
    pub statistics: HashMap<LazerHitResult, i64>,
    pub maximum_statistics: HashMap<LazerHitResult, i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModInfo {
    pub acronym: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone)]
pub struct ReplayData {
    pub time: f64,
    pub mouse_x: f64,
    pub mouse_y: f64,
    pub key_pressed: KeyPressed,
}

#[derive(Debug, Clone)]
pub struct KeyPressed {
    pub left_click: bool,
    pub right_click: bool,
    pub key1: bool,
    pub key2: bool,
    pub smoke: bool,
}

#[derive(Debug, Clone)]
pub struct LifeBarGraph {
    pub time: i32,
    pub hp: f32,
}
