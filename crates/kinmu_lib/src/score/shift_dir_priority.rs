//! 指定したシフトが月の前後どちらにあるほうが良いか設定する
//! Scoreのフィールドが正なら前を優先、負なら後ろを優先

use super::{
    CondWrapper, DayConfig, DayState, Schedule, ScheduleConfig, Shift, ShiftState, StaffConfig,
    StdScoreProp,
};

use kinmu_input_by_file::Check;
use kinmu_model::{Score, ScoreProp};

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
            let mid = (len as Score) / 2.0 - 0.5;

            let mut a = 0.0;
            let mut i = 0;
            for day in 0..$day_config.count {
                if $self.cond.$eval(staff, day, $staff_config, $day_config) {
                    is_valid = true;
                    if $schedule[staff][day] == $self.shift {
                        i += 1;
                        a += $self.score * ((i as Score) - (mid as Score));
                    }
                }
            }
            if is_valid {
                sum += a;
            }
        }
        sum
    }};
}

#[derive(Debug, PartialEq, Clone)]
pub struct ShiftDirPriority {
    pub cond: CondWrapper,
    pub shift: Shift,
    pub score: Score,
}

impl ShiftDirPriority {
    pub fn new((cond, shift, score): (CondWrapper, Shift, Score)) -> Self {
        Self { cond, shift, score }
    }
}

impl ScoreProp<Shift, ShiftState, DayState> for ShiftDirPriority {
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

impl Check<StdScoreProp, Shift, ShiftState, DayState> for ShiftDirPriority {
    fn check(&self, schedule_config: &ScheduleConfig) -> anyhow::Result<()> {
        self.cond.check(schedule_config)
    }
}

#[cfg(test)]
mod tests {
    use crate::Cond;

    use super::super::super::ScheduleConfig;
    use super::*;

    /// 前を優先
    #[test]
    fn test_front() {
        let schedule = {
            use Shift::*;
            vec![vec![I, A, K, I, A, K]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let mut sp = ShiftDirPriority::new((CondWrapper::new(Cond::True), Shift::I, 1.0));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);

        assert_eq!(2.0, score);
    }

    /// 後ろを優先
    #[test]
    fn test_back() {
        let schedule = {
            use Shift::*;
            vec![vec![I, A, K, I, A, K]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let mut sp = ShiftDirPriority::new((CondWrapper::new(Cond::True), Shift::I, -1.0));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);

        assert_eq!(-2.0, score);
    }
}
