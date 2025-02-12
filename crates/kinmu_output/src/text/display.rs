//! 出力する型のdisplay実装

use ::kinmu_lib::types::{DayState, ScoreProp, Shift};

pub trait Display {
    fn show(&self) -> String;
}

impl Display for Shift {
    fn show(&self) -> String {
        match self {
            Shift::N => String::from("N"),
            Shift::K => String::from("K"),
            Shift::I => String::from("I"),
            Shift::A => String::from("A"),
            Shift::O => String::from("O"),
            Shift::H => String::from("H"),
            Shift::Y => String::from("Y"),
            Shift::D => String::from("D"),
            Shift::U => String::from("U"),
        }
    }
}

impl Display for DayState {
    fn show(&self) -> String {
        match self {
            DayState::Weekday => String::from("W"),
            DayState::Holiday => String::from("H"),
            DayState::Bath => String::from("B"),
            DayState::Bath2 => String::from("2"),
            DayState::Measure => String::from("M"),
        }
    }
}

impl Display for ScoreProp {
    fn show(&self) -> String {
        match self {
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
            ScoreProp::StaffCountAtLeast(p) => format!("StaffCountAtLeast {:?}", p),
            ScoreProp::StaffCountWithPremise(p) => format!("StaffCountWithPremise {:?}", p),
            ScoreProp::NGPair(p) => format!("NGPair {:?}", p),
            ScoreProp::NoSamePair(p) => format!("NoSamePair {:?}", p),
        }
    }
}
