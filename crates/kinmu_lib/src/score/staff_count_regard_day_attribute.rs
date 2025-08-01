//! 指定したシフトがDayAttributeで指定した数いない場合に発火するスコア

use crate::DayAttributeNameWrapper;

use super::{
    CondWrapper, DayConfig, DayState, Schedule, ScheduleConfig, Shift, ShiftState, StaffConfig,
    StdScoreProp,
};

use kinmu_input_by_file::Check;
use kinmu_model::{DayAttributeName, Score, ScoreProp};

macro_rules! eval {
    ($eval:ident, $can_skip_day:ident, $self:expr, $staff_config:expr, $day_config:expr, $schedule:expr) => {{
        let mut sum = 0.0;
        for day in 0..$day_config.count {
            if $self.cond.$can_skip_day(day, $staff_config, $day_config) {
                continue;
            }
            let mut is_valid = false;
            let mut count = 0;
            for staff in 0..$staff_config.count {
                if $self.cond.$eval(staff, day, $staff_config, $day_config) {
                    is_valid = true;
                    if $schedule[staff][day] == $self.shift {
                        count += 1;
                    }
                }
            }
            if is_valid {
                let count_needed = $day_config.attributes.get(&$self.attribute).unwrap()[day];
                let d = (count - count_needed).abs() as Score;
                sum += d * d;
            }
        }
        sum * $self.score
    }};
}

#[derive(Debug, PartialEq, Clone)]
pub struct StaffCountRegardDayAttribute {
    pub cond: CondWrapper,
    pub shift: Shift,
    pub attribute: DayAttributeName,
    pub score: Score,
}

impl StaffCountRegardDayAttribute {
    pub fn new(
        (cond, shift, attribute, score): (CondWrapper, Shift, DayAttributeName, Score),
    ) -> Self {
        Self {
            cond,
            shift,
            attribute,
            score,
        }
    }
}

impl ScoreProp<Shift, ShiftState, DayState> for StaffCountRegardDayAttribute {
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

impl Check<StdScoreProp, Shift, ShiftState, DayState> for StaffCountRegardDayAttribute {
    fn check(&self, schedule_config: &ScheduleConfig) -> anyhow::Result<()> {
        self.cond
            .check(schedule_config)
            .and(DayAttributeNameWrapper(&self.attribute).check(schedule_config))
    }
}

#[cfg(test)]
mod tests {
    use crate::Cond;

    use super::super::super::ScheduleConfig;
    use super::*;

    /// Nが指定した数あるケース
    #[test]
    fn test_pass() {
        let schedule = {
            use Shift::*;
            vec![vec![O, N, N, N], vec![N, N, O, O]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();
        schedule_config
            .day
            .attributes
            .insert(String::from("n_staff_count"), vec![1, 2, 1, 1]);

        let mut sp = StaffCountRegardDayAttribute::new((
            CondWrapper::new(Cond::True),
            Shift::N,
            String::from("n_staff_count"),
            1.0,
        ));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);

        assert_eq!(0.0, score);
    }

    /// Nが指定した数ないケース
    #[test]
    fn test_hit() {
        let schedule = {
            use Shift::*;
            vec![vec![O, N, N, N], vec![N, K, O, O]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();
        schedule_config
            .day
            .attributes
            .insert(String::from("n_staff_count"), vec![1, 2, 1, 1]);

        let mut sp = StaffCountRegardDayAttribute::new((
            CondWrapper::new(Cond::True),
            Shift::N,
            String::from("n_staff_count"),
            1.0,
        ));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);

        assert_eq!(1.0, score);
    }
}
