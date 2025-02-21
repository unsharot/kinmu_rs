//! 条件に関わる型の定義

use super::{DayState, ScheduleConfig, StaffAttributeName};

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Cond {
    Every,
    Or((Box<Cond>, Box<Cond>)),
    And((Box<Cond>, Box<Cond>)),
    Not(Box<Cond>),

    DayExceptBuffer,
    DayInRange((usize, usize)),
    ParticularDayState(DayState),
    BeforeDayState(DayState),
    ParticularDay(usize),

    StaffInRange((usize, usize)),
    StaffWithAttribute((StaffAttributeName, i32)),
    ParticularStaff(usize),
}

impl Cond {
    pub fn eval(&self, staff: usize, day: usize, sc: &ScheduleConfig) -> bool {
        match self {
            Cond::Every => true,
            Cond::Or((cond1, cond2)) => cond1.eval(staff, day, sc) || cond2.eval(staff, day, sc),
            Cond::And((cond1, cond2)) => cond1.eval(staff, day, sc) && cond2.eval(staff, day, sc),
            Cond::Not(cond) => !cond.eval(staff, day, sc),
            Cond::DayExceptBuffer => sc.day.buffer_count <= day,
            Cond::DayInRange((day_start, day_end)) => *day_start <= day && day <= *day_end,
            Cond::ParticularDayState(ds) => sc.day.days[day] == *ds,
            Cond::BeforeDayState(ds) => {
                if day + 1 >= sc.day.count {
                    false
                } else {
                    sc.day.days[day + 1] == *ds
                }
            }
            Cond::ParticularDay(d) => *d == day,
            Cond::StaffInRange((staff_start, staff_end)) => {
                *staff_start <= staff && staff <= *staff_end
            }
            Cond::StaffWithAttribute((attribute, value)) => {
                sc.get_attribute(staff, attribute) == *value
            }
            Cond::ParticularStaff(s) => *s == staff,
        }
    }
}

/// Condをメモ化して高速化するためのラッパー
#[derive(PartialEq, Clone)]
pub struct CondWrapper {
    pub cond: Cond,
    memo: Vec<Vec<Option<bool>>>,
}

impl CondWrapper {
    pub fn new(cond: Cond) -> Self {
        CondWrapper {
            cond,
            memo: <Vec<Vec<Option<bool>>>>::new(),
        }
    }

    /// ScheduleConfigが焼きなましの過程で変化しない制限の上で
    pub fn eval(&mut self, staff: usize, day: usize, sc: &ScheduleConfig) -> bool {
        if self.memo.is_empty() {
            self.memo = vec![vec![None; sc.day.count]; sc.staff.count];
        }
        match self.memo[staff][day] {
            Some(ans) => ans,
            None => {
                let ans = self.cond.eval(staff, day, sc);
                self.memo[staff][day] = Some(ans);
                ans
            }
        }
    }
}

impl fmt::Debug for CondWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.cond)
    }
}
