//! 指定したパターンが存在するスタッフに対して発火するスコア
//! ただし、パターンは固定
//! 計算量はO(NM)
//! TODO: RollingHash、FSMやTrie木を用いた高速化

use super::super::types::{CondWrapper, Schedule, ScheduleConfig, Shift};

use ::kinmu_model::Score;

macro_rules! eval {
    ($eval:ident, $schedule_config:expr, $schedule:expr, $cond:expr, $shift_pattern:expr, $score:expr) => {{
        let mut sum = 0.0;
        for staff in 0..$schedule_config.staff.count {
            let mut any = false;
            for day in 0..$schedule_config.day.count {
                let mut hit = true;
                let mut is_valid = false;
                for dd in 0..$shift_pattern.len() {
                    if $schedule_config.day.count <= day + dd {
                        hit = false;
                        break;
                    } else if $cond.$eval(staff, day + dd, $schedule_config) {
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
                    any = true;
                    break;
                }
            }
            if any {
                sum += *$score;
            }
        }
        sum
    }};
}

#[allow(clippy::needless_range_loop)]
pub(super) fn eval_mut(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond, shift_pattern, score): &mut (CondWrapper, Vec<Shift>, Score),
) -> Score {
    eval!(
        eval_mut,
        schedule_config,
        schedule,
        cond,
        shift_pattern,
        score
    )
}

#[allow(clippy::needless_range_loop)]
pub(super) fn eval_immut(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond, shift_pattern, score): &(CondWrapper, Vec<Shift>, Score),
) -> Score {
    eval!(
        eval_immut,
        schedule_config,
        schedule,
        cond,
        shift_pattern,
        score
    )
}

#[cfg(test)]
mod tests {
    use crate::types::Cond;

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
            &schedule_config,
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
            &schedule_config,
            &schedule,
            &mut (CondWrapper::new(Cond::Every), vec![Shift::O, Shift::H], 1.0),
        );

        assert_eq!(1.0, score);
    }

    /// 2回パターンが存在する場合も1回としてカウントする
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
            &schedule_config,
            &schedule,
            &mut (CondWrapper::new(Cond::Every), vec![Shift::O, Shift::H], 1.0),
        );

        assert_eq!(1.0, score);
    }
}
