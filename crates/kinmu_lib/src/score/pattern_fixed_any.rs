//! 指定したパターンが存在するスタッフに対して発火するスコア
//! ただし、パターンは固定
//! 計算量はO(NM)
//! TODO: RollingHash、FSMやTrie木を用いた高速化

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
            let mut any = false;
            for day in 0..$day_config.count {
                let mut hit = true;
                let mut is_valid = false;
                for dd in 0..$self.shift_pattern.len() {
                    if $day_config.count <= day + dd {
                        hit = false;
                        break;
                    } else if $self
                        .cond
                        .$eval(staff, day + dd, $staff_config, $day_config)
                    {
                        is_valid = true;
                        if $self.shift_pattern[dd] != $schedule[staff][day + dd] {
                            hit = false;
                            break;
                        }
                    } else {
                        hit = false;
                        break;
                    }
                }
                if hit && is_valid {
                    any = true;
                    break;
                }
            }
            if any {
                sum += $self.score;
            }
        }
        sum
    }};
}

#[derive(Debug, PartialEq, Clone)]
pub struct PatternFixedAny {
    pub cond: CondWrapper,
    pub shift_pattern: Vec<Shift>,
    pub score: Score,
}

impl PatternFixedAny {
    pub fn new((cond, shift_pattern, score): (CondWrapper, Vec<Shift>, Score)) -> Self {
        Self {
            cond,
            shift_pattern,
            score,
        }
    }
}

impl ScoreProp<Shift, ShiftState, DayState> for PatternFixedAny {
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

impl Check<StdScoreProp, Shift, ShiftState, DayState> for PatternFixedAny {
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
            vec![vec![O, O, K, H, A, K], vec![N, N, O, I, H, K]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let mut sp =
            PatternFixedAny::new((CondWrapper::new(Cond::True), vec![Shift::O, Shift::H], 1.0));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);

        assert_eq!(0.0, score);
    }

    /// OHパターンの検出
    #[test]
    fn test_hit() {
        let schedule = {
            use Shift::*;
            vec![vec![O, O, K, H, A, K], vec![N, N, O, H, A, K]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let mut sp =
            PatternFixedAny::new((CondWrapper::new(Cond::True), vec![Shift::O, Shift::H], 1.0));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);

        assert_eq!(1.0, score);
    }

    /// 2回パターンが存在する場合も1回としてカウントする
    #[test]
    fn test_double() {
        let schedule = {
            use Shift::*;
            vec![vec![O, O, K, H, A, K], vec![N, N, O, H, O, H]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let mut sp =
            PatternFixedAny::new((CondWrapper::new(Cond::True), vec![Shift::O, Shift::H], 1.0));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);

        assert_eq!(1.0, score);
    }
}
