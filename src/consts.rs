pub const OSU: i8 = 0;
pub const TAIKO: i8 = 1;
pub const CTB: i8 = 2;
pub const MANIA: i8 = 3;

pub const LEFTCLICK: i32 = 1 << 0;
pub const RIGHTCLICK: i32 = 1 << 1;
pub const KEY1: i32 = 1 << 2;
pub const KEY2: i32 = 1 << 3;
pub const SMOKE: i32 = 1 << 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LazerHitResult {
    None,
    Miss,
    Meh,
    Ok,
    Good,
    Great,
    Perfect,
    SmallTickMiss,
    SmallTickHit,
    LargeTickMiss,
    LargeTickHit,
    SmallBonus,
    LargeBonus,
    IgnoreMiss,
    IgnoreHit,
    ComboBreak,
    SliderTailHit,
    LegacyComboIncrease,
}
