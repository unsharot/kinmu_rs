//! 指定したシフトパターンの数に応じて発火するスコア
//! ただし、シフトパターンは複数候補を指定可能
//! TODO: HashMapやTrie木を用いた高速化

use super::super::types::{CondWrapper, Schedule, ScheduleConfig, Score, Shift};

#[allow(clippy::needless_range_loop)]
pub(super) fn eval(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond, shift_pattern, score): &mut (CondWrapper, Vec<Vec<Shift>>, Score),
) -> Score {
    let mut sum = 0.0;
    for staff in 0..schedule_config.staff.count {
        let mut a = 0.0;
        for day in 0..schedule_config.day.count {
            let mut hit = true;
            let mut is_valid = false;
            for dd in 0..shift_pattern.len() {
                if schedule_config.day.count <= day + dd {
                    hit = false;
                    break;
                } else if cond.eval(staff, day + dd, schedule_config) {
                    is_valid = true;
                    if !(shift_pattern[dd].contains(&schedule[staff][day + dd])) {
                        hit = false;
                        break;
                    }
                } else {
                    hit = false;
                    break;
                }
            }
            if hit && is_valid {
                a += *score;
                break;
            }
        }
        sum += a;
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::kinmu_lib::types::Cond;

    use super::*;

    #[test]
    fn general_pattern_test() {
        let mut sc: ScheduleConfig = Default::default();
        sc.staff.count = 1;
        sc.day.count = 4;

        let score = eval(
            &sc,
            &vec![vec![Shift::H, Shift::H, Shift::A, Shift::Y]],
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

        sc.day.count = 37;

        let score = eval(
            &sc,
            {
                use Shift::*;
                &vec![vec![
                    N, N, K, K, A, N, N, N, N, K, K, N, A, A, N, K, K, K, N, N, A, A, A, K, K, N,
                    N, N, N, N, K, K, A, N, A, N, A,
                ]]
            },
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
