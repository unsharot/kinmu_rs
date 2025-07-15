//! 指定したパターンが存在するスタッフに対して発火するスコア
//! 計算量はO(NM)
//! TODO: RollingHash、FSMやTrie木を用いた高速化

use super::{
    CondWrapper, DayConfig, DayState, Schedule, ScheduleConfig, Shift, ShiftState, StaffConfig,
    StdScoreProp,
};

use kinmu_input_by_file::Check;
use kinmu_model::{Score, ScoreProp};

macro_rules! eval {
    ($eval:ident, $can_skip_staff:ident, $self:expr, $staff_config:expr, $day_config:expr, $schedule:expr) => {{
        let mut sum = 0.0;
        for staff in 0..$staff_config.count {
            if $self
                .cond
                .$can_skip_staff(staff, $staff_config, $day_config)
            {
                continue;
            }
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
                        if !($self.shift_pattern[dd].contains(&$schedule[staff][day + dd])) {
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
pub struct PatternGeneralAny {
    pub cond: CondWrapper,
    pub shift_pattern: Vec<Vec<Shift>>,
    pub score: Score,
}

impl PatternGeneralAny {
    pub fn new((cond, shift_pattern, score): (CondWrapper, Vec<Vec<Shift>>, Score)) -> Self {
        Self {
            cond,
            shift_pattern,
            score,
        }
    }
}

impl ScoreProp<Shift, ShiftState, DayState> for PatternGeneralAny {
    fn eval_mut(
        &mut self,
        staff_config: &StaffConfig,
        day_config: &DayConfig,
        schedule: &Schedule,
    ) -> Score {
        eval!(
            eval_mut,
            can_skip_staff_mut,
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
            can_skip_staff_immut,
            self,
            staff_config,
            day_config,
            schedule
        )
    }
}

impl Check<StdScoreProp, Shift, ShiftState, DayState> for PatternGeneralAny {
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
    fn test_pass_with_cond() {
        let schedule = {
            use Shift::*;
            &vec![vec![
                N, K, K, K, O, I, A, K, H, O, K, H, N, I, A, K, H, I, A, K, O, N, I, A, K, N, O, N,
                K, I, A, K, H, I, A, K, O,
            ]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.day.buffer_count = 3;
        schedule_config.staff.count = 1;

        let mut sp = PatternGeneralAny::new((
            CondWrapper::new(Cond::NoBuffer),
            vec![vec![Shift::K, Shift::Y], vec![Shift::K, Shift::Y]],
            -1000.0,
        ));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);
        assert_eq!(0.0, score);
    }

    /// ヒットするパターン
    #[test]
    fn test_hit_with_cond() {
        let schedule = {
            use Shift::*;
            &vec![vec![
                N, K, K, K, O, I, A, K, H, O, K, H, N, I, A, K, H, I, A, K, O, N, I, A, K, N, O, N,
                K, I, A, K, H, I, A, K, O,
            ]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.day.buffer_count = 3;
        schedule_config.staff.count = 1;

        let mut sp = PatternGeneralAny::new((
            CondWrapper::new(Cond::True),
            vec![vec![Shift::K, Shift::Y], vec![Shift::K, Shift::Y]],
            -1000.0,
        ));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);
        assert_eq!(-1000.0, score);
    }
}
