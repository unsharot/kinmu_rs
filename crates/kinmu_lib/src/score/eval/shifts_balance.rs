//! 指定した2つのシフト数がスタッフあたりでバランス良いか判定するスコア

use super::super::{CondWrapper, Schedule, ScheduleConfig, Shift};

use ::kinmu_model::Score;

macro_rules! eval {
    ($eval:ident, $schedule_config:expr, $schedule:expr, $cond:expr, $shift1:expr, $shift2:expr, $score:expr) => {{
        let mut sum = 0.0;
        for staff in 0..$schedule_config.staff.count {
            let mut is_valid = false;
            let mut count1: i32 = 0;
            let mut count2: i32 = 0;
            for day in 0..$schedule_config.day.count {
                if $cond.$eval(staff, day, $schedule_config) {
                    is_valid = true;
                    if $schedule[staff][day] == *$shift1 {
                        count1 += 1;
                    }
                    if $schedule[staff][day] == *$shift2 {
                        count2 += 1;
                    }
                }
            }
            if is_valid {
                let d = (count1 - count2).abs() as Score;
                let a = d * *$score;
                sum += a * a;
            }
        }
        sum
    }};
}

#[allow(clippy::needless_range_loop)]
pub(super) fn eval_mut(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond, shift1, shift2, score): &mut (CondWrapper, Shift, Shift, Score),
) -> Score {
    eval!(
        eval_mut,
        schedule_config,
        schedule,
        cond,
        shift1,
        shift2,
        score
    )
}

#[allow(clippy::needless_range_loop)]
pub(super) fn eval_immut(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond, shift1, shift2, score): &(CondWrapper, Shift, Shift, Score),
) -> Score {
    eval!(
        eval_immut,
        schedule_config,
        schedule,
        cond,
        shift1,
        shift2,
        score
    )
}

#[cfg(test)]
mod tests {
    use crate::Cond;

    use super::*;

    /// 2つのシフトの数が同じ場合
    #[test]
    fn test_eq() {
        let schedule = {
            use Shift::*;
            vec![vec![O, H, N, N]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let score = eval_mut(
            &schedule_config,
            &schedule,
            &mut (CondWrapper::new(Cond::Every), Shift::O, Shift::H, 1.0),
        );

        assert_eq!(0.0, score);
    }

    /// 2つのシフトの数が違う場合
    #[test]
    fn test_neq() {
        let schedule = {
            use Shift::*;
            vec![vec![O, H, N, O]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let score = eval_mut(
            &schedule_config,
            &schedule,
            &mut (CondWrapper::new(Cond::Every), Shift::O, Shift::H, 1.0),
        );

        assert_eq!(1.0, score);
    }
}
