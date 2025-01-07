//! main_configを読み込むモジュール

use crate::{io::input::reader::types::RawMainConfig, kinmu_lib::types::MainConfig};

pub fn convert_main_config(config: RawMainConfig) -> Result<MainConfig, String> {
    Ok(MainConfig {
        schedule_config_paths: config.schedule_config_paths,
        thread_count: config.thread_count,
    })
}
