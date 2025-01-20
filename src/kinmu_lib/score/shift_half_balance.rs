//! 指定したシフトが月の前後でバランスよく配置されているかを判定するスコア

use super::super::types::{CondWrapper, Schedule, ScheduleConfig, Score, Shift};

#[allow(clippy::needless_range_loop)]
pub(super) fn eval(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond, shift, score): &mut (CondWrapper, Shift, Score),
) -> Score {
    let mut sum = 0.0;
    for staff in 0..schedule_config.staff.count {
        let mut is_valid = false;
        // 条件を満たすdayの中から中間を探す
        let mut len = 0;
        for day in 0..schedule_config.day.count {
            if cond.eval(staff, day, schedule_config) {
                is_valid = true;
                if schedule[staff][day] == *shift {
                    len += 1;
                }
            }
        }
        let mid = len / 2;

        let mut cf: i32 = 0;
        let mut cl: i32 = 0;
        let mut i = 0;
        for day in 0..schedule_config.day.count {
            if cond.eval(staff, day, schedule_config) {
                is_valid = true;
                if schedule[staff][day] == *shift {
                    i += 1;
                    if i < mid {
                        cf += 1;
                    } else {
                        cl += 1;
                    }
                }
            }
        }
        if is_valid {
            let d = (cf - cl).abs() as Score;
            let a = d * *score;
            sum += a * a;
        }
    }
    sum
}
