//! 勤務表に関わる型の宣言

use kinmu_input::{FromConfig, MapState};

use std::fmt;

/// シフトの型
#[derive(Debug, Clone, PartialEq, Copy, Default)]
pub enum Shift {
    #[default]
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

impl fmt::Display for Shift {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Shift::N => write!(f, "N"),
            Shift::K => write!(f, "K"),
            Shift::I => write!(f, "I"),
            Shift::A => write!(f, "A"),
            Shift::O => write!(f, "O"),
            Shift::H => write!(f, "H"),
            Shift::Y => write!(f, "Y"),
            Shift::D => write!(f, "D"),
            Shift::U => write!(f, "U"),
        }
    }
}

impl FromConfig for Shift {
    fn from_config(s: &str) -> anyhow::Result<Self> {
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
            _ => Err(anyhow::anyhow!("Failed to parse Shift: {}", s)),
        }
    }
}

/// Shiftを用いる具体的なSchedule
pub type Schedule = kinmu_model::Schedule<Shift>;

/// シフトが固定かどうかを管理する型
#[derive(PartialEq, Clone, Debug, Default)]
pub enum ShiftState {
    #[default]
    Absolute,
    Random,
}

/// ShiftStateを用いる具体的なScheduleState
pub type ScheduleState = kinmu_model::ScheduleState<ShiftState>;

impl MapState<ShiftState> for Shift {
    const BUFFER_CASE: ShiftState = ShiftState::Absolute;
    fn to_state(&self) -> ShiftState {
        match self {
            Shift::U => ShiftState::Random,
            _ => ShiftState::Absolute,
        }
    }
}

/// 曜日を管理する型
#[derive(Debug, PartialEq, Clone, Default)]
pub enum DayState {
    #[default]
    Weekday,
    Holiday,
    Bath,
    Bath2,
    Measure,
}

impl fmt::Display for DayState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DayState::Weekday => write!(f, "W"),
            DayState::Holiday => write!(f, "H"),
            DayState::Bath => write!(f, "B"),
            DayState::Bath2 => write!(f, "2"),
            DayState::Measure => write!(f, "M"),
        }
    }
}

impl FromConfig for DayState {
    fn from_config(s: &str) -> anyhow::Result<Self> {
        match s {
            "W" => Ok(DayState::Weekday),
            "H" => Ok(DayState::Holiday),
            "B" => Ok(DayState::Bath),
            "2" => Ok(DayState::Bath2),
            "M" => Ok(DayState::Measure),
            _ => Err(anyhow::anyhow!("Failed to parse DayState: {}", s)),
        }
    }
}
