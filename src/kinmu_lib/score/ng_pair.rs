//! NGリストにあるペアがともに指定したシフトなら発火するスコア

use super::super::types::{CondWrapper, Schedule, ScheduleConfig, Score, Shift};

pub(super) fn eval(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond, shift, score): &mut (CondWrapper, Shift, Score),
) -> Score {
    let mut sum = 0.0;
    for day in 0..schedule_config.day.count {
        let mut a = 0.0;
        for i in 0..schedule_config.staff.ng_list.len() {
            let (staff1, staff2) = schedule_config.staff.ng_list[i];
            if cond.eval(staff1, day, schedule_config)
                && cond.eval(staff2, day, schedule_config)
                && schedule[staff1][day] == *shift
                && schedule[staff2][day] == *shift
            {
                a += *score;
            }
        }
        sum += a;
    }
    sum
}
