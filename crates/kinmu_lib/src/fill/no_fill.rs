use super::super::{Schedule, ScheduleConfig};

use rand::Rng;

/// 表を埋めない
/// 変更なし
pub fn no_fill<R: Rng>(schedule_config: &ScheduleConfig, _rng: &mut R) -> Schedule {
    schedule_config.day.requested_schedule.clone()
}
