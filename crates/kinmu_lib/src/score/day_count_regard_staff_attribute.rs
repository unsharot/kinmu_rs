//! 指定したシフトをStaffAttributeで指定した数入らなかった場合に発火するスコア

use super::{
    CondWrapper, DayConfig, DayState, Schedule, ScheduleConfig, Shift, ShiftState,
    StaffAttributeNameWrapper, StdScoreProp,
};

use kinmu_input_by_file::Check;
use kinmu_model::{Score, ScoreProp, StaffAttributeName, StaffConfig};

macro_rules! eval {
    ($eval:ident, $self:expr, $staff_config:expr, $day_config:expr, $schedule:expr) => {{
        let mut sum = 0.0;
        for staff in 0..$staff_config.count {
            let mut is_valid = false;
            let mut count = 0;
            for day in 0..$day_config.count {
                if $self.cond.$eval(staff, day, $staff_config, $day_config) {
                    is_valid = true;
                    if $schedule[staff][day] == $self.shift {
                        count += 1;
                    }
                }
            }
            if is_valid {
                let count_needed = $staff_config.get_attribute(staff, &$self.attribute);
                if count_needed != -1 {
                    // 値が-1の場合、任意の数を許すためスコアを増やさない
                    let d = (count - count_needed).abs() as Score;
                    let a = d * $self.score;
                    sum += a * a;
                }
            }
        }
        sum
    }};
}

#[derive(Debug, PartialEq, Clone)]
pub struct DayCountRegardStaffAttribute {
    pub cond: CondWrapper,
    pub shift: Shift,
    pub attribute: StaffAttributeName,
    pub score: Score,
}

impl DayCountRegardStaffAttribute {
    pub fn new(
        (cond, shift, attribute, score): (CondWrapper, Shift, StaffAttributeName, Score),
    ) -> Self {
        Self {
            cond,
            shift,
            attribute,
            score,
        }
    }
}

impl ScoreProp<Shift, ShiftState, DayState> for DayCountRegardStaffAttribute {
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

impl Check<StdScoreProp, Shift, ShiftState, DayState> for DayCountRegardStaffAttribute {
    fn check(&self, schedule_config: &ScheduleConfig) -> anyhow::Result<()> {
        self.cond
            .check(schedule_config)
            .and(StaffAttributeNameWrapper(&self.attribute).check(schedule_config))
    }
}

#[cfg(test)]
mod tests {
    use crate::Cond;

    use kinmu_model::Staff;

    use super::super::super::ScheduleConfig;
    use super::*;

    /// ヒットするべきでないパターン
    #[test]
    fn test_pass() {
        let schedule = {
            use Shift::*;
            vec![vec![I, A, K, N]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();
        schedule_config.staff.list.push(Staff {
            name: String::from(""),
            attributes: vec![1],
        });
        schedule_config
            .staff
            .attribute_map
            .names
            .push(String::from("n_count"));
        schedule_config
            .staff
            .attribute_map
            .name_to_index
            .insert(String::from("n_count"), 0);

        let mut sp = DayCountRegardStaffAttribute::new((
            CondWrapper::new(Cond::Every),
            Shift::N,
            String::from("n_count"),
            1.0,
        ));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);

        assert_eq!(0.0, score);
    }

    /// ヒットするパターン
    #[test]
    fn test_hit() {
        let schedule = {
            use Shift::*;
            vec![vec![I, A, N, N]]
        };

        let mut schedule_config: ScheduleConfig = Default::default();
        schedule_config.day.count = schedule[0].len();
        schedule_config.staff.count = schedule.len();
        schedule_config.staff.list.push(Staff {
            name: String::from(""),
            attributes: vec![1],
        });
        schedule_config
            .staff
            .attribute_map
            .names
            .push(String::from("n_count"));
        schedule_config
            .staff
            .attribute_map
            .name_to_index
            .insert(String::from("n_count"), 0);

        let mut sp = DayCountRegardStaffAttribute::new((
            CondWrapper::new(Cond::Every),
            Shift::N,
            String::from("n_count"),
            1.0,
        ));

        let score = sp.eval_mut(&schedule_config.staff, &schedule_config.day, &schedule);

        assert_eq!(1.0, score);
    }
}
