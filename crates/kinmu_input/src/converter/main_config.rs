//! main_configを読み込むモジュール

use super::super::reader::types::RawMainConfig;
use ::kinmu_model::MainConfig;

pub fn convert_main_config<SP, S, SS, DS>(
    config: RawMainConfig,
) -> anyhow::Result<MainConfig<SP, S, SS, DS>> {
    Ok(MainConfig {
        schedule_configs: Default::default(),
        thread_count: config.thread_count,
    })
}
