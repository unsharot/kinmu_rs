//! 指定した2つのシフト数がスタッフあたりでバランス良いか判定するスコア

use super::super::types::{CondWrapper, Schedule, ScheduleConfig, Score, Shift};

#[allow(clippy::needless_range_loop)]
pub(super) fn eval(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond, shift1, shift2, score): &mut (CondWrapper, Shift, Shift, Score),
) -> Score {
    let mut sum = 0.0;
    for day in 0..schedule_config.day.count {
        let mut is_valid = false;
        let mut count1: i32 = 0;
        let mut count2: i32 = 0;
        for staff in 0..schedule_config.staff.count {
            if cond.eval(staff, day, schedule_config) {
                is_valid = true;
                if schedule[staff][day] == *shift1 {
                    count1 += 1;
                }
                if schedule[staff][day] == *shift2 {
                    count2 += 1;
                }
            }
        }
        if is_valid {
            let d = (count1 - count2).abs() as Score;
            let a = d * *score;
            sum += a * a;
        }
    }
    sum
}
