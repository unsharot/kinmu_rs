//! NGリストにあるペアがともに指定したシフトなら発火するスコア

use super::super::{CondWrapper, DayConfig, Schedule, Shift, StaffConfig};

use ::kinmu_model::Score;

macro_rules! eval {
    ($eval:ident, $staff_config:expr, $day_config:expr, $schedule:expr, $cond:expr, $shift:expr, $score:expr) => {{
        let mut sum = 0.0;
        for day in 0..$day_config.count {
            let mut a = 0.0;
            for i in 0..$staff_config.ng_list.len() {
                let (staff1, staff2) = $staff_config.ng_list[i];
                if $cond.$eval(staff1, day, $staff_config, $day_config)
                    && $cond.$eval(staff2, day, $staff_config, $day_config)
                    && $schedule[staff1][day] == *$shift
                    && $schedule[staff2][day] == *$shift
                {
                    a += *$score;
                }
            }
            sum += a;
        }
        sum
    }};
}

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

    /// NGを正常に検出できるか
    #[test]
    fn test_basic() {
        let schedule = {
            use Shift::*;
            vec![vec![I, A, K, I, A, K], vec![N, N, N, I, A, K]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();
        schedule_config.staff.ng_list.push((0, 1));

        let score = eval_mut(
            &schedule_config.staff,
            &schedule_config.day,
            &schedule,
            &mut (CondWrapper::new(Cond::Every), Shift::I, 1.0),
        );

        assert_eq!(1.0, score);
    }

    /// NGの設定に重複がある場合、重複してスコアが計算されるか
    #[test]
    fn test_duplicated() {
        let schedule = {
            use Shift::*;
            vec![vec![I, A, K, I, A, K], vec![N, N, N, I, A, K]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();
        schedule_config.staff.ng_list.push((0, 1));
        schedule_config.staff.ng_list.push((0, 1));

        let score = eval_mut(
            &schedule_config.staff,
            &schedule_config.day,
            &schedule,
            &mut (CondWrapper::new(Cond::Every), Shift::I, 1.0),
        );

        assert_eq!(2.0, score);
    }
}
