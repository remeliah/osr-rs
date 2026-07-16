use serde::{Deserialize, Serialize};

use crate::structs::ModInfo;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModAcronym {
    // Difficulty Reduction
    EZ,  // Easy
    NF,  // No Fail
    HT,  // Half Time
    DC,  // Daycore
    NR,  // No Release (Mania)

    // Difficulty Increase
    HR,  // Hard Rock
    SD,  // Sudden Death
    PF,  // Perfect
    DT,  // Double Time
    NC,  // Nightcore
    FI,  // Fade In (Mania)
    HD,  // Hidden
    CO,  // Cover (Mania)
    FL,  // Flashlight
    BL,  // Blinds
    ST,  // Strict Tracking
    AC,  // Accuracy Challenge

    // Automation
    AT,  // Autoplay
    CN,  // Cinema
    RX,  // Relax
    AP,  // Autopilot
    SO,  // Spun Out

    // Conversion
    TP,  // Target Practice
    DA,  // Difficulty Adjust
    CL,  // Classic
    RD,  // Random
    MR,  // Mirror
    AL,  // Alternate
    SW,  // Swap
    SG,  // Single Tap
    IN,  // Invert
    CS,  // Constant Speed
    HO,  // Hold Off
    K1,  // 1K
    K2,  // 2K
    K3,  // 3K
    K4,  // 4K
    K5,  // 5K
    K6,  // 6K
    K7,  // 7K
    K8,  // 8K
    K9,  // 9K
    K10, // 10K

    // Fun
    TR,  // Transform
    WG,  // Wiggle
    SI,  // Spin In
    GR,  // Grow
    DF,  // Deflate
    WU,  // Wind Up
    WD,  // Wind Down
    TC,  // Traceable
    BR,  // Barrel Roll
    AD,  // Approach Different
    FF,  // Floating Fruits
    MU,  // Muted
    NS,  // No Scope
    MG,  // Magnetised
    RP,  // Repel
    AS,  // Adaptive Speed
    FR,  // Freeze Frame
    BU,  // Bubbles
    SY,  // Synesthesia
    DP,  // Depth
    BM,  // Bloom
    SR,  // Simplified Rhythm (Taiko)
}

