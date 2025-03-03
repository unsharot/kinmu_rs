//! 指定したシフトが指定回数連続して存在するか判定するスコア
//! 指定回数+1回連続は1回分としてカウントされる

use super::super::{CondWrapper, Schedule, ScheduleConfig, Shift};

use ::kinmu_model::Score;

macro_rules! eval {
    ($eval:ident, $schedule_config:expr, $schedule:expr, $cond:expr, $target_shifts:expr, $streak_count:expr, $score:expr) => {{
        let mut sum = 0.0;
        for staff in 0..$schedule_config.staff.count {
            let mut a = 0.0;
            let mut accum = 0;
            for day in 0..$schedule_config.day.count {
                if $cond.$eval(staff, day, $schedule_config) {
                    if $target_shifts.contains(&$schedule[staff][day]) {
                        accum += 1;
                    } else {
                        accum = 0;
                    }
                    if accum >= *$streak_count {
                        a += *$score;
                        accum = 0;
                    }
                }
            }
            sum += a;
        }
        sum
    }};
}

#[allow(clippy::needless_range_loop)]
pub(super) fn eval_mut(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond, target_shifts, streak_count, score): &mut (CondWrapper, Vec<Shift>, i32, Score),
) -> Score {
    eval!(
        eval_mut,
        schedule_config,
        schedule,
        cond,
        target_shifts,
        streak_count,
        score
    )
}

#[allow(clippy::needless_range_loop)]
pub(super) fn eval_immut(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond, target_shifts, streak_count, score): &(CondWrapper, Vec<Shift>, i32, Score),
) -> Score {
    eval!(
        eval_immut,
        schedule_config,
        schedule,
        cond,
        target_shifts,
        streak_count,
        score
    )
}

#[cfg(test)]
mod tests {
    use crate::Cond;

    use super::*;

    /// YまたはKが2連続でないことを検知
    #[test]
    fn test_pass() {
        let schedule = {
            use Shift::*;
            vec![vec![N, K, N, N]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let score = eval_mut(
            &schedule_config,
            &schedule,
            &mut (
                CondWrapper::new(Cond::Every),
                vec![Shift::K, Shift::Y],
                2,
                -1.0,
            ),
        );

        assert_eq!(0.0, score);
    }

    /// YまたはKが2連続であることを検知
    #[test]
    fn test_hit() {
        let schedule = {
            use Shift::*;
            vec![vec![N, K, Y, N]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let score = eval_mut(
            &schedule_config,
            &schedule,
            &mut (
                CondWrapper::new(Cond::Every),
                vec![Shift::K, Shift::Y],
                2,
                -1.0,
            ),
        );

        assert_eq!(-1.0, score);
    }
}
