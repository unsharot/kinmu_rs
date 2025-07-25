//! 指定したシフトが指定した数より少ない場合に発火するスコア

use super::{
    CondWrapper, DayConfig, DayState, Schedule, ScheduleConfig, Shift, ShiftState, StaffConfig,
    StdScoreProp,
};

use kinmu_input_by_file::Check;
use kinmu_model::{Score, ScoreProp};

macro_rules! eval {
    ($eval:ident, $can_skip_day:ident, $self:expr, $staff_config:expr, $day_config:expr, $schedule:expr) => {{
        let mut sum = 0.0;
        for day in 0..$day_config.count {
            if $self.cond.$can_skip_day(day, $staff_config, $day_config) {
                continue;
            }
            let mut is_valid = false;
            let mut staff_count = 0;
            for staff in 0..$staff_config.count {
                if $self.cond.$eval(staff, day, $staff_config, $day_config) {
                    is_valid = true;
                    if $schedule[staff][day] == $self.shift {
                        staff_count += 1;
                    }
                }
            }
            if is_valid {
                let d = std::cmp::min(staff_count - $self.count, 0) as Score;
                sum += d * d;
            }
        }
        sum * $self.score
    }};
}

#[derive(Debug, PartialEq, Clone)]
pub struct StaffCountAtLeast {
    pub cond: CondWrapper,
    pub shift: Shift,
    pub count: i32,
    pub score: Score,
}

impl StaffCountAtLeast {
    pub fn new((cond, shift, count, score): (CondWrapper, Shift, i32, Score)) -> Self {
        Self {
            cond,
            shift,
            count,
            score,
        }
    }
}

impl ScoreProp<Shift, ShiftState, DayState> for StaffCountAtLeast {
    fn eval_mut(
        &mut self,
        staff_config: &StaffConfig,
        day_config: &DayConfig,
        schedule: &Schedule,
    ) -> Score {
        eval!(
            eval_mut,
            can_skip_day_mut,
            self,
            staff_config,
            day_config,
            schedule
        )
    }

    fn eval_immut(
        &self,
        staff_config: &StaffConfig,
        day_config: &DayConfig,
        schedule: &Schedule,
    ) -> Score {
        eval!(
            eval_immut,
            can_skip_day_immut,
            self,
            staff_config,
            day_config,
            schedule
        )
    }
}

impl Check<StdScoreProp, Shift, ShiftState, DayState> for StaffCountAtLeast {
    fn check(&self, schedule_config: &ScheduleConfig) -> anyhow::Result<()> {
        self.cond.check(schedule_config)
    }
}

#[cfg(test)]
mod tests {
    use crate::Cond;

    use super::super::super::ScheduleConfig;
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

        let mut sp = StaffCountAtLeast::new((CondWrapper::new(Cond::True), Shift::N, 1, 1.0));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);

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

        let mut sp = StaffCountAtLeast::new((CondWrapper::new(Cond::True), Shift::N, 1, 1.0));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);

        assert_eq!(1.0, score);
    }
}
