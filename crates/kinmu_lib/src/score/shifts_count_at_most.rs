//! 指定したシフトが指定した数より多かったスタッフごとに発火するスコア

use super::{
    CondWrapper, DayConfig, DayState, Schedule, ScheduleConfig, Shift, ShiftState, StdScoreProp,
};

use kinmu_input_by_file::Check;
use kinmu_model::{Score, ScoreProp, StaffConfig};

macro_rules! eval {
    ($eval:ident, $self:expr, $staff_config:expr, $day_config:expr, $schedule:expr) => {{
        let mut sum = 0.0;
        for staff in 0..$staff_config.count {
            let mut is_valid = false;
            let mut count = 0;
            for day in 0..$day_config.count {
                if $self.cond.$eval(staff, day, $staff_config, $day_config) {
                    is_valid = true;
                    if $self.shifts.contains(&$schedule[staff][day]) {
                        count += 1;
                    }
                }
            }
            if is_valid {
                let count_border = $self.count;
                let d = std::cmp::max(count - count_border, 0) as Score;
                let a = d * $self.score;
                sum += a * a;
            }
        }
        sum
    }};
}

#[derive(Debug, PartialEq, Clone)]
pub struct ShiftsCountAtMost {
    pub cond: CondWrapper,
    pub shifts: Vec<Shift>,
    pub count: i32,
    pub score: Score,
}

impl ShiftsCountAtMost {
    pub fn new((cond, shifts, count, score): (CondWrapper, Vec<Shift>, i32, Score)) -> Self {
        Self {
            cond,
            shifts,
            count,
            score,
        }
    }
}

impl ScoreProp<Shift, ShiftState, DayState> for ShiftsCountAtMost {
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

impl Check<StdScoreProp, Shift, ShiftState, DayState> for ShiftsCountAtMost {
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
            vec![vec![K, K, N, N]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let mut sp = ShiftsCountAtMost::new((CondWrapper::new(Cond::True), vec![Shift::N], 2, 1.0));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);

        assert_eq!(0.0, score);
    }

    /// ヒットするパターン
    #[test]
    fn test_hit() {
        let schedule = {
            use Shift::*;
            vec![vec![N, K, N, N]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let mut sp = ShiftsCountAtMost::new((CondWrapper::new(Cond::True), vec![Shift::N], 2, 1.0));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);

        assert_eq!(1.0, score);
    }
}
