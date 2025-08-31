//! 焼きなましで使う更新関数のモジュール

use super::{ScheduleConfig, ScheduleState};

mod iak_safe;
mod iaknoh;
mod iaknoh_repeat;
mod noh_repeat;
mod range;
mod range_or_iaknoh_repeat;
mod range_or_noh_repeat;

use iak_safe::IAKSafe;

/// 生成器で用いる更新関数のための型
/// GeneratorWithAnnealingのUpdateを実装
#[derive(Debug, Clone)]
pub enum StdUpdate {
    UpdateIAKNOHRepeat,
    UpdateIAKNOH,
    UpdateNOHRepeat,
    UpdateIAKSafe(IAKSafe),
    UpdateRange,
    UpdateRangeOrNOHRepeat,
    UpdateRangeOrIAKNOHRepeat,
}

pub fn load_update(
    s: &str,
    schedule_config: ScheduleConfig,
    schedule_state: ScheduleState,
) -> anyhow::Result<StdUpdate> {
    let words: Vec<&str> = s.splitn(2, ' ').collect();
    anyhow::ensure!(words.len() >= 2, "Needs 2 fields, but not enough.");
    anyhow::ensure!(2 >= words.len(), "Needs 2 fields, but too much given.");
    match (words[0], words[1]) {
        ("UpdateIAKNOHRepeat", _p) => Ok(StdUpdate::UpdateIAKNOHRepeat),
        ("UpdateIAKNOH", _p) => Ok(StdUpdate::UpdateIAKNOH),
        ("UpdateNOHRepeat", _p) => Ok(StdUpdate::UpdateNOHRepeat),
        ("UpdateIAKSafe", _p) => Ok(StdUpdate::UpdateIAKSafe(IAKSafe {
            schedule_config,
            schedule_state,
        })),
        ("UpdateRange", _p) => Ok(StdUpdate::UpdateRange),
        ("UpdateRangeOrNOHRepeat", _p) => Ok(StdUpdate::UpdateRangeOrNOHRepeat),
        ("UpdateRangeOrIAKNOHRepeat", _p) => Ok(StdUpdate::UpdateRangeOrIAKNOHRepeat),
        _ => Err(anyhow::anyhow!("Failed to parse update function {}", s)),
    }
}
