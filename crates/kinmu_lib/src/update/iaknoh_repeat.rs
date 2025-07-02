use super::super::{Schedule, ScheduleConfig, ScheduleState, Shift, ShiftState};

use rand::Rng;

/// ランダムな1つの枠をランダムな枠に変える
/// Absoluteの場合繰り返す
pub fn update_iaknoh_repeat<R: Rng>(
    schedule_config: &ScheduleConfig,
    schedule_state: &ScheduleState,
    schedule: &Schedule,
    rng: &mut R,
) -> Schedule {
    let mut new_schedule = schedule.clone();
    let rx: usize = rng.gen_range(0..schedule_config.staff.count);
    let ry: usize = rng.gen_range(schedule_config.day.buffer_count..schedule_config.day.count);
    if schedule_state[rx][ry] != ShiftState::Absolute {
        new_schedule[rx][ry] =
            [Shift::N, Shift::K, Shift::I, Shift::A, Shift::O, Shift::H][rng.gen_range(0..6)];
        new_schedule
    } else {
        update_iaknoh_repeat(schedule_config, schedule_state, schedule, rng)
    }
}
