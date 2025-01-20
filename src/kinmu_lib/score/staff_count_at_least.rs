//! 指定したシフトが指定した数より少ない場合に発火するスコア

use super::super::types::{CondWrapper, Schedule, ScheduleConfig, Score, Shift};

#[allow(clippy::needless_range_loop)]
pub(super) fn eval(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond, shift, count, score): &mut (CondWrapper, Shift, i32, Score),
) -> Score {
    let mut sum = 0.0;
    for day in 0..schedule_config.day.count {
        let mut is_valid = false;
        let mut staff_count = 0;
        for staff in 0..schedule_config.staff.count {
            if cond.eval(staff, day, schedule_config) {
                is_valid = true;
                if schedule[staff][day] == *shift {
                    staff_count += 1;
                }
            }
        }
        if is_valid {
            let d = std::cmp::min(staff_count - *count, 0) as Score;
            let a = d * *score;
            sum += a * a;
        }
    }
    sum
}
