//! 指定したシフトの人数を満たした日付に対して、指定したシフトが指定した値いない時に発火するスコア

use super::super::types::{CondWrapper, Schedule, ScheduleConfig, Score, Shift};

#[allow(clippy::needless_range_loop)]
pub(super) fn eval(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond_premise, shift_premise, count_premise, cond_main, shift_main, count_main, score): &mut (
        CondWrapper,
        Shift,
        i32,
        CondWrapper,
        Shift,
        i32,
        Score,
    ),
) -> Score {
    let mut sum = 0.0;
    for day in 0..schedule_config.day.count {
        let mut count = 0;
        for staff in 0..schedule_config.staff.count {
            if cond_premise.eval(staff, day, schedule_config)
                && schedule[staff][day] == *shift_premise
            {
                count += 1;
            }
        }
        if count == *count_premise {
            let mut is_valid = false;
            let mut count2 = 0;
            for staff in 0..schedule_config.staff.count {
                if cond_main.eval(staff, day, schedule_config) {
                    is_valid = true;
                    if schedule[staff][day] == *shift_main {
                        count2 += 1;
                    }
                }
            }
            if is_valid {
                let d = (count2 - *count_main).abs() as Score;
                let a = d * *score;
                sum += a * a;
            }
        }
    }
    sum
}
