//! 指定したパターンが存在するスタッフに対して発火するスコア
//! 計算量はO(NM)
//! TODO: RollingHash、FSMやTrie木を用いた高速化

use super::super::{CondWrapper, Schedule, ScheduleConfig, Shift};

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
                        if !($shift_pattern[dd].contains(&$schedule[staff][day + dd])) {
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
    (cond, shift_pattern, score): &mut (CondWrapper, Vec<Vec<Shift>>, Score),
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
    (cond, shift_pattern, score): &(CondWrapper, Vec<Vec<Shift>>, Score),
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
    use crate::Cond;

    use super::*;

    /// ヒットするべきでないパターン
    #[test]
    fn test_pass_with_cond() {
        let schedule = {
            use Shift::*;
            &vec![vec![
                N, K, K, K, O, I, A, K, H, O, K, H, N, I, A, K, H, I, A, K, O, N, I, A, K, N, O, N,
                K, I, A, K, H, I, A, K, O,
            ]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.day.buffer_count = 3;
        schedule_config.staff.count = 1;

        let score = eval_mut(
            &schedule_config,
            &schedule,
            &mut (
                CondWrapper::new(Cond::DayExceptBuffer),
                vec![vec![Shift::K, Shift::Y], vec![Shift::K, Shift::Y]],
                -1000.0,
            ),
        );
        assert_eq!(0.0, score);
    }

    /// ヒットするパターン
    #[test]
    fn test_hit_with_cond() {
        let schedule = {
            use Shift::*;
            &vec![vec![
                N, K, K, K, O, I, A, K, H, O, K, H, N, I, A, K, H, I, A, K, O, N, I, A, K, N, O, N,
                K, I, A, K, H, I, A, K, O,
            ]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.day.buffer_count = 3;
        schedule_config.staff.count = 1;

        let score = eval_mut(
            &schedule_config,
            &schedule,
            &mut (
                CondWrapper::new(Cond::Every),
                vec![vec![Shift::K, Shift::Y], vec![Shift::K, Shift::Y]],
                -1000.0,
            ),
        );
        assert_eq!(-1000.0, score);
    }
}
