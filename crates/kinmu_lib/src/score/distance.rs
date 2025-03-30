//! 指定したシフトをStaffAttributeで指定した数入らなかった場合に発火するスコア

use super::{
    CondWrapper, DayConfig, DayState, Schedule, ScheduleConfig, ScoreProp, Shift, ShiftState,
};

use kinmu_input_by_file::Check;
use kinmu_model::{Score, ScorePropTrait, StaffConfig};

macro_rules! eval {
    ($eval:ident, $self:expr, $staff_config:expr, $day_config:expr, $schedule:expr) => {{
        let mut sum = 0.0;
        for staff in 0..$staff_config.count {
            let mut is_valid = false;
            let mut prev: Option<usize> = None;
            let mut a = 0.0;
            for day in 0..$day_config.count {
                if $self.cond.$eval(staff, day, $staff_config, $day_config) {
                    is_valid = true;
                    if $schedule[staff][day] == $self.shift {
                        if let Some(prev_day) = prev {
                            let d = (day - prev_day) as Score;
                            a += d * d;
                        }
                        prev = Some(day);
                    }
                }
            }
            if is_valid {
                sum += a;
            }
        }
        sum * $self.score
    }};
}

#[derive(Debug, PartialEq, Clone)]
pub struct ShiftDistance {
    pub cond: CondWrapper,
    pub shift: Shift,
    pub score: Score,
}

impl ShiftDistance {
    pub fn new((cond, shift, score): (CondWrapper, Shift, Score)) -> Self {
        Self { cond, shift, score }
    }
}

impl ScorePropTrait<Shift, ShiftState, DayState> for ShiftDistance {
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

impl Check<ScoreProp, Shift, ShiftState, DayState> for ShiftDistance {
    fn check(&self, schedule_config: &ScheduleConfig) -> anyhow::Result<()> {
        self.cond.check(schedule_config)
    }
}

#[cfg(test)]
mod tests {
    use crate::Cond;

    use super::super::super::ScheduleConfig;
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

        let mut sp = ShiftDistance::new((CondWrapper::new(Cond::Every), Shift::I, 1.0));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);

        assert_eq!(0.0, score);
    }

    /// ヒットするパターン
    #[test]
    fn test_hit() {
        let schedule = {
            use Shift::*;
            vec![vec![I, A, I, N, I]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let mut sp = ShiftDistance::new((CondWrapper::new(Cond::Every), Shift::I, 1.0));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);

        assert_eq!(8.0, score);
    }
}
