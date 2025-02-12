//! main_configを定義

use super::ScheduleConfig;

#[derive(Clone, Debug, Default)]
pub struct MainConfig {
    pub schedule_configs: Vec<ScheduleConfig>,
    pub thread_count: Option<u32>,
}
