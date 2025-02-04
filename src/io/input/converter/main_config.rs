//! main_configを読み込むモジュール

use super::super::reader::types::RawMainConfig;
use crate::kinmu_lib::types::MainConfig;

pub fn convert_main_config(config: RawMainConfig) -> anyhow::Result<MainConfig> {
    Ok(MainConfig {
        schedule_configs: Default::default(),
        thread_count: config.thread_count,
    })
}
