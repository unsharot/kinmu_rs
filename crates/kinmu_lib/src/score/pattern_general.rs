//! 指定したシフトパターンの数に応じて発火するスコア
//! ただし、シフトパターンは複数候補を指定可能
//! 計算量はO(NM)
//! TODO: RollingHash、FSMやTrie木を用いた高速化

use super::super::types::{CondWrapper, Schedule, ScheduleConfig, Shift};

use ::kinmu_model::Score;

macro_rules! eval {
    ($eval:ident, $schedule_config:expr, $schedule:expr, $cond:expr, $shift_pattern:expr, $score:expr) => {{
        let mut sum = 0.0;
        for staff in 0..$schedule_config.staff.count {
            let mut a = 0.0;
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
    use crate::types::Cond;

    use super::*;

    /// 単純な表におけるパターン検出
    #[test]
    fn test_simple() {
        let schedule = vec![vec![Shift::H, Shift::H, Shift::A, Shift::Y]];

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.staff.count = schedule.len();
        schedule_config.day.count = schedule[0].len();

        let score = eval_mut(
            &schedule_config,
            &schedule,
            &mut (
                CondWrapper::new(Cond::Every),
                vec![
                    vec![Shift::N, Shift::O, Shift::H, Shift::A, Shift::K, Shift::Y],
                    vec![Shift::A],
                ],
                1.0,
            ),
        );

        assert_eq!(1.0, score);
    }

    /// 複雑な表における複数パターン検出
    #[test]
    fn test_complex() {
        let schedule = {
            use Shift::*;
            &vec![vec![
                N, N, K, K, A, N, N, N, N, K, K, N, A, A, N, K, K, K, N, N, A, A, A, K, K, N, N, N,
                N, N, K, K, A, N, A, N, A,
            ]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.staff.count = schedule.len();
        schedule_config.day.count = schedule[0].len();

        let score = eval_mut(
            &schedule_config,
            &schedule,
            &mut (
                CondWrapper::new(Cond::Every),
                {
                    use Shift::*;
                    vec![vec![N, O, H, A, K, Y], vec![A]]
                },
                1.0,
            ),
        );

        assert_eq!(9.0, score);
    }
}
