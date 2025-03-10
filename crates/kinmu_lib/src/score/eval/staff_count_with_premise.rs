//! 指定したシフトの人数を満たした日付に対して、指定したシフトが指定した値いない時に発火するスコア

use super::super::{CondWrapper, DayConfig, Schedule, Shift, StaffConfig};

use ::kinmu_model::Score;

macro_rules! eval {
    ($eval:ident, $staff_config:expr, $day_config:expr, $schedule:expr, $cond_premise:expr, $shift_premise:expr, $count_premise:expr, $cond_main:expr, $shift_main:expr, $count_main:expr, $score:expr) => {{
        let mut sum = 0.0;
        for day in 0..$day_config.count {
            let mut count = 0;
            for staff in 0..$staff_config.count {
                if $cond_premise.$eval(staff, day, $staff_config, $day_config)
                    && $schedule[staff][day] == *$shift_premise
                {
                    count += 1;
                }
            }
            if count == *$count_premise {
                let mut is_valid = false;
                let mut count2 = 0;
                for staff in 0..$staff_config.count {
                    if $cond_main.$eval(staff, day, $staff_config, $day_config) {
                        is_valid = true;
                        if $schedule[staff][day] == *$shift_main {
                            count2 += 1;
                        }
                    }
                }
                if is_valid {
                    let d = (count2 - *$count_main).abs() as Score;
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
    staff_config: &StaffConfig,
    day_config: &DayConfig,
    schedule: &Schedule,
    (cond_premise, shift_premise, count_premise, cond_main, shift_main, count_main, score): &mut (
        CondWrapper,
        Shift,
        i32,
        CondWrapper,
        Shift,
        i32,
        Score,
    ),
) -> Score {
    eval!(
        eval_mut,
        staff_config,
        day_config,
        schedule,
        cond_premise,
        shift_premise,
        count_premise,
        cond_main,
        shift_main,
        count_main,
        score
    )
}

#[allow(clippy::needless_range_loop)]
pub(super) fn eval_immut(
    staff_config: &StaffConfig,
    day_config: &DayConfig,
    schedule: &Schedule,
    (cond_premise, shift_premise, count_premise, cond_main, shift_main, count_main, score): &(
        CondWrapper,
        Shift,
        i32,
        CondWrapper,
        Shift,
        i32,
        Score,
    ),
) -> Score {
    eval!(
        eval_immut,
        staff_config,
        day_config,
        schedule,
        cond_premise,
        shift_premise,
        count_premise,
        cond_main,
        shift_main,
        count_main,
        score
    )
}

#[cfg(test)]
mod tests {
    use crate::Cond;

    use ::kinmu_model::Staff;

    use super::super::super::ScheduleConfig;
    use super::*;

    /// Iを一人で担当する人にその資格がある場合
    #[test]
    fn test_pass() {
        let schedule = {
            use Shift::*;
            vec![vec![I, N, N, I], vec![N, N, O, I]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();
        schedule_config.staff.list.push(Staff {
            name: Default::default(),
            attributes: vec![1],
        });
        schedule_config.staff.list.push(Staff {
            name: Default::default(),
            attributes: vec![0],
        });
        schedule_config
            .staff
            .attribute_map
            .names
            .push(String::from("I_alone_ok"));
        schedule_config
            .staff
            .attribute_map
            .name_to_index
            .insert(String::from("I_alone_ok"), 0);

        let score = eval_mut(
            &schedule_config.staff,
            &schedule_config.day,
            &schedule,
            &mut (
                CondWrapper::new(Cond::Every),
                Shift::I,
                1,
                CondWrapper::new(Cond::StaffWithAttribute((String::from("I_alone_ok"), 1))),
                Shift::I,
                1,
                1.0,
            ),
        );

        assert_eq!(0.0, score);
    }

    /// Iを一人で担当する人にその資格がない場合
    #[test]
    fn test_hit() {
        let schedule = {
            use Shift::*;
            vec![vec![N, N, N, I], vec![I, N, O, I]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();
        schedule_config.staff.list.push(Staff {
            name: Default::default(),
            attributes: vec![1],
        });
        schedule_config.staff.list.push(Staff {
            name: Default::default(),
            attributes: vec![0],
        });
        schedule_config
            .staff
            .attribute_map
            .names
            .push(String::from("I_alone_ok"));
        schedule_config
            .staff
            .attribute_map
            .name_to_index
            .insert(String::from("I_alone_ok"), 0);

        let score = eval_mut(
            &schedule_config.staff,
            &schedule_config.day,
            &schedule,
            &mut (
                CondWrapper::new(Cond::Every),
                Shift::I,
                1,
                CondWrapper::new(Cond::StaffWithAttribute((String::from("I_alone_ok"), 1))),
                Shift::I,
                1,
                1.0,
            ),
        );

        assert_eq!(1.0, score);
    }
}