impl ModAcronym {
    pub fn as_str(&self) -> &'static str {
        match self {
            ModAcronym::EZ => "EZ",
            ModAcronym::NF => "NF",
            ModAcronym::HT => "HT",
            ModAcronym::DC => "DC",
            ModAcronym::NR => "NR",
            ModAcronym::HR => "HR",
            ModAcronym::SD => "SD",
            ModAcronym::PF => "PF",
            ModAcronym::DT => "DT",
            ModAcronym::NC => "NC",
            ModAcronym::FI => "FI",
            ModAcronym::HD => "HD",
            ModAcronym::CO => "CO",
            ModAcronym::FL => "FL",
            ModAcronym::BL => "BL",
            ModAcronym::ST => "ST",
            ModAcronym::AC => "AC",
            ModAcronym::AT => "AT",
            ModAcronym::CN => "CN",
            ModAcronym::RX => "RX",
            ModAcronym::AP => "AP",
            ModAcronym::SO => "SO",
            ModAcronym::TP => "TP",
            ModAcronym::DA => "DA",
            ModAcronym::CL => "CL",
            ModAcronym::RD => "RD",
            ModAcronym::MR => "MR",
            ModAcronym::AL => "AL",
            ModAcronym::SW => "SW",
            ModAcronym::SG => "SG",
            ModAcronym::IN => "IN",
            ModAcronym::CS => "CS",
            ModAcronym::HO => "HO",
            ModAcronym::K1 => "1K",
            ModAcronym::K2 => "2K",
            ModAcronym::K3 => "3K",
            ModAcronym::K4 => "4K",
            ModAcronym::K5 => "5K",
            ModAcronym::K6 => "6K",
            ModAcronym::K7 => "7K",
            ModAcronym::K8 => "8K",
            ModAcronym::K9 => "9K",
            ModAcronym::K10 => "10K",
            ModAcronym::TR => "TR",
            ModAcronym::WG => "WG",
            ModAcronym::SI => "SI",
            ModAcronym::GR => "GR",
            ModAcronym::DF => "DF",
            ModAcronym::WU => "WU",
            ModAcronym::WD => "WD",
            ModAcronym::TC => "TC",
            ModAcronym::BR => "BR",
            ModAcronym::AD => "AD",
            ModAcronym::FF => "FF",
            ModAcronym::MU => "MU",
            ModAcronym::NS => "NS",
            ModAcronym::MG => "MG",
            ModAcronym::RP => "RP",
            ModAcronym::AS => "AS",
            ModAcronym::FR => "FR",
            ModAcronym::BU => "BU",
            ModAcronym::SY => "SY",
            ModAcronym::DP => "DP",
            ModAcronym::BM => "BM",
            ModAcronym::SR => "SR",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifficultyAdjustSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "CS")]
    pub circle_size: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "AR")]
    pub approach_rate: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "OD")]
    pub overall_difficulty: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "HP")]
    pub drain_rate: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_limits: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlashlightSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_delay: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_multiplier: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub combo_based_size: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HiddenSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_fade_approach_circles: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApproachDifferentSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<AnimationStyle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum AnimationStyle {
    Linear,
    Gravity,
    InOut1,
    InOut2,
    Accelerate1,
    Accelerate2,
    Accelerate3,
    Decelerate1,
    Decelerate2,
    Decelerate3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BloomSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_combo_count: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_cursor_size: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassicSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_slider_head_accuracy: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub classic_note_lock: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_play_tail_sample: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_hit_circle_early: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub classic_health: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepthSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_depth: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_approach_circles: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagnetisedSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attraction_strength: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepelSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repulsion_strength: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirrorSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reflection: Option<MirrorType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MirrorType {
    Horizontal,
    Vertical,
    Both,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub angle_sharpness: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WiggleSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetPracticeSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metronome: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuddenDeathSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_on_slider_tail: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectScaleTweenSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_scale: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateAdjustSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_change: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjust_pitch: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRampSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_rate: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_rate: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjust_pitch: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveSpeedSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_rate: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjust_pitch: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<CoverDirection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum CoverDirection {
    Up,
    Down,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerfectSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_perfect_hits: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaikoDifficultyAdjustSettings {
    #[serde(flatten)]
    pub base: DifficultyAdjustSettings,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll_speed: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatchDifficultyAdjustSettings {
    #[serde(flatten)]
    pub base: DifficultyAdjustSettings,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard_rock_offsets: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimplifiedRhythmSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_third_conversion: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_sixth_conversion: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_eighth_conversion: Option<bool>,
}

impl ModInfo {
    pub fn new(acronym: ModAcronym) -> Self {
        Self {
            acronym: acronym.as_str().to_string(),
            settings: None,
        }
    }

    pub fn with_settings(acronym: ModAcronym, settings: serde_json::Value) -> Self {
        Self {
            acronym: acronym.as_str().to_string(),
            settings: Some(settings.as_object().unwrap().iter().map(|(k, v)| (k.clone(), v.clone())).collect()),
        }
    }
}

pub fn simple_mod(acronym: ModAcronym) -> ModInfo {
    ModInfo::new(acronym)
}

pub fn difficulty_adjust(settings: DifficultyAdjustSettings) -> ModInfo {
    ModInfo::with_settings(ModAcronym::DA, serde_json::to_value(&settings).unwrap())
}

pub fn flashlight(settings: FlashlightSettings) -> ModInfo {
    ModInfo::with_settings(ModAcronym::FL, serde_json::to_value(&settings).unwrap())
}

pub fn hidden(settings: HiddenSettings) -> ModInfo {
    ModInfo::with_settings(ModAcronym::HD, serde_json::to_value(&settings).unwrap())
}

pub fn approach_different(settings: ApproachDifferentSettings) -> ModInfo {
    ModInfo::with_settings(ModAcronym::AD, serde_json::to_value(&settings).unwrap())
}

pub fn bloom(settings: BloomSettings) -> ModInfo {
    ModInfo::with_settings(ModAcronym::BM, serde_json::to_value(&settings).unwrap())
}

pub fn classic(settings: ClassicSettings) -> ModInfo {
    ModInfo::with_settings(ModAcronym::CL, serde_json::to_value(&settings).unwrap())
}

pub fn depth(settings: DepthSettings) -> ModInfo {
    ModInfo::with_settings(ModAcronym::DP, serde_json::to_value(&settings).unwrap())
}

pub fn magnetised(settings: MagnetisedSettings) -> ModInfo {
    ModInfo::with_settings(ModAcronym::MG, serde_json::to_value(&settings).unwrap())
}

pub fn repel(settings: RepelSettings) -> ModInfo {
    ModInfo::with_settings(ModAcronym::RP, serde_json::to_value(&settings).unwrap())
}

pub fn mirror(settings: MirrorSettings) -> ModInfo {
    ModInfo::with_settings(ModAcronym::MR, serde_json::to_value(&settings).unwrap())
}

pub fn random(settings: RandomSettings) -> ModInfo {
    ModInfo::with_settings(ModAcronym::RD, serde_json::to_value(&settings).unwrap())
}

pub fn wiggle(settings: WiggleSettings) -> ModInfo {
    ModInfo::with_settings(ModAcronym::WG, serde_json::to_value(&settings).unwrap())
}

pub fn target_practice(settings: TargetPracticeSettings) -> ModInfo {
    ModInfo::with_settings(ModAcronym::TP, serde_json::to_value(&settings).unwrap())
}

pub fn sudden_death(settings: SuddenDeathSettings) -> ModInfo {
    ModInfo::with_settings(ModAcronym::SD, serde_json::to_value(&settings).unwrap())
}

pub fn grow(settings: ObjectScaleTweenSettings) -> ModInfo {
    ModInfo::with_settings(ModAcronym::GR, serde_json::to_value(&settings).unwrap())
}

pub fn deflate(settings: ObjectScaleTweenSettings) -> ModInfo {
    ModInfo::with_settings(ModAcronym::DF, serde_json::to_value(&settings).unwrap())
}

pub fn spin_in(settings: ObjectScaleTweenSettings) -> ModInfo {
    ModInfo::with_settings(ModAcronym::SI, serde_json::to_value(&settings).unwrap())
}

pub fn double_time(settings: RateAdjustSettings) -> ModInfo {
    ModInfo::with_settings(ModAcronym::DT, serde_json::to_value(&settings).unwrap())
}

pub fn half_time(settings: RateAdjustSettings) -> ModInfo {
    ModInfo::with_settings(ModAcronym::HT, serde_json::to_value(&settings).unwrap())
}

pub fn nightcore(settings: RateAdjustSettings) -> ModInfo {
    ModInfo::with_settings(ModAcronym::NC, serde_json::to_value(&settings).unwrap())
}

pub fn daycore(settings: RateAdjustSettings) -> ModInfo {
    ModInfo::with_settings(ModAcronym::DC, serde_json::to_value(&settings).unwrap())
}

pub fn wind_up(settings: TimeRampSettings) -> ModInfo {
    ModInfo::with_settings(ModAcronym::WU, serde_json::to_value(&settings).unwrap())
}

pub fn wind_down(settings: TimeRampSettings) -> ModInfo {
    ModInfo::with_settings(ModAcronym::WD, serde_json::to_value(&settings).unwrap())
}

pub fn adaptive_speed(settings: AdaptiveSpeedSettings) -> ModInfo {
    ModInfo::with_settings(ModAcronym::AS, serde_json::to_value(&settings).unwrap())
}

pub fn cover(settings: CoverSettings) -> ModInfo {
    ModInfo::with_settings(ModAcronym::CO, serde_json::to_value(&settings).unwrap())
}

pub fn perfect(settings: PerfectSettings) -> ModInfo {
    ModInfo::with_settings(ModAcronym::PF, serde_json::to_value(&settings).unwrap())
}

pub fn taiko_difficulty_adjust(settings: TaikoDifficultyAdjustSettings) -> ModInfo {
    ModInfo::with_settings(ModAcronym::DA, serde_json::to_value(&settings).unwrap())
}

pub fn catch_difficulty_adjust(settings: CatchDifficultyAdjustSettings) -> ModInfo {
    ModInfo::with_settings(ModAcronym::DA, serde_json::to_value(&settings).unwrap())
}

pub fn simplified_rhythm(settings: SimplifiedRhythmSettings) -> ModInfo {
    ModInfo::with_settings(ModAcronym::SR, serde_json::to_value(&settings).unwrap())
}
