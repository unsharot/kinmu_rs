//! 指定したシフトが指定した数いない場合に発火するスコア

use super::super::types::{CondWrapper, Schedule, ScheduleConfig, Shift};

use ::kinmu_model::Score;

macro_rules! eval {
    ($eval:ident, $schedule_config:expr, $schedule:expr, $cond:expr, $shift:expr, $count:expr, $score:expr) => {{
        let mut sum = 0.0;
        for day in 0..$schedule_config.day.count {
            let mut is_valid = false;
            let mut staff_count = 0;
            for staff in 0..$schedule_config.staff.count {
                if $cond.$eval(staff, day, $schedule_config) {
                    is_valid = true;
                    if $schedule[staff][day] == *$shift {
                        staff_count += 1;
                    }
                }
            }
            if is_valid {
                let d = (staff_count - *$count).abs() as Score;
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
    (cond, shift, count, score): &mut (CondWrapper, Shift, i32, Score),
) -> Score {
    eval!(
        eval_mut,
        schedule_config,
        schedule,
        cond,
        shift,
        count,
        score
    )
}

#[allow(clippy::needless_range_loop)]
pub(super) fn eval_immut(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond, shift, count, score): &(CondWrapper, Shift, i32, Score),
) -> Score {
    eval!(
        eval_immut,
        schedule_config,
        schedule,
        cond,
        shift,
        count,
        score
    )
}

#[cfg(test)]
mod tests {
    use crate::types::Cond;

    use super::*;

    /// Nが常に1なケース
    #[test]
    fn test_pass() {
        let schedule = {
            use Shift::*;
            vec![vec![O, H, N, N], vec![N, N, O, O]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let score = eval_mut(
            &schedule_config,
            &schedule,
            &mut (CondWrapper::new(Cond::Every), Shift::N, 1, 1.0),
        );

        assert_eq!(0.0, score);
    }

    /// Nが一部0なケース
    #[test]
    fn test_hit_over() {
        let schedule = {
            use Shift::*;
            vec![vec![O, H, N, N], vec![N, K, O, O]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let score = eval_mut(
            &schedule_config,
            &schedule,
            &mut (CondWrapper::new(Cond::Every), Shift::N, 1, 1.0),
        );

        assert_eq!(1.0, score);
    }

    /// Nが一部2なケース
    #[test]
    fn test_hit_lack() {
        let schedule = {
            use Shift::*;
            vec![vec![O, N, N, N], vec![N, N, O, O]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let score = eval_mut(
            &schedule_config,
            &schedule,
            &mut (CondWrapper::new(Cond::Every), Shift::N, 1, 1.0),
        );

        assert_eq!(1.0, score);
    }
}
