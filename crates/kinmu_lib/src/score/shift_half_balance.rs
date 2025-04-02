//! 指定したシフトが月の前後でバランスよく配置されているかを判定するスコア

use super::{
    CondWrapper, DayConfig, DayState, Schedule, ScheduleConfig, StdScoreProp, Shift, ShiftState,
    StaffConfig,
};

use kinmu_input_by_file::Check;
use kinmu_model::{Score, ScorePropTrait};

macro_rules! eval {
    ($eval:ident, $self:expr, $staff_config:expr, $day_config:expr, $schedule:expr) => {{
        let mut sum = 0.0;
        for staff in 0..$staff_config.count {
            let mut is_valid = false;
            // 条件を満たすdayの中から中間を探す
            let mut len = 0;
            for day in 0..$day_config.count {
                if $self.cond.$eval(staff, day, $staff_config, $day_config) {
                    is_valid = true;
                    if $schedule[staff][day] == $self.shift {
                        len += 1;
                    }
                }
            }
            let mid = len / 2;

            let mut cf: i32 = 0;
            let mut cl: i32 = 0;
            let mut i = 0;
            for day in 0..$day_config.count {
                if $self.cond.$eval(staff, day, $staff_config, $day_config) {
                    is_valid = true;
                    if $schedule[staff][day] == $self.shift {
                        i += 1;
                        if i <= mid {
                            cf += 1;
                        } else {
                            cl += 1;
                        }
                    }
                }
            }
            if is_valid {
                let d = (cf - cl).abs() as Score;
                let a = d * $self.score;
                sum += a * a;
            }
        }
        sum
    }};
}

#[derive(Debug, PartialEq, Clone)]
pub struct ShiftHalfBalance {
    pub cond: CondWrapper,
    pub shift: Shift,
    pub score: Score,
}

impl ShiftHalfBalance {
    pub fn new((cond, shift, score): (CondWrapper, Shift, Score)) -> Self {
        Self { cond, shift, score }
    }
}

impl ScorePropTrait<Shift, ShiftState, DayState> for ShiftHalfBalance {
    fn eval_mut(
        &mut self,
        staff_config: &StaffConfig,
        day_config: &DayConfig,
        schedule: &Schedule,
    ) -> Score {
        eval!(eval_mut, self, staff_config, day_config, schedule)
    }

    fn eval_immut(
        &self,
        staff_config: &StaffConfig,
        day_config: &DayConfig,
        schedule: &Schedule,
    ) -> Score {
        eval!(eval_immut, self, staff_config, day_config, schedule)
    }
}

impl Check<StdScoreProp, Shift, ShiftState, DayState> for ShiftHalfBalance {
    fn check(&self, schedule_config: &ScheduleConfig) -> anyhow::Result<()> {
        self.cond.check(schedule_config)
    }
}

#[cfg(test)]
mod tests {
    use crate::Cond;

    use super::super::super::ScheduleConfig;
    use super::*;

    /// 前後で同じ場合
    #[test]
    fn test_eq() {
        let schedule = {
            use Shift::*;
            vec![vec![I, A, K, I, A, K]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let mut sp = ShiftHalfBalance::new((CondWrapper::new(Cond::Every), Shift::I, 1.0));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);

        assert_eq!(0.0, score);
    }

    /// 前後で違う場合
    #[test]
    fn test_neq() {
        let schedule = {
            use Shift::*;
            vec![vec![I, A, K, N, N, K]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let mut sp = ShiftHalfBalance::new((CondWrapper::new(Cond::Every), Shift::I, 1.0));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);

        assert_eq!(1.0, score);
    }
}
