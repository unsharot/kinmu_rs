//! 指定回数以上同じペアなら発火するスコア

use super::super::types::{CondWrapper, Schedule, ScheduleConfig, Score, Shift};

use std::collections::HashMap;

#[allow(clippy::needless_range_loop)]
pub(super) fn eval(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond, pair_limit, shift, score): &mut (CondWrapper, i32, Shift, Score),
) -> Score {
    let mut pair_map: HashMap<Vec<usize>, i32> = HashMap::new();
    for day in 0..schedule_config.day.count {
        let mut i_list: Vec<usize> = Vec::new();
        for staff in 0..schedule_config.staff.count {
            if cond.eval(staff, day, schedule_config) && schedule[staff][day] == *shift {
                i_list.push(staff);
            }
        }
        // ある日の夜勤の人数が2人以上ならペアのマップに加算
        if i_list.len() >= 2 {
            *pair_map.entry(i_list).or_insert(0) += 1;
        }
    }
    let mut ans = 0.0;
    for count in pair_map.values() {
        let a = *count - *pair_limit + 1;
        if a > 0 {
            ans += (a as Score) * *score
        }
    }
    ans
}
