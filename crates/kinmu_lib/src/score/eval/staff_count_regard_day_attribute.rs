//! 指定したシフトがDayAttributeで指定した数いない場合に発火するスコア

use super::super::{CondWrapper, Schedule, ScheduleConfig, Shift};

use kinmu_model::{DayAttributeName, Score};

macro_rules! eval {
    ($eval:ident, $schedule_config:expr, $schedule:expr, $cond:expr, $shift:expr, $attribute:expr, $score:expr) => {{
        let mut sum = 0.0;
        for day in 0..$schedule_config.day.count {
            let mut is_valid = false;
            let mut count = 0;
            for staff in 0..$schedule_config.staff.count {
                if $cond.$eval(staff, day, $schedule_config) {
                    is_valid = true;
                    if $schedule[staff][day] == *$shift {
                        count += 1;
                    }
                }
            }
            if is_valid {
                let count_needed = $schedule_config.day.attributes.get($attribute).unwrap()[day];
                let d = (count - count_needed).abs() as Score;
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
    (cond, shift, attribute, score): &mut (CondWrapper, Shift, DayAttributeName, Score),
) -> Score {
    eval!(
        eval_mut,
        schedule_config,
        schedule,
        cond,
        shift,
        attribute,
        score
    )
}

#[allow(clippy::needless_range_loop)]
pub(super) fn eval_immut(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond, shift, attribute, score): &(CondWrapper, Shift, DayAttributeName, Score),
) -> Score {
    eval!(
        eval_immut,
        schedule_config,
        schedule,
        cond,
        shift,
        attribute,
        score
    )
}

#[cfg(test)]
mod tests {
    use crate::Cond;

    use super::*;

    /// Nが指定した数あるケース
    #[test]
    fn test_pass() {
        let schedule = {
            use Shift::*;
            vec![vec![O, N, N, N], vec![N, N, O, O]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();
        schedule_config
            .day
            .attributes
            .insert(String::from("n_staff_count"), vec![1, 2, 1, 1]);

        let score = eval_mut(
            &schedule_config,
            &schedule,
            &mut (
                CondWrapper::new(Cond::Every),
                Shift::N,
                String::from("n_staff_count"),
                1.0,
            ),
        );

        assert_eq!(0.0, score);
    }

    /// Nが指定した数ないケース
    #[test]
    fn test_hit() {
        let schedule = {
            use Shift::*;
            vec![vec![O, N, N, N], vec![N, K, O, O]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();
        schedule_config
            .day
            .attributes
            .insert(String::from("n_staff_count"), vec![1, 2, 1, 1]);

        let score = eval_mut(
            &schedule_config,
            &schedule,
            &mut (
                CondWrapper::new(Cond::Every),
                Shift::N,
                String::from("n_staff_count"),
                1.0,
            ),
        );

        assert_eq!(1.0, score);
    }
}
