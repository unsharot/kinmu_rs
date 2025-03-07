//! 指定したシフトが月の前後でバランスよく配置されているかを判定するスコア

use super::super::{CondWrapper, DayConfig, Schedule, Shift, StaffConfig};

use ::kinmu_model::Score;

macro_rules! eval {
    ($eval:ident, $staff_config:expr, $day_config:expr, $schedule:expr, $cond:expr, $shift:expr, $score:expr) => {{
        let mut sum = 0.0;
        for staff in 0..$staff_config.count {
            let mut is_valid = false;
            // 条件を満たすdayの中から中間を探す
            let mut len = 0;
            for day in 0..$day_config.count {
                if $cond.$eval(staff, day, $staff_config, $day_config) {
                    is_valid = true;
                    if $schedule[staff][day] == *$shift {
                        len += 1;
                    }
                }
            }
            let mid = len / 2;

            let mut cf: i32 = 0;
            let mut cl: i32 = 0;
            let mut i = 0;
            for day in 0..$day_config.count {
                if $cond.$eval(staff, day, $staff_config, $day_config) {
                    is_valid = true;
                    if $schedule[staff][day] == *$shift {
                        i += 1;
                        if i <= mid {
                            cf += 1;
                        } else {
                            cl += 1;
                        }
                    }
                }
            }
            if is_valid {
                let d = (cf - cl).abs() as Score;
                let a = d * *$score;
                sum += a * a;
            }
        }
        sum
    }};
}

#[allow(clippy::needless_range_loop)]
pub(super) fn eval_mut(
    staff_config: &StaffConfig,
    day_config: &DayConfig,
    schedule: &Schedule,
    (cond, shift, score): &mut (CondWrapper, Shift, Score),
) -> Score {
    eval!(
        eval_mut,
        staff_config,
        day_config,
        schedule,
        cond,
        shift,
        score
    )
}

#[allow(clippy::needless_range_loop)]
pub(super) fn eval_immut(
    staff_config: &StaffConfig,
    day_config: &DayConfig,
    schedule: &Schedule,
    (cond, shift, score): &(CondWrapper, Shift, Score),
) -> Score {
    eval!(
        eval_immut,
        staff_config,
        day_config,
        schedule,
        cond,
        shift,
        score
    )
}

#[cfg(test)]
mod tests {
    use crate::Cond;

    use super::super::super::ScheduleConfig;
    use super::*;

    /// 前後で同じ場合
    #[test]
    fn test_eq() {
        let schedule = {
            use Shift::*;
            vec![vec![I, A, K, I, A, K]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let score = eval_mut(
            &schedule_config.staff,
            &schedule_config.day,
            &schedule,
            &mut (CondWrapper::new(Cond::Every), Shift::I, 1.0),
        );

        assert_eq!(0.0, score);
    }

    /// 前後で違う場合
    #[test]
    fn test_neq() {
        let schedule = {
            use Shift::*;
            vec![vec![I, A, K, N, N, K]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let score = eval_mut(
            &schedule_config.staff,
            &schedule_config.day,
            &schedule,
            &mut (CondWrapper::new(Cond::Every), Shift::I, 1.0),
        );

        assert_eq!(1.0, score);
    }
}
