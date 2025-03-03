use super::config::ScheduleConfig;
use super::Schedule;

use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Answer<SP, S, SS, DS> {
    pub models: Vec<Schedule<S>>,
    pub schedule_config: ScheduleConfig<SP, S, SS, DS>,
    pub total_time: Duration,
}
