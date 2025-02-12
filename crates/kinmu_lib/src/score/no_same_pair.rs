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

#[cfg(test)]
mod tests {
    use crate::types::Cond;

    use super::*;

    /// 2度同じペアにならない場合、検出しない
    #[test]
    fn test_pass2() {
        let schedule = {
            use Shift::*;
            vec![vec![I, A, K, I, A, K], vec![N, N, N, I, A, K]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let score = eval(
            &schedule_config,
            &schedule,
            &mut (CondWrapper::new(Cond::Every), 2, Shift::I, 1.0),
        );

        assert_eq!(0.0, score);
    }

    /// 2度同じペアになる場合の検出
    #[test]
    fn test_hit2() {
        let schedule = {
            use Shift::*;
            vec![vec![I, A, K, I, A, K], vec![I, N, N, I, A, K]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let score = eval(
            &schedule_config,
            &schedule,
            &mut (CondWrapper::new(Cond::Every), 2, Shift::I, 1.0),
        );

        assert_eq!(1.0, score);
    }

    /// 3度同じペアにならない場合、検出しない
    #[test]
    fn test_pass3() {
        let schedule = {
            use Shift::*;
            vec![vec![I, A, K, I, A, K], vec![I, N, N, I, A, K]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let score = eval(
            &schedule_config,
            &schedule,
            &mut (CondWrapper::new(Cond::Every), 3, Shift::I, 1.0),
        );

        assert_eq!(0.0, score);
    }

    /// 3度同じペアになる場合の検出
    #[test]
    fn test_hit3() {
        let schedule = {
            use Shift::*;
            vec![vec![I, I, K, I, A, K], vec![I, I, N, I, A, K]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let score = eval(
            &schedule_config,
            &schedule,
            &mut (CondWrapper::new(Cond::Every), 3, Shift::I, 1.0),
        );

        assert_eq!(1.0, score);
    }
}
