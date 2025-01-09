//! 勤務表に関わる型の宣言

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

pub type Schedule = Vec<Vec<Shift>>;

#[derive(PartialEq, Clone, Debug)]
pub enum ShiftState {
    Absolute,
    Random,
}

pub type ScheduleState = Vec<Vec<ShiftState>>;

#[derive(Debug, PartialEq, Clone)]
pub enum DayState {
    Weekday,
    Holiday,
    Bath,
    Bath2,
    Measure,
}
