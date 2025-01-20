//! 指定したシフトがDayAttributeで指定した数いない場合に発火するスコア

use super::super::types::{CondWrapper, DayAttributeName, Schedule, ScheduleConfig, Score, Shift};

#[allow(clippy::needless_range_loop)]
pub(super) fn eval(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond, shift, attribute, score): &mut (CondWrapper, Shift, DayAttributeName, Score),
) -> Score {
    let mut sum = 0.0;
    for day in 0..schedule_config.day.count {
        let mut is_valid = false;
        let mut count = 0;
        for staff in 0..schedule_config.staff.count {
            if cond.eval(staff, day, schedule_config) {
                is_valid = true;
                if schedule[staff][day] == *shift {
                    count += 1;
                }
            }
        }
        if is_valid {
            let count_needed = schedule_config.day.attributes.get(attribute).unwrap()[day];
            let d = (count - count_needed).abs() as Score;
            let a = d * *score;
            sum += a * a;
        }
    }
    sum
}
