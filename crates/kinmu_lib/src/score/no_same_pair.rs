//! 指定回数以上同じペアなら発火するスコア

use super::{
    CondWrapper, DayConfig, DayState, Schedule, ScheduleConfig, ScoreProp, Shift, ShiftState,
    StaffConfig,
};

use kinmu_input::Check;
use kinmu_model::{Score, ScorePropTrait};

use std::collections::HashMap;

macro_rules! eval {
    ($eval:ident, $self:expr, $staff_config:expr, $day_config:expr, $schedule:expr) => {{
        let mut pair_map: HashMap<Vec<usize>, i32> = HashMap::new();
        for day in 0..$day_config.count {
            let mut i_list: Vec<usize> = Vec::new();
            for staff in 0..$staff_config.count {
                if $self.cond.$eval(staff, day, $staff_config, $day_config)
                    && $schedule[staff][day] == $self.shift
                {
                    i_list.push(staff);
                }
            }
            // ある日の夜勤の人数が2人以上ならペアのマップに加算
            if i_list.len() >= 2 {
                *pair_map.entry(i_list).or_insert(0) += 1;
            }
        }
        let mut ans = 0.0;
        for count in pair_map.values() {
            let a = *count - $self.pair_limit + 1;
            if a > 0 {
                ans += (a as Score) * $self.score
            }
        }
        ans
    }};
}

#[derive(Debug, PartialEq, Clone)]
pub struct NoSamePair {
    pub cond: CondWrapper,
    pub pair_limit: i32,
    pub shift: Shift,
    pub score: Score,
}

impl NoSamePair {
    pub fn new((cond, pair_limit, shift, score): (CondWrapper, i32, Shift, Score)) -> Self {
        Self {
            cond,
            pair_limit,
            shift,
            score,
        }
    }
}

impl ScorePropTrait<Shift, ShiftState, DayState> for NoSamePair {
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

impl Check<ScoreProp, Shift, ShiftState, DayState> for NoSamePair {
    fn check(&self, schedule_config: &ScheduleConfig) -> anyhow::Result<()> {
        self.cond.check(schedule_config)
    }
}

#[cfg(test)]
mod tests {
    use crate::Cond;

    use super::super::super::ScheduleConfig;
    use super::*;

    /// 2度同じペアにならない場合、検出しない
    #[test]
    fn test_pass2() {
        let schedule = {
            use Shift::*;
            vec![vec![I, A, K, I, A, K], vec![N, N, N, I, A, K]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let mut sp = NoSamePair::new((CondWrapper::new(Cond::Every), 2, Shift::I, 1.0));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);

        assert_eq!(0.0, score);
    }

    /// 2度同じペアになる場合の検出
    #[test]
    fn test_hit2() {
        let schedule = {
            use Shift::*;
            vec![vec![I, A, K, I, A, K], vec![I, N, N, I, A, K]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let mut sp = NoSamePair::new((CondWrapper::new(Cond::Every), 2, Shift::I, 1.0));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);

        assert_eq!(1.0, score);
    }

    /// 3度同じペアにならない場合、検出しない
    #[test]
    fn test_pass3() {
        let schedule = {
            use Shift::*;
            vec![vec![I, A, K, I, A, K], vec![I, N, N, I, A, K]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let mut sp = NoSamePair::new((CondWrapper::new(Cond::Every), 3, Shift::I, 1.0));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);

        assert_eq!(0.0, score);
    }

    /// 3度同じペアになる場合の検出
    #[test]
    fn test_hit3() {
        let schedule = {
            use Shift::*;
            vec![vec![I, I, K, I, A, K], vec![I, I, N, I, A, K]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let mut sp = NoSamePair::new((CondWrapper::new(Cond::Every), 3, Shift::I, 1.0));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);

        assert_eq!(1.0, score);
    }
}
