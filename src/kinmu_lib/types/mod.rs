//! 生成に関わる型の宣言

mod cond;
mod config;
mod schedule;
mod score;
mod staff;

pub use self::cond::*;
pub use self::config::*;
pub use self::schedule::*;
pub use self::score::*;
pub use self::staff::*;

use std::time::Duration;

pub type FilePath = String;

#[derive(Debug, Clone)]
pub struct Answer {
    pub models: Vec<Schedule>,
    pub schedule_prop: ScheduleProp,
    pub total_time: Duration,
}
