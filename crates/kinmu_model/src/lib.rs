//! Config, Answer, ScorePropTrait, Staff, Schedule, ScheduleStateを定義する
//! 実際のScoreProp, Shift, ShiftState, DayStateには依存せず、パラメータとして残す

mod answer;
mod config;
mod schedule;
mod score;
mod staff;

pub use self::answer::*;
pub use self::config::*;
pub use self::schedule::*;
pub use self::score::*;
pub use self::staff::*;
