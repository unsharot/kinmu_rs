//! 指定したシフトが月の前後どちらにあるほうが良いか設定する
//! Scoreのフィールドが正なら前を優先、負なら後ろを優先

use super::super::types::{CondWrapper, Schedule, ScheduleConfig, Shift};

use ::kinmu_model::Score;

macro_rules! eval {
    ($eval:ident, $schedule_config:expr, $schedule:expr, $cond:expr, $shift:expr, $score:expr) => {{
        let mut sum = 0.0;
        for staff in 0..$schedule_config.staff.count {
            let mut is_valid = false;
            // 条件を満たすdayの中から中間を探す
            let mut len = 0;
            for day in 0..$schedule_config.day.count {
                if $cond.$eval(staff, day, $schedule_config) {
                    is_valid = true;
                    if $schedule[staff][day] == *$shift {
                        len += 1;
                    }
                }
            }
            let mid = (len as Score) / 2.0 - 0.5;

            let mut a = 0.0;
            let mut i = 0;
            for day in 0..$schedule_config.day.count {
                if $cond.$eval(staff, day, $schedule_config) {
                    is_valid = true;
                    if $schedule[staff][day] == *$shift {
                        i += 1;
                        a += *$score * ((i as Score) - (mid as Score));
                    }
                }
            }
            if is_valid {
                sum += a;
            }
        }
        sum
    }};
}

#[allow(clippy::needless_range_loop)]
pub(super) fn eval_mut(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond, shift, score): &mut (CondWrapper, Shift, Score),
) -> Score {
    eval!(eval_mut, schedule_config, schedule, cond, shift, score)
}

#[allow(clippy::needless_range_loop)]
pub(super) fn eval_immut(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond, shift, score): &(CondWrapper, Shift, Score),
) -> Score {
    eval!(eval_immut, schedule_config, schedule, cond, shift, score)
}

#[cfg(test)]
mod tests {
    use crate::types::Cond;

    use super::*;

    /// 前を優先
    #[test]
    fn test_front() {
        let schedule = {
            use Shift::*;
            vec![vec![I, A, K, I, A, K]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let score = eval_mut(
            &schedule_config,
            &schedule,
            &mut (CondWrapper::new(Cond::Every), Shift::I, 1.0),
        );

        assert_eq!(2.0, score);
    }

    /// 後ろを優先
    #[test]
    fn test_back() {
        let schedule = {
            use Shift::*;
            vec![vec![I, A, K, I, A, K]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let score = eval_mut(
            &schedule_config,
            &schedule,
            &mut (CondWrapper::new(Cond::Every), Shift::I, -1.0),
        );

        assert_eq!(-2.0, score);
    }
}
