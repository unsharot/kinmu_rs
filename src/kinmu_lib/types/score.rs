//! スコアに関わる型の宣言

use super::cond::CondWrapper;
use super::schedule::Shift;
use super::staff::StaffAttributeName;

pub type Score = f32;

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
