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

#[cfg(test)]
mod tests {
    use crate::types::Cond;

    use super::*;

    /// NGを正常に検出できるか
    #[test]
    fn test_basic() {
        let schedule = {
            use Shift::*;
            vec![vec![I, A, K, I, A, K], vec![N, N, N, I, A, K]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();
        schedule_config.staff.ng_list.push((0, 1));

        let score = eval(
            &schedule_config,
            &schedule,
            &mut (CondWrapper::new(Cond::Every), Shift::I, 1.0),
        );

        assert_eq!(1.0, score);
    }

    /// NGの設定に重複がある場合、重複してスコアが計算されるか
    #[test]
    fn test_duplicated() {
        let schedule = {
            use Shift::*;
            vec![vec![I, A, K, I, A, K], vec![N, N, N, I, A, K]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();
        schedule_config.staff.ng_list.push((0, 1));
        schedule_config.staff.ng_list.push((0, 1));

        let score = eval(
            &schedule_config,
            &schedule,
            &mut (CondWrapper::new(Cond::Every), Shift::I, 1.0),
        );

        assert_eq!(2.0, score);
    }
}
