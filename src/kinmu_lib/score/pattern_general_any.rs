//! 指定したパターンが存在するスタッフに対して発火するスコア
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
        let mut any = false;
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
                any = true;
                break;
            }
        }
        if any {
            sum += *score;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::kinmu_lib::types::Cond;

    use super::*;

    #[test]
    fn general_pattern_any_with_cond() {
        let mut sc: ScheduleConfig = Default::default();
        sc.staff.count = 1;
        sc.day.count = 37;
        sc.day.buffer_count = 3;
        let schedule = {
            use Shift::*;
            &vec![vec![
                N, K, K, K, O, I, A, K, H, O, K, H, N, I, A, K, H, I, A, K, O, N, I, A, K, N, O, N,
                K, I, A, K, H, I, A, K, O,
            ]]
        };

        let score = eval(
            &sc,
            &schedule,
            &mut (
                CondWrapper::new(Cond::DayExceptBuffer),
                vec![vec![Shift::K, Shift::Y], vec![Shift::K, Shift::Y]],
                -1000.0,
            ),
        );
        assert_eq!(0.0, score);

        let score = eval(
            &sc,
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
