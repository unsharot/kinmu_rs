//! 列ごとの指定したシフトの数の分散に定数を掛けた値を返す

use super::{
    CondWrapper, DayConfig, DayState, Schedule, ScheduleConfig, Shift, ShiftState, StaffConfig,
    StdScoreProp,
};

use kinmu_input_by_file::Check;
use kinmu_model::{Score, ScoreProp};

macro_rules! eval {
    ($eval:ident, $can_skip_day:ident, $self:expr, $staff_config:expr, $day_config:expr, $schedule:expr) => {{
        let mut cs = Vec::new();
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
                cs.push(staff_count);
            }
        }

        // 平均をとる
        let mut ave = 0.0;
        for c in &cs {
            ave += *c as f32;
        }
        ave /= cs.len() as f32;

        // 分散をとる
        let mut var = 0.0;
        for c in &cs {
            let d = *c as f32 - ave;
            var += d * d;
        }
        var /= cs.len() as f32;

        var * $self.score
    }};
}

#[derive(Debug, PartialEq, Clone)]
pub struct StaffCountVariance {
    pub cond: CondWrapper,
    pub shift: Shift,
    pub score: Score,
}

impl StaffCountVariance {
    pub fn new((cond, shift, score): (CondWrapper, Shift, Score)) -> Self {
        Self { cond, shift, score }
    }
}

impl ScoreProp<Shift, ShiftState, DayState> for StaffCountVariance {
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

impl Check<StdScoreProp, Shift, ShiftState, DayState> for StaffCountVariance {
    fn check(&self, schedule_config: &ScheduleConfig) -> anyhow::Result<()> {
        self.cond.check(schedule_config)
    }
}

#[cfg(test)]
mod tests {
    use crate::Cond;

    use super::super::super::ScheduleConfig;
    use super::*;

    /// Nが常に1なケース
    #[test]
    fn test_pass() {
        let schedule = {
            use Shift::*;
            vec![vec![O, H, N, N], vec![N, N, O, O]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let mut sp = StaffCountVariance::new((CondWrapper::new(Cond::True), Shift::N, 1.0));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);

        assert_eq!(0.0, score);

        let mut sp = StaffCountVariance::new((CondWrapper::new(Cond::True), Shift::N, 10.0));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);

        assert_eq!(0.0, score);
    }

    /// Nが一部0なケース
    #[test]
    fn test_hit_over() {
        let schedule = {
            use Shift::*;
            vec![vec![O, H, N, N], vec![N, K, O, O]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let mut sp = StaffCountVariance::new((CondWrapper::new(Cond::True), Shift::N, 1.0));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);

        assert_eq!(0.1875, score);

        let mut sp = StaffCountVariance::new((CondWrapper::new(Cond::True), Shift::N, 10.0));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);

        assert_eq!(1.875, score);
    }

    /// Nが一部2なケース
    #[test]
    fn test_hit_lack() {
        let schedule = {
            use Shift::*;
            vec![vec![O, N, N, N], vec![N, N, O, O]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();

        let mut sp = StaffCountVariance::new((CondWrapper::new(Cond::True), Shift::N, 1.0));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);

        assert_eq!(0.1875, score);

        let mut sp = StaffCountVariance::new((CondWrapper::new(Cond::True), Shift::N, 10.0));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);

        assert_eq!(1.875, score);
    }
}
