//! main_configを読み込むモジュール

use crate::kinmu_lib::types::MainConfig;

use super::super::super::reader;

pub fn load_main_config(path: &str) -> Result<MainConfig, String> {
    let config = reader::read_main_config(path)?;
    Ok(MainConfig {
        schedule_config_paths: config.schedule_config_paths,
        thread_count: config.thread_count,
    })
}
