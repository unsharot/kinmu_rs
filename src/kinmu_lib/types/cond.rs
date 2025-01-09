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
    pub fn eval(&self, r: usize, c: usize, sp: &ScheduleConfig) -> bool {
        match self {
            Cond::Every => true,
            Cond::Or((cond1, cond2)) => cond1.eval(r, c, sp) || cond2.eval(r, c, sp),
            Cond::And((cond1, cond2)) => cond1.eval(r, c, sp) && cond2.eval(r, c, sp),
            Cond::Not(cond) => !cond.eval(r, c, sp),
            Cond::DayExceptBuffer => sp.day.buffer_count <= c,
            Cond::DayInRange((day_start, day_end)) => *day_start <= c && c <= *day_end, // indexおかしいかも
            Cond::ParticularDayState(ds) => sp.day.days[c] == *ds, // indexおかしいかも
            Cond::BeforeDayState(ds) => {
                if c == 0 {
                    false
                } else {
                    sp.day.days[c - 1] == *ds
                }
            }
            Cond::ParticularDay(d) => *d == c,
            Cond::StaffInRange((staff_start, staff_end)) => *staff_start <= r && r <= *staff_end, // indexおかしいかも
            Cond::StaffWithAttribute((attribute, value)) => {
                sp.get_attribute(r, attribute) == *value
            }
            Cond::ParticularStaff(staff) => *staff == c, // indexおかしいかも
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
    pub fn eval(&mut self, r: usize, c: usize, sp: &ScheduleConfig) -> bool {
        if self.memo.is_empty() {
            self.memo = vec![vec![None; sp.day.count]; sp.staff.count];
        }
        match self.memo[r][c] {
            Some(ans) => ans,
            None => {
                let ans = self.cond.eval(r, c, sp);
                self.memo[r][c] = Some(ans);
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
