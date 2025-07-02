use super::super::{Schedule, ScheduleConfig, ScheduleState};

use super::iaknoh_repeat::update_iaknoh_repeat;
use super::range::update_range;

use rand::Rng;

/// rangeとiaknoh_repeatをランダムに等確率で実行
pub fn update_range_or_iaknoh_repeat<R: Rng>(
    schedule_config: &ScheduleConfig,
    schedule_state: &ScheduleState,
    schedule: &Schedule,
    rng: &mut R,
) -> Schedule {
    let r = rng.gen_range(0..2);
    if r == 0 {
        update_range(schedule_config, schedule_state, schedule, rng)
    } else {
        update_iaknoh_repeat(schedule_config, schedule_state, schedule, rng)
    }
}
