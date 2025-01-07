//! 勤務表に使う型の宣言

mod config;

pub use self::config::*;

use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Shift {
    N,
    K,
    I,
    A,
    O,
    H,
    Y,
    D,
    U,
}

impl FromStr for Shift {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" => Ok(Shift::N),
            "K" => Ok(Shift::K),
            "I" => Ok(Shift::I),
            "A" => Ok(Shift::A),
            "O" => Ok(Shift::O),
            "H" => Ok(Shift::H),
            "Y" => Ok(Shift::Y),
            "D" => Ok(Shift::D),
            "U" => Ok(Shift::U),
            " " => Ok(Shift::U),
            _ => Err(format!("Failed to parse Shift: {}", s)),
        }
    }
}

impl fmt::Display for Shift {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Shift::N => "N",
            Shift::K => "K",
            Shift::I => "I",
            Shift::A => "A",
            Shift::O => "O",
            Shift::H => "H",
            Shift::Y => "Y",
            Shift::D => "D",
            Shift::U => "U",
        };
        write!(f, "{}", s)
    }
}

pub type Schedule = Vec<Vec<Shift>>;

pub type Score = f32;

#[derive(PartialEq, Clone)]
pub enum ShiftState {
    Absolute,
    Random,
}

pub type ScheduleState = Vec<Vec<ShiftState>>;

#[derive(Clone)]
pub struct Staff {
    pub name: String,
    pub attributes: Vec<i32>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum DayState {
    Weekday,
    Holiday,
    Bath,
    Bath2,
    Measure,
}

impl fmt::Display for DayState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            DayState::Weekday => "W",
            DayState::Holiday => "H",
            DayState::Bath => "B",
            DayState::Bath2 => "2",
            DayState::Measure => "M",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for DayState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "W" => Ok(DayState::Weekday),
            "H" => Ok(DayState::Holiday),
            "B" => Ok(DayState::Bath),
            "2" => Ok(DayState::Bath2),
            "M" => Ok(DayState::Measure),
            _ => Err(format!("Failed to parse DayState: {}", s)),
        }
    }
}

pub type Days = Vec<DayState>;

pub type NG = (usize, usize);

pub type NGList = Vec<NG>;

pub type StaffAttributeName = String;

pub type DayAttributeName = String;

#[derive(Debug, PartialEq, Clone)]
pub enum ScoreProp {
    PatternGeneral((CondWrapper, Vec<Vec<Shift>>, Score)),
    PatternFixed((CondWrapper, Vec<Shift>, Score)),
    PatternGeneralAny((CondWrapper, Vec<Vec<Shift>>, Score)),
    PatternFixedAny((CondWrapper, Vec<Shift>, Score)),
    Streak((CondWrapper, Vec<Shift>, i32, Score)),
    ShiftsBalance((CondWrapper, Shift, Shift, Score)),
    ShiftHalfBalance((CondWrapper, Shift, Score)),
    ShiftDirPriority((CondWrapper, Shift, Score)),
    DayCountRegardStaffAttribute((CondWrapper, Shift, StaffAttributeName, Score)),
    StaffCountRegardDayAttribute((CondWrapper, Shift, DayAttributeName, Score)),
    StaffCount((CondWrapper, Shift, i32, Score)),
    StaffCountWithPremise((CondWrapper, Shift, i32, CondWrapper, Shift, i32, Score)),
    NGPair((CondWrapper, Shift, Score)),
    NoSamePair((CondWrapper, i32, Shift, Score)),
}

impl fmt::Display for ScoreProp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            ScoreProp::PatternGeneral(p) => format!("PatternGeneral {:?}", p),
            ScoreProp::PatternFixed(p) => format!("PatternFixed {:?}", p),
            ScoreProp::PatternGeneralAny(p) => format!("PatternGeneralAny {:?}", p),
            ScoreProp::PatternFixedAny(p) => format!("PatternFixedAny {:?}", p),
            ScoreProp::Streak(p) => format!("Streak {:?}", p),
            ScoreProp::ShiftsBalance(p) => format!("ShiftsBalance {:?}", p),
            ScoreProp::ShiftHalfBalance(p) => format!("ShiftHalfBalance {:?}", p),
            ScoreProp::ShiftDirPriority(p) => format!("ShiftDirPriority {:?}", p),
            ScoreProp::DayCountRegardStaffAttribute(p) => {
                format!("DayCountRegardStaffAttribute {:?}", p)
            }
            ScoreProp::StaffCountRegardDayAttribute(p) => {
                format!("StaffCountRegardDayAttribute {:?}", p)
            }
            ScoreProp::StaffCount(p) => format!("StaffCount {:?}", p),
            ScoreProp::StaffCountWithPremise(p) => format!("StaffCountWithPremise {:?}", p),
            ScoreProp::NGPair(p) => format!("NGPair {:?}", p),
            ScoreProp::NoSamePair(p) => format!("NoSamePair {:?}", p),
        };
        write!(f, "{}", s)
    }
}

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
    pub fn eval(&self, r: usize, c: usize, sp: &ScheduleProp) -> bool {
        match self {
            Cond::Every => true,
            Cond::Or((cond1, cond2)) => cond1.eval(r, c, sp) || cond2.eval(r, c, sp),
            Cond::And((cond1, cond2)) => cond1.eval(r, c, sp) && cond2.eval(r, c, sp),
            Cond::Not(cond) => !cond.eval(r, c, sp),
            Cond::DayExceptBuffer => sp.buffer <= c,
            Cond::DayInRange((day_start, day_end)) => *day_start <= c && c <= *day_end, // indexおかしいかも
            Cond::ParticularDayState(ds) => sp.days[c] == *ds, // indexおかしいかも
            Cond::BeforeDayState(ds) => {
                if c == 0 {
                    false
                } else {
                    sp.days[c - 1] == *ds
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
    cond: Cond,
    memo: Vec<Vec<Option<bool>>>,
}

impl CondWrapper {
    pub fn new(cond: Cond) -> Self {
        CondWrapper {
            cond,
            memo: <Vec<Vec<Option<bool>>>>::new(),
        }
    }

    /// SchedulePropが焼きなましの過程で変化しない制限の上で
    pub fn eval(&mut self, r: usize, c: usize, sp: &ScheduleProp) -> bool {
        if self.memo.is_empty() {
            self.memo = vec![vec![None; sp.day_count]; sp.staff_count];
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

#[derive(Clone)]
pub struct StaffAttributeNameIndexMap {
    pub names: Vec<StaffAttributeName>,
    pub name_to_index: HashMap<StaffAttributeName, usize>,
}
