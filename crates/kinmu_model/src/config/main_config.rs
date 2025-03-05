use super::ScheduleConfig;

/// 最大単位のConfig
#[derive(Clone, Debug, Default)]
pub struct MainConfig<SP, S, SS, DS> {
    pub schedule_configs: Vec<ScheduleConfig<SP, S, SS, DS>>,
    pub thread_count: Option<u32>,
}
