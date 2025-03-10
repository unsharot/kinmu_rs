//! 指定したシフトをStaffAttributeで指定した数入らなかった場合に発火するスコア

use super::super::{CondWrapper, Schedule, ScheduleConfig, Shift};

use ::kinmu_model::{Score, StaffAttributeName};

macro_rules! eval {
    ($eval:ident, $schedule_config:expr, $schedule:expr, $cond:expr, $shift:expr, $attribute:expr, $score:expr) => {{
        let mut sum = 0.0;
        for staff in 0..$schedule_config.staff.count {
            let mut is_valid = false;
            let mut count = 0;
            for day in 0..$schedule_config.day.count {
                if $cond.$eval(staff, day, $schedule_config) {
                    is_valid = true;
                    if $schedule[staff][day] == *$shift {
                        count += 1;
                    }
                }
            }
            if is_valid {
                let count_needed = $schedule_config.get_attribute(staff, $attribute);
                if count_needed != -1 {
                    // 値が-1の場合、任意の数を許すためスコアを増やさない
                    let d = (count - count_needed).abs() as Score;
                    let a = d * *$score;
                    sum += a * a;
                }
            }
        }
        sum
    }};
}

#[allow(clippy::needless_range_loop)]
pub(super) fn eval_mut(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond, shift, attribute, score): &mut (CondWrapper, Shift, StaffAttributeName, Score),
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
    (cond, shift, attribute, score): &(CondWrapper, Shift, StaffAttributeName, Score),
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

    use ::kinmu_model::Staff;

    use super::*;

    /// ヒットするべきでないパターン
    #[test]
    fn test_pass() {
        let schedule = {
            use Shift::*;
            vec![vec![I, A, K, N]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();
        schedule_config.staff.list.push(Staff {
            name: String::from(""),
            attributes: vec![1],
        });
        schedule_config
            .staff
            .attribute_map
            .names
            .push(String::from("n_count"));
        schedule_config
            .staff
            .attribute_map
            .name_to_index
            .insert(String::from("n_count"), 0);

        let score = eval_mut(
            &schedule_config,
            &schedule,
            &mut (
                CondWrapper::new(Cond::Every),
                Shift::N,
                String::from("n_count"),
                1.0,
            ),
        );

        assert_eq!(0.0, score);
    }

    /// ヒットするパターン
    #[test]
    fn test_hit() {
        let schedule = {
            use Shift::*;
            vec![vec![I, A, N, N]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();
        schedule_config.staff.list.push(Staff {
            name: String::from(""),
            attributes: vec![1],
        });
        schedule_config
            .staff
            .attribute_map
            .names
            .push(String::from("n_count"));
        schedule_config
            .staff
            .attribute_map
            .name_to_index
            .insert(String::from("n_count"), 0);

        let score = eval_mut(
            &schedule_config,
            &schedule,
            &mut (
                CondWrapper::new(Cond::Every),
                Shift::N,
                String::from("n_count"),
                1.0,
            ),
        );

        assert_eq!(1.0, score);
    }
}
