//! Cond型の定義と実装

use super::{
    DayConfig, DayState, ScheduleConfig, Shift, ShiftState, StaffAttributeNameWrapper, StdScoreProp,
};

use kinmu_input_by_file::{Check, FromConfig, VecWrapper};
use kinmu_model::{StaffAttributeName, StaffConfig};

use anyhow::Context as _;
use std::fmt;

/// ScorePropに用いる条件を管理する型
#[derive(Debug, PartialEq, Clone, Default)]
pub enum Cond {
    // 基本条件
    #[default]
    True,
    False,
    Not(Box<Cond>),
    Or((Box<Cond>, Box<Cond>)),
    And((Box<Cond>, Box<Cond>)),
    Any(Vec<Cond>),
    All(Vec<Cond>),

    // 日付についての条件
    Day(usize),
    DayInRange((usize, usize)),
    DayInList(Vec<usize>),
    NoBuffer,
    DayState(DayState),
    BeforeDayState(DayState),

    // スタッフについての条件
    Staff(usize),
    StaffInRange((usize, usize)),
    StaffWithAttribute((StaffAttributeName, i32)),
}

impl Cond {
    pub fn eval(&self, staff: usize, day: usize, sc: &StaffConfig, dc: &DayConfig) -> bool {
        match self {
            Cond::True => true,
            Cond::False => false,
            Cond::Not(cond) => !cond.eval(staff, day, sc, dc),
            Cond::Or((cond1, cond2)) => {
                cond1.eval(staff, day, sc, dc) || cond2.eval(staff, day, sc, dc)
            }
            Cond::And((cond1, cond2)) => {
                cond1.eval(staff, day, sc, dc) && cond2.eval(staff, day, sc, dc)
            }
            Cond::Any(cs) => cs.iter().any(|c| c.eval(staff, day, sc, dc)),
            Cond::All(cs) => cs.iter().all(|c| c.eval(staff, day, sc, dc)),

            Cond::Day(d) => *d == day,
            Cond::DayInRange((day_start, day_end)) => *day_start <= day && day <= *day_end,
            Cond::DayInList(ds) => ds.iter().any(|d| *d == day),
            Cond::NoBuffer => dc.buffer_count <= day,
            Cond::DayState(ds) => dc.days[day] == *ds,
            Cond::BeforeDayState(ds) => {
                if day + 1 >= dc.count {
                    false
                } else {
                    dc.days[day + 1] == *ds
                }
            }

            Cond::Staff(s) => *s == staff,
            Cond::StaffInRange((staff_start, staff_end)) => {
                *staff_start <= staff && staff <= *staff_end
            }
            Cond::StaffWithAttribute((attribute, value)) => {
                sc.get_attribute(staff, attribute) == *value
            }
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
    pub fn eval_mut(&mut self, staff: usize, day: usize, sc: &StaffConfig, dc: &DayConfig) -> bool {
        if self.memo.is_empty() {
            self.memo = vec![vec![None; dc.count]; sc.count];
        }
        match self.memo[staff][day] {
            Some(ans) => ans,
            None => {
                let ans = self.cond.eval(staff, day, sc, dc);
                self.memo[staff][day] = Some(ans);
                ans
            }
        }
    }

    /// メモに記入しないCondの評価
    /// ただし、メモが記入されている場合は利用する
    pub fn eval_immut(&self, staff: usize, day: usize, sc: &StaffConfig, dc: &DayConfig) -> bool {
        if self.memo.is_empty() {
            return self.cond.eval(staff, day, sc, dc);
        }
        match self.memo[staff][day] {
            Some(ans) => ans,
            None => self.cond.eval(staff, day, sc, dc),
        }
    }

    /// メモを参照しないCondの評価
    pub fn eval_anyway(&self, staff: usize, day: usize, sc: &StaffConfig, dc: &DayConfig) -> bool {
        self.cond.eval(staff, day, sc, dc)
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
            ("True", _) => Ok(Cond::True),
            ("False", _) => Ok(Cond::False),
            ("Not", p) => Ok(Cond::Not(Box::new(<Cond>::from_config(p)?))),
            ("Or", p) => Ok(Cond::Or(<(Box<Cond>, Box<Cond>)>::from_config(p)?)),
            ("And", p) => Ok(Cond::And(<(Box<Cond>, Box<Cond>)>::from_config(p)?)),
            ("Any", p) => Ok(Cond::Any(<VecWrapper<Cond>>::from_config(p)?.0)),
            ("All", p) => Ok(Cond::All(<VecWrapper<Cond>>::from_config(p)?.0)),

            ("Day", p) => Ok(Cond::Day(<usize>::from_config(p)?)),
            ("DayInRange", p) => Ok(Cond::DayInRange(<(usize, usize)>::from_config(p)?)),
            ("DayInList", p) => Ok(Cond::DayInList(<VecWrapper<usize>>::from_config(p)?.0)),
            ("NoBuffer", _) => Ok(Cond::NoBuffer),
            ("DayState", p) => Ok(Cond::DayState(<DayState>::from_config(p)?)),
            ("BeforeDayState", p) => Ok(Cond::BeforeDayState(<DayState>::from_config(p)?)),

            ("Staff", p) => Ok(Cond::Staff(<usize>::from_config(p)?)),
            ("StaffInRange", p) => Ok(Cond::StaffInRange(<(usize, usize)>::from_config(p)?)),
            ("StaffWithAttribute", p) => Ok(Cond::StaffWithAttribute(
                <(StaffAttributeName, i32)>::from_config(p)?,
            )),
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

impl Check<StdScoreProp, Shift, ShiftState, DayState> for Cond {
    fn check(&self, schedule_config: &ScheduleConfig) -> anyhow::Result<()> {
        match self {
            Cond::True => Ok(()),
            Cond::False => Ok(()),
            Cond::Not(c) => c.check(schedule_config),
            Cond::Or((c1, c2)) => c1.check(schedule_config).and(c2.check(schedule_config)),
            Cond::And((c1, c2)) => c1.check(schedule_config).and(c2.check(schedule_config)),
            Cond::Any(cs) => cs.iter().try_for_each(|_| Ok(())),
            Cond::All(cs) => cs.iter().try_for_each(|_| Ok(())),

            Cond::Day(_) => Ok(()),
            Cond::DayInRange(_) => Ok(()),
            Cond::DayInList(_) => Ok(()),
            Cond::NoBuffer => Ok(()),
            Cond::DayState(_) => Ok(()),
            Cond::BeforeDayState(_) => Ok(()),

            Cond::Staff(_) => Ok(()),
            Cond::StaffInRange(_) => Ok(()),
            Cond::StaffWithAttribute((sa, _)) => {
                StaffAttributeNameWrapper(sa).check(schedule_config)
            }
        }
        .with_context(|| format!("Cond {:?} の変換チェックに失敗しました", self))?;
        Ok(())
    }
}

impl Check<StdScoreProp, Shift, ShiftState, DayState> for CondWrapper {
    fn check(&self, schedule_config: &ScheduleConfig) -> anyhow::Result<()> {
        self.cond.check(schedule_config).with_context(|| {
            format!("CondWrapper {:?} の変換チェックに失敗しました", &self.cond)
        })?;
        Ok(())
    }
}
