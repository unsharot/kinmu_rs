//! main_configを変換する関数を提供するモジュール

use super::super::reader::types::RawMainConfig;
use kinmu_model::MainConfig;

/// RawMainConfigをMainConfigに変換する
/// schedule_configsフィールドは空なので、あとから設定しなおす
pub fn convert_main_config<SP, S, SS, DS>(
    config: RawMainConfig,
) -> anyhow::Result<MainConfig<SP, S, SS, DS>> {
    Ok(MainConfig {
        schedule_configs: Default::default(),
        thread_count: config.thread_count,
    })
}
