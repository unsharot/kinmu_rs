//! NGリストにあるペアがともに指定したシフトなら発火するスコア

use super::super::{
    CondWrapper, DayConfig, DayState, Schedule, ScheduleConfig, ScoreProp, Shift, ShiftState,
    StaffConfig,
};

use kinmu_input::Check;
use kinmu_model::{Score, ScorePropTrait};

macro_rules! eval {
    ($eval:ident, $self:expr, $staff_config:expr, $day_config:expr, $schedule:expr) => {{
        let mut sum = 0.0;
        for day in 0..$day_config.count {
            let mut a = 0.0;
            for i in 0..$staff_config.ng_list.len() {
                let (staff1, staff2) = $staff_config.ng_list[i];
                if $self.cond.$eval(staff1, day, $staff_config, $day_config)
                    && $self.cond.$eval(staff2, day, $staff_config, $day_config)
                    && $schedule[staff1][day] == $self.shift
                    && $schedule[staff2][day] == $self.shift
                {
                    a += $self.score;
                }
            }
            sum += a;
        }
        sum
    }};
}

#[derive(Debug, PartialEq, Clone)]
pub struct NGPair {
    pub cond: CondWrapper,
    pub shift: Shift,
    pub score: Score,
}

impl NGPair {
    pub fn new((cond, shift, score): (CondWrapper, Shift, Score)) -> Self {
        Self { cond, shift, score }
    }
}

impl ScorePropTrait<Shift, ShiftState, DayState> for NGPair {
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

impl Check<ScoreProp, Shift, ShiftState, DayState> for NGPair {
    fn check(&self, schedule_config: &ScheduleConfig) -> anyhow::Result<()> {
        self.cond.check(schedule_config)
    }
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

        let mut sp = NGPair::new((CondWrapper::new(Cond::Every), Shift::I, 1.0));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);

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

        let mut sp = NGPair::new((CondWrapper::new(Cond::Every), Shift::I, 1.0));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);

        assert_eq!(2.0, score);
    }
}
