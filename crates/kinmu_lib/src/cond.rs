//! 条件に関わる型の定義

use super::{DayState, ScheduleConfig};

use ::kinmu_input::FromConfig;
use ::kinmu_model::StaffAttributeName;

use std::fmt;

#[derive(Debug, PartialEq, Clone, Default)]
pub enum Cond {
    #[default]
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
#[derive(PartialEq, Clone, Default)]
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

    /// メモ化したCondの評価
    /// ScheduleConfigが焼きなましの過程で変化しない制限の上で
    pub fn eval_mut(&mut self, staff: usize, day: usize, sc: &ScheduleConfig) -> bool {
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

    /// メモに記入しないCondの評価
    /// ただし、メモが記入されている場合は利用する
    pub fn eval_immut(&self, staff: usize, day: usize, sc: &ScheduleConfig) -> bool {
        if self.memo.is_empty() {
            return self.cond.eval(staff, day, sc);
        }
        match self.memo[staff][day] {
            Some(ans) => ans,
            None => self.cond.eval(staff, day, sc),
        }
    }

    /// メモを参照しないCondの評価
    pub fn eval_anyway(&self, staff: usize, day: usize, sc: &ScheduleConfig) -> bool {
        self.cond.eval(staff, day, sc)
    }
}

impl fmt::Debug for CondWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.cond)
    }
}

impl FromConfig for Cond {
    fn from_config(s: &str) -> anyhow::Result<Self> {
        let words: Vec<&str> = s.splitn(2, ' ').collect();
        anyhow::ensure!(words.len() >= 2, "Needs 2 fields, but not enough.");
        anyhow::ensure!(2 >= words.len(), "Needs 2 fields, but too much given.");
        match (words[0], words[1]) {
            ("Every", _) => Ok(Cond::Every),
            ("Or", p) => Ok(Cond::Or(<(Box<Cond>, Box<Cond>)>::from_config(p)?)),
            ("And", p) => Ok(Cond::And(<(Box<Cond>, Box<Cond>)>::from_config(p)?)),
            ("Not", p) => Ok(Cond::Not(Box::new(<Cond>::from_config(p)?))),
            ("DayExceptBuffer", _) => Ok(Cond::DayExceptBuffer),
            ("DayInRange", p) => Ok(Cond::DayInRange(<(usize, usize)>::from_config(p)?)),
            ("ParticularDayState", p) => Ok(Cond::ParticularDayState(<DayState>::from_config(p)?)),
            ("BeforeDayState", p) => Ok(Cond::BeforeDayState(<DayState>::from_config(p)?)),
            ("ParticularDay", p) => Ok(Cond::ParticularDay(<usize>::from_config(p)?)),
            ("StaffInRange", p) => Ok(Cond::StaffInRange(<(usize, usize)>::from_config(p)?)),
            ("StaffWithAttribute", p) => Ok(Cond::StaffWithAttribute(
                <(StaffAttributeName, i32)>::from_config(p)?,
            )),
            ("ParticularStaff", p) => Ok(Cond::ParticularStaff(<usize>::from_config(p)?)),
            (s, p) => Err(anyhow::anyhow!("Failed to parse Cond: {} {}", s, p)),
        }
    }
}

impl FromConfig for Box<Cond> {
    fn from_config(s: &str) -> anyhow::Result<Self> {
        let cond = Cond::from_config(s)?;
        Ok(Box::new(cond))
    }
}

impl FromConfig for CondWrapper {
    fn from_config(s: &str) -> anyhow::Result<Self> {
        let cond = Cond::from_config(s)?;
        Ok(CondWrapper::new(cond))
    }
}
