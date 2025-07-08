use super::super::{Schedule, ScheduleConfig, Shift, ShiftState};

use rand::Rng;

/// 表をN, O, Hのいずれかでランダムに埋める
/// Uになっている枠のみ埋める
/// Absoluteなら埋めない
#[allow(clippy::needless_range_loop)]
pub fn fill_noh<R: Rng>(schedule_config: &ScheduleConfig, rng: &mut R) -> Schedule {
    let mut schedule = schedule_config.day.requested_schedule.clone();
    for r in 0..schedule_config.staff.count {
        for c in schedule_config.day.buffer_count..schedule_config.day.count {
            if schedule_config.day.schedule_states[r][c] != ShiftState::Absolute
                && schedule[r][c] == Shift::U
            {
                schedule[r][c] = [Shift::N, Shift::O, Shift::H][rng.gen_range(0..3)];
            }
        }
    }
    schedule
}
