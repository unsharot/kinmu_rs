//! 指定したシフトをStaffAttributeで指定した数入らなかった場合に発火するスコア

use super::super::types::{
    CondWrapper, Schedule, ScheduleConfig, Score, Shift, StaffAttributeName,
};

#[allow(clippy::needless_range_loop)]
pub(super) fn eval(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond, shift, attribute, score): &mut (CondWrapper, Shift, StaffAttributeName, Score),
) -> Score {
    let mut sum = 0.0;
    for staff in 0..schedule_config.staff.count {
        let mut is_valid = false;
        let mut count = 0;
        for day in 0..schedule_config.day.count {
            if cond.eval(staff, day, schedule_config) {
                is_valid = true;
                if schedule[staff][day] == *shift {
                    count += 1;
                }
            }
        }
        if is_valid {
            let count_needed = schedule_config.get_attribute(staff, attribute);
            if count_needed != -1 {
                // 値が-1の場合、任意の数を許すためスコアを増やさない
                let d = (count - count_needed).abs() as Score;
                let a = d * *score;
                sum += a * a;
            }
        }
    }
    sum
}
