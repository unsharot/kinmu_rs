//! 指定したシフトが指定回数連続して存在するか判定するスコア
//! 指定回数+1回連続は1回分としてカウントされる

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
            let mut a = 0.0;
            let mut accum = 0;
            for day in 0..$day_config.count {
                if $self.cond.$eval(staff, day, $staff_config, $day_config) {
                    if $self.target_shifts.contains(&$schedule[staff][day]) {
                        accum += 1;
                    } else {
                        accum = 0;
                    }
                    if accum >= $self.streak_count {
                        a += $self.score;
                        accum = 0;
                    }
                }
            }
            sum += a;
        }
        sum
    }};
}

#[derive(Debug, PartialEq, Clone)]
pub struct Streak {
    pub cond: CondWrapper,
    pub target_shifts: Vec<Shift>,
    pub streak_count: i32,
    pub score: Score,
}

impl Streak {
    pub fn new(
        (cond, target_shifts, streak_count, score): (CondWrapper, Vec<Shift>, i32, Score),
    ) -> Self {
        Self {
            cond,
            target_shifts,
            streak_count,
            score,
        }
    }
}

impl ScoreProp<Shift, ShiftState, DayState> for Streak {
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

impl Check<StdScoreProp, Shift, ShiftState, DayState> for Streak {
    fn check(&self, schedule_config: &ScheduleConfig) -> anyhow::Result<()> {
        self.cond.check(schedule_config)
    }
}

#[cfg(test)]
mod tests {
    use crate::Cond;

    use super::super::super::ScheduleConfig;
    use super::*;

    /// YまたはKが2連続でないことを検知
    #[test]
    fn test_pass() {
        let schedule = {
            use Shift::*;
            vec![vec![N, K, N, N]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let mut sp = Streak::new((
            CondWrapper::new(Cond::True),
            vec![Shift::K, Shift::Y],
            2,
            -1.0,
        ));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);

        assert_eq!(0.0, score);
    }

    /// YまたはKが2連続であることを検知
    #[test]
    fn test_hit() {
        let schedule = {
            use Shift::*;
            vec![vec![N, K, Y, N]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let mut sp = Streak::new((
            CondWrapper::new(Cond::True),
            vec![Shift::K, Shift::Y],
            2,
            -1.0,
        ));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);

        assert_eq!(-1.0, score);
    }
}
