//! 指定したシフトパターンの数に応じて発火するスコア
//! 計算量はO(NM)
//! TODO: RollingHash、FSMやTrie木を用いた高速化

use super::super::{CondWrapper, DayConfig, Schedule, Shift, StaffConfig};

use ::kinmu_model::Score;

macro_rules! eval {
    ($eval:ident, $staff_config:expr, $day_config:expr, $schedule:expr, $cond:expr, $shift_pattern:expr, $score:expr) => {{
        let mut sum = 0.0;
        for staff in 0..$staff_config.count {
            let mut a = 0.0;
            for day in 0..$day_config.count {
                let mut hit = true;
                let mut is_valid = false;
                for dd in 0..$shift_pattern.len() {
                    if $day_config.count <= day + dd {
                        hit = false;
                        break;
                    } else if $cond.$eval(staff, day + dd, $staff_config, $day_config) {
                        is_valid = true;
                        if $shift_pattern[dd] != $schedule[staff][day + dd] {
                            hit = false;
                            break;
                        }
                    } else {
                        hit = false;
                        break;
                    }
                }
                if hit && is_valid {
                    a += *$score;
                }
            }
            sum += a;
        }
        sum
    }};
}

#[allow(clippy::needless_range_loop)]
pub(super) fn eval_mut(
    staff_config: &StaffConfig,
    day_config: &DayConfig,
    schedule: &Schedule,
    (cond, shift_pattern, score): &mut (CondWrapper, Vec<Shift>, Score),
) -> Score {
    eval!(
        eval_mut,
        staff_config,
        day_config,
        schedule,
        cond,
        shift_pattern,
        score
    )
}

#[allow(clippy::needless_range_loop)]
pub(super) fn eval_immut(
    staff_config: &StaffConfig,
    day_config: &DayConfig,
    schedule: &Schedule,
    (cond, shift_pattern, score): &(CondWrapper, Vec<Shift>, Score),
) -> Score {
    eval!(
        eval_immut,
        staff_config,
        day_config,
        schedule,
        cond,
        shift_pattern,
        score
    )
}

#[cfg(test)]
mod tests {
    use crate::Cond;

    use super::super::super::ScheduleConfig;
    use super::*;

    /// ヒットするべきでないパターン
    #[test]
    fn test_pass() {
        let schedule = {
            use Shift::*;
            vec![vec![O, O, K, H, A, K], vec![N, N, O, I, H, K]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let score = eval_mut(
            &schedule_config.staff,
            &schedule_config.day,
            &schedule,
            &mut (CondWrapper::new(Cond::Every), vec![Shift::O, Shift::H], 1.0),
        );

        assert_eq!(0.0, score);
    }

    /// OHパターンの検出
    #[test]
    fn test_hit() {
        let schedule = {
            use Shift::*;
            vec![vec![O, O, K, H, A, K], vec![N, N, O, H, A, K]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let score = eval_mut(
            &schedule_config.staff,
            &schedule_config.day,
            &schedule,
            &mut (CondWrapper::new(Cond::Every), vec![Shift::O, Shift::H], 1.0),
        );

        assert_eq!(1.0, score);
    }

    /// 2回パターンが存在する場合は2回としてカウントする
    #[test]
    fn test_double() {
        let schedule = {
            use Shift::*;
            vec![vec![O, O, K, H, A, K], vec![N, N, O, H, O, H]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let score = eval_mut(
            &schedule_config.staff,
            &schedule_config.day,
            &schedule,
            &mut (CondWrapper::new(Cond::Every), vec![Shift::O, Shift::H], 1.0),
        );

        assert_eq!(2.0, score);
    }
}
