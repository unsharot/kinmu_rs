//! 指定したシフトの人数を満たした日付に対して、指定したシフトが指定した値いない時に発火するスコア

use super::{
    CondWrapper, DayConfig, DayState, Schedule, ScheduleConfig, ScoreProp, Shift, ShiftState,
    StaffConfig,
};

use kinmu_input::Check;
use kinmu_model::{Score, ScorePropTrait};

macro_rules! eval {
    ($eval:ident, $self:expr, $staff_config:expr, $day_config:expr, $schedule:expr) => {{
        let mut sum = 0.0;
        for day in 0..$day_config.count {
            let mut count = 0;
            for staff in 0..$staff_config.count {
                if $self
                    .cond_premise
                    .$eval(staff, day, $staff_config, $day_config)
                    && $schedule[staff][day] == $self.shift_premise
                {
                    count += 1;
                }
            }
            if count == $self.count_premise {
                let mut is_valid = false;
                let mut count2 = 0;
                for staff in 0..$staff_config.count {
                    if $self
                        .cond_main
                        .$eval(staff, day, $staff_config, $day_config)
                    {
                        is_valid = true;
                        if $schedule[staff][day] == $self.shift_main {
                            count2 += 1;
                        }
                    }
                }
                if is_valid {
                    let d = (count2 - $self.count_main).abs() as Score;
                    let a = d * $self.score;
                    sum += a * a;
                }
            }
        }
        sum
    }};
}

#[derive(Debug, PartialEq, Clone)]
pub struct StaffCountWithPremise {
    pub cond_premise: CondWrapper,
    pub shift_premise: Shift,
    pub count_premise: i32,
    pub cond_main: CondWrapper,
    pub shift_main: Shift,
    pub count_main: i32,
    pub score: Score,
}

impl StaffCountWithPremise {
    pub fn new(
        (cond_premise, shift_premise, count_premise, cond_main, shift_main, count_main, score): (
            CondWrapper,
            Shift,
            i32,
            CondWrapper,
            Shift,
            i32,
            Score,
        ),
    ) -> Self {
        Self {
            cond_premise,
            shift_premise,
            count_premise,
            cond_main,
            shift_main,
            count_main,
            score,
        }
    }
}

impl ScorePropTrait<Shift, ShiftState, DayState> for StaffCountWithPremise {
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

impl Check<ScoreProp, Shift, ShiftState, DayState> for StaffCountWithPremise {
    fn check(&self, schedule_config: &ScheduleConfig) -> anyhow::Result<()> {
        self.cond_premise
            .check(schedule_config)
            .and(self.cond_main.check(schedule_config))
    }
}

#[cfg(test)]
mod tests {
    use crate::Cond;

    use kinmu_model::Staff;

    use super::super::super::ScheduleConfig;
    use super::*;

    /// Iを一人で担当する人にその資格がある場合
    #[test]
    fn test_pass() {
        let schedule = {
            use Shift::*;
            vec![vec![I, N, N, I], vec![N, N, O, I]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();
        schedule_config.staff.list.push(Staff {
            name: Default::default(),
            attributes: vec![1],
        });
        schedule_config.staff.list.push(Staff {
            name: Default::default(),
            attributes: vec![0],
        });
        schedule_config
            .staff
            .attribute_map
            .names
            .push(String::from("I_alone_ok"));
        schedule_config
            .staff
            .attribute_map
            .name_to_index
            .insert(String::from("I_alone_ok"), 0);

        let mut sp = StaffCountWithPremise::new((
            CondWrapper::new(Cond::Every),
            Shift::I,
            1,
            CondWrapper::new(Cond::StaffWithAttribute((String::from("I_alone_ok"), 1))),
            Shift::I,
            1,
            1.0,
        ));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);

        assert_eq!(0.0, score);
    }

    /// Iを一人で担当する人にその資格がない場合
    #[test]
    fn test_hit() {
        let schedule = {
            use Shift::*;
            vec![vec![N, N, N, I], vec![I, N, O, I]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();
        schedule_config.staff.list.push(Staff {
            name: Default::default(),
            attributes: vec![1],
        });
        schedule_config.staff.list.push(Staff {
            name: Default::default(),
            attributes: vec![0],
        });
        schedule_config
            .staff
            .attribute_map
            .names
            .push(String::from("I_alone_ok"));
        schedule_config
            .staff
            .attribute_map
            .name_to_index
            .insert(String::from("I_alone_ok"), 0);

        let mut sp = StaffCountWithPremise::new((
            CondWrapper::new(Cond::Every),
            Shift::I,
            1,
            CondWrapper::new(Cond::StaffWithAttribute((String::from("I_alone_ok"), 1))),
            Shift::I,
            1,
            1.0,
        ));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);

        assert_eq!(1.0, score);
    }
}
