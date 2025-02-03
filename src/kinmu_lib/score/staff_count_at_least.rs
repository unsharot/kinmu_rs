//! 指定したシフトが指定した数より少ない場合に発火するスコア

use super::super::types::{CondWrapper, Schedule, ScheduleConfig, Score, Shift};

#[allow(clippy::needless_range_loop)]
pub(super) fn eval(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond, shift, count, score): &mut (CondWrapper, Shift, i32, Score),
) -> Score {
    let mut sum = 0.0;
    for day in 0..schedule_config.day.count {
        let mut is_valid = false;
        let mut staff_count = 0;
        for staff in 0..schedule_config.staff.count {
            if cond.eval(staff, day, schedule_config) {
                is_valid = true;
                if schedule[staff][day] == *shift {
                    staff_count += 1;
                }
            }
        }
        if is_valid {
            let d = std::cmp::min(staff_count - *count, 0) as Score;
            let a = d * *score;
            sum += a * a;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::kinmu_lib::types::Cond;

    use super::*;

    /// Nが最低1つあるケース
    #[test]
    fn test_pass() {
        let schedule = {
            use Shift::*;
            vec![vec![O, H, N, N], vec![N, N, O, O]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let score = eval(
            &schedule_config,
            &schedule,
            &mut (CondWrapper::new(Cond::Every), Shift::N, 1, 1.0),
        );

        assert_eq!(0.0, score);
    }

    /// Nが最低1つないケース
    #[test]
    fn test_hit() {
        let schedule = {
            use Shift::*;
            vec![vec![O, H, N, N], vec![N, K, O, O]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let score = eval(
            &schedule_config,
            &schedule,
            &mut (CondWrapper::new(Cond::Every), Shift::N, 1, 1.0),
        );

        assert_eq!(1.0, score);
    }
}
