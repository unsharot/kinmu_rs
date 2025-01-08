//! 出力する型のdisplay実装

use crate::kinmu_lib::types::{DayState, ScoreProp, Shift};

use std::fmt;

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
