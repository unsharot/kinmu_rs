//! 生成に関わる型の宣言

mod cond;
mod schedule;
mod score;

pub use self::cond::*;
pub use self::schedule::*;
pub use self::score::*;

pub type FilePath = String;

pub type ScheduleConfig = kinmu_model::ScheduleConfig<ScoreProp, Shift, ShiftState, DayState>;
