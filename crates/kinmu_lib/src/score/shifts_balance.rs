//! 指定した2つのシフト数がスタッフあたりでバランス良いか判定するスコア

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
            let mut is_valid = false;
            let mut count1: i32 = 0;
            let mut count2: i32 = 0;
            for day in 0..$day_config.count {
                if $self.cond.$eval(staff, day, $staff_config, $day_config) {
                    is_valid = true;
                    if $schedule[staff][day] == $self.shift1 {
                        count1 += 1;
                    }
                    if $schedule[staff][day] == $self.shift2 {
                        count2 += 1;
                    }
                }
            }
            if is_valid {
                let d = (count1 - count2).abs() as Score;
                sum += d * d;
            }
        }
        sum * $self.score
    }};
}

#[derive(Debug, PartialEq, Clone)]
pub struct ShiftsBalance {
    pub cond: CondWrapper,
    pub shift1: Shift,
    pub shift2: Shift,
    pub score: Score,
}

impl ShiftsBalance {
    pub fn new((cond, shift1, shift2, score): (CondWrapper, Shift, Shift, Score)) -> Self {
        Self {
            cond,
            shift1,
            shift2,
            score,
        }
    }
}

impl ScoreProp<Shift, ShiftState, DayState> for ShiftsBalance {
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

impl Check<StdScoreProp, Shift, ShiftState, DayState> for ShiftsBalance {
    fn check(&self, schedule_config: &ScheduleConfig) -> anyhow::Result<()> {
        self.cond.check(schedule_config)
    }
}

#[cfg(test)]
mod tests {
    use crate::Cond;

    use super::super::super::ScheduleConfig;
    use super::*;

    /// 2つのシフトの数が同じ場合
    #[test]
    fn test_eq() {
        let schedule = {
            use Shift::*;
            vec![vec![O, H, N, N]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let mut sp = ShiftsBalance::new((CondWrapper::new(Cond::True), Shift::O, Shift::H, 1.0));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);

        assert_eq!(0.0, score);
    }

    /// 2つのシフトの数が違う場合
    #[test]
    fn test_neq() {
        let schedule = {
            use Shift::*;
            vec![vec![O, H, N, O]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let mut sp = ShiftsBalance::new((CondWrapper::new(Cond::True), Shift::O, Shift::H, 1.0));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);

        assert_eq!(1.0, score);
    }
}
